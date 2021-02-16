// sudoku
#![allow(dead_code)]

use rand::{Rng, seq::SliceRandom};
use scoped_pool::{Pool};
use num_cpus;

use std::{char, {time::{Instant, Duration}}, sync::{Arc, Mutex, MutexGuard}};
use array2d::*;
use druid::Data;

pub enum Level { VeryEasy, Easy, Medium, Difficult, Master }

// Coord & Board
type Coord = (usize, usize);
type Board = Array2D<u8>;

// Arc<Mutex<T>>
#[derive(Clone, Debug, Data)]
struct ArcMutex<T> (Arc<Mutex<T>>);

impl<T> ArcMutex<T> {
	pub fn new(v : T) -> Self { ArcMutex( Arc::new(Mutex::new(v)) ) } 
	pub fn get(&self) ->  T where T: Copy { *self.0.lock().unwrap() }
	pub fn get_ref(&self) -> MutexGuard<T> { self.0.lock().unwrap() }
	pub fn clone(&self) -> Self { ArcMutex( Arc::clone(&self.0)) }
	pub fn set(&mut self, v : T) { *self.0.lock().unwrap() = v }
}


// sudoku
#[derive(Debug, Clone, Data)]
pub struct Sudoku {
				pub n	  				: usize,
				sz_box				: usize,
#[data(ignore)]	pub board 				: Board,
#[data(ignore)]	board_prt			: Board,
#[data(ignore)]	lookup_upper_cells 	: Array2D<Vec<Coord>>,
#[data(ignore)]	lookup_all		   	: Array2D<Vec<Coord>>,
#[data(ignore)]	one_2_n_vals		: Vec<usize>,
				abort				: ArcMutex<bool>,
#[data(ignore)]	solutions			: Vec<Board>,
				max_solutions		: usize,
				count_evals			: ArcMutex<usize>,
}

// misc utils
fn rand(n : u32) -> u32{
	let mut rng = rand::thread_rng();
	let r = rng.gen::<u32>() % n;
	r
}

pub fn num_fmt(n : usize) -> String {
	if n >= 1_000_000_000  {format!("{}G",n/1_000_000_000)}
	else if n >= 1_000_000 {format!("{}M",n/1_000_000)}
	else if n >= 1_000 	   {format!("{}K",n/1_000)}
	else {format!("{}",n)}		
}


impl Default for Sudoku { fn default() -> Self { Self::new(0) } }

impl Sudoku {
	pub fn new(sz_box : usize) -> Self {
		let n = sz_box * sz_box;

		let mut sdk = Self { 
			n					: n, 
			sz_box				: sz_box,
			board				: Array2D::filled_with(0_u8, n, n), 
			board_prt			: Array2D::filled_with(0_u8, n, n),
			lookup_upper_cells 	: Array2D::filled_with(vec![], n, n), 
			lookup_all			: Array2D::filled_with(vec![], n, n), 
			one_2_n_vals 		: (1..n+1).collect(),
			abort		 		: ArcMutex::<bool>::new(false),
			solutions			: vec![],
			max_solutions		: 1,
			count_evals			: ArcMutex::<usize>::new(0),
		};
		sdk.gen_lookup();
		sdk
	}

	pub fn get_abort(&self) -> bool { self.abort.get() }
	pub fn set_abort(&mut self, val : bool) { self.abort.set(val) }

	pub fn get_count_evals(&self) -> usize { self.count_evals.get() }
	fn add_count_evals(&mut self, n : usize)  {  *self.count_evals.get_ref() += n	}

	pub fn get(&self, row : usize, col :usize) -> (u8, char) {
		let brc = self.board[(row, col)];
		(brc, char::from_digit(brc as u32, (self.n+1) as u32).unwrap())
	}

	pub fn get_n(&self) -> usize { self.n }
	pub fn get_szbox(&self) -> usize { self.sz_box }
	pub fn get_size(&self) -> (usize, usize) { (self.sz_box, self.n) }
	
	pub fn is_running(&self) -> bool { ! self.get_abort() }

	fn gen_lookup(&mut self) {  // generate UPPER lookUp vector per cell including box
		let n = self.n;

		for row in 0..self.n {
			for col in 0..self.n {
				let lk  = &mut self.lookup_upper_cells[(row,col)];
				let lka = &mut self.lookup_all[(row,col)];
			
				let curr_coord = (row, col);
				
				for r in 0..row { lk.push( (r, col) ) }  // upper rows
				for c in 0..col { lk.push( (row, c) ) }  // & cols ONLY not lower ones

				// all coords except current (row,col)
				for r in 0..n {  if r != row { lka.push( (r, col) ) }	}
				for c in 0..n {  if c != col { lka.push( (row, c) ) }	}
				
				let rb = self.sz_box * (row / self.sz_box);
				let cb = self.sz_box * (col / self.sz_box);

				for r in 0..self.sz_box {
					for c in 0..self.sz_box {
						let coord = ( r + rb, c + cb );

						if coord != curr_coord && lk.iter().find(|&&c| c==coord).is_none() {  // != current & unique
							lk .push(coord);
							lka.push(coord)
						}
					}
				}
		}
	  }
	}

	fn gen_moves(&self, row : usize, col : usize) -> ( Vec<usize>, usize) {
		let mut vals = self.one_2_n_vals.clone();  // 1..N
	
		// from row,col to begining doesn't, look lower values
	
		let mut n_removed = 0;

		for l in &self.lookup_upper_cells[(row,col)] {
		  let v = self.board[(l.0, l.1)];
		  if v != 0 {                // not empty
			if vals[(v - 1) as usize] != 0 {  // not removed yet
				vals[(v - 1) as usize] = 0;
				n_removed+=1;
				if n_removed == self.n { break }  // all removed -> end
			}
		  }
		}
		if n_removed != self.n { // create vec with non zero items
			let mut v = vec![];
			for vv in vals { if vv!=0 { v.push(vv) } }
			let len_v = v.len();
			(v, len_v)
		}
		else { (vec![], 0) }
	}

	pub fn scan(&mut self) {
		let n = self.n;
		self.board = Array2D::filled_with(0_u8, n, n);
		self.solutions.clear();
		self.set_abort(false);
		self.count_evals = ArcMutex::<usize>::new(0);

		// self._scan(0, 0);
		self.solve().unwrap();

		self.set_abort(true);
	}

	fn _move(&mut self, row : usize, col : usize, val : usize) { self.board[(row,col)] = val as u8; }

	fn _scan(&mut self, row : usize, col :usize) {
		if ! self.get_abort() {

			if self.board[(row,col)] == 0 {  // skip non empty cells (solve process)

				let (moves, n_moves) = &self.gen_moves(row, col);

				for mv in moves {

					self._move(row, col, *mv);
		
					if col < self.n-1   { self._scan(row, col + 1) }
					else {
						if row < self.n-1   { self._scan(row + 1, 0) }
						else			    { self.save_solution()   }
					}
		
					self._move(row, col, 0);  // b(r,c)=0;		
				}
				self.add_count_evals(*n_moves);

			} else {  // next cell
			  if col < self.n-1 { self._scan(row, col + 1) }
			  else {
				if row < self.n-1 { self._scan(row + 1, 0) }
				else			  { self.save_solution()   }
			  }
			}
		} 
	}

	fn is_valid(&self) -> bool {
		
		let mut valid = true;
	
		for rc in 0..2 {  // rows & cols
			if !valid { break }

			for r in 0..self.n {

				let mut vb = vec![0_usize; self.n];

				for c in 0..self.n {
					let b = if rc == 0 { self.board[(r,c)] } else { self.board[(c,r)] } as usize;
					if b!=0 { vb[(b - 1)] = b}
					else {
						valid = false;
						break
					}
				}
				if vb != self.one_2_n_vals {
					valid = false;
					break
				}
			}
		}
	
		for row in (0..self.n).step_by(self.sz_box) {
			if !valid { break }

			for col in (0..self.n).step_by(self.sz_box) {

				let mut vb=vec![0_usize; self.n];
				for r in 0..self.sz_box { // box
					for c in 0..self.sz_box {
						let b = self.board[(r + row,c + col)] as usize;
						if b!=0 { vb[b - 1] = b }
						else {
							valid = false;
							break
						}
					}
				}
				if vb != self.one_2_n_vals {
					valid = false;
					break
				}
			}
		}
	
		valid		  
	}


	fn rand_no_solvable(&mut self, n_fill : u32 ) {  //  generate a probably unsolvable random board
		let (n, nn) = ( self.n, self.n * self.n );
		self.board = Array2D::filled_with(0_u8, self.n, self.n);

	  	for r in 0..n {
			for c in 0..n {
				if rand(nn as u32) > n_fill {
					let (moves, n_moves) = self.gen_moves(r, c);
					if n_moves!=0 { self.board[(r,c)] = moves[rand(n_moves as u32) as usize] as u8}
				}
			}
		}
	}

	fn n_box(&self, i : usize) -> usize { i / self.sz_box }

	fn swap_col(&mut self, c0 : usize, c1 : usize) {
		for c in 0..self.board.row_len() {
			let tmp = self.board[(c,c0)];
			self.board[(c,c0)] = self.board[(c,c1)];
			self.board[(c,c1)] = tmp;
		}		
	}
	fn swap_row(&mut self, r0 : usize, r1 : usize) {
		for r in 0..self.board.column_len() {
			let tmp = self.board[(r0,r)];
			self.board[(r0,r)] = self.board[(r1,r)];
			self.board[(r1,r)] = tmp;
		}		
	}
	

	pub fn gen_problem(&mut self, level : Level) {// generate a random solvable problem
		let n = self.n;
		let szb = self.sz_box;

		let rnd = || { rand(n as u32) as usize };

		// generate first sdk
		self.max_solutions = 1;	  
		self.scan();
	
		if ! self.solutions.is_empty() {

			self.board = self.solutions[0].clone(); // shuffle

			let mut s0 : Vec<usize> = (0..szb).collect();
			let mut s1 = s0.clone();

			for b in 0..self.n {	
				let (r, c) = ( b / szb, b % szb );
				
				for _ in 0..n*n {
					s0.shuffle(&mut rand::thread_rng());
					s1.shuffle(&mut rand::thread_rng());
					
					for (x0, x1) in s0.iter().zip(s1.iter()) {
						let (rs, cs) = (r * szb, c * szb);
						self.swap_col(szb-1 + cs, cs);
						self.swap_row(szb-1 + rs, rs);
						self.swap_col(*x0 + cs, x1 + cs);
						self.swap_row(*x0 + rs, x1 + rs)
					}	
				}
			}
			
			assert_eq!(self.is_valid(), true);
			
			// the higher the level the more empty cells 0:n/3, 1:n/2, 2:2*n/3
			let nn =n*n;
			for _ in 0..(nn / (2+(Level::Master as usize)-(level as usize))) { self.board[(rnd(),rnd())] = 0; }
		}
		self.set_abort(true);
	}

	fn print(&self) { print!("{}", self.to_string()) }	
	
	pub fn to_string(&self) -> String {

		let mut s_out = String::new(); 

		let line = | ch : char, n : usize | {	format!("{}\n", String::from_utf8(vec![ch as u8; n]).unwrap()) };
		let n1 = self.n*2+1;
	

		for r in 0..self.n {
			if r == 0 { s_out += &line('_', n1)  }
			else {
				if r % self.sz_box == 0 { s_out += &line('-', n1) }
			}
			for c in 0..self.n { 
				let (b, ch) = self.get(r,c);
				let ch = if b==0 { '_' } else { ch };
				s_out += &format!("{}{}", if c % self.sz_box == 0 {"|"} else {" "}, ch)
			}
			s_out+="|\n";
		}		

		s_out + &line('-', n1)
	}	

	fn save_solution(&mut self) {
		if self.is_valid() {
			self.solutions.push(self.board.clone());
			if self.solutions.len() >= self.max_solutions { self.set_abort(true) }
		}
	}

	fn init(&mut self) {
		self.set_abort(false);
		self.solutions.clear();
		self.max_solutions=1;
		self.count_evals=ArcMutex::<usize>::new(0);
	}
	
	pub fn zero_board(&mut self) {	self.board = Array2D::filled_with(0_u8, self.n, self.n) }

	fn find_first_empty(&self) -> Result<Coord, &'static str> {
		let mut found = false;
		let mut coord = (0,0);
		
		'_exit : for r in 0..self.n { 
			for c in 0..self.n {
				if self.board[(r,c)]==0 {
					found=true; coord=(r, c); break '_exit
				}
			}
		}
		if found { Ok(coord) } else { Err("board is solved") }
	}

	pub fn is_solved(&self) -> bool {
		if let Ok(_) = self.find_first_empty() { false } else { true }
	}
	

	pub fn solve__org(&mut self) -> Result<Duration, &'static str> { // using  Vec<Sudoku>

		let t = Instant::now();

		if let Ok(coord) = self.find_first_empty() {

			let (mvs, n_moves) = self.gen_moves(coord.0, coord.1);
			
			if !mvs.is_empty() {
				self.init(); // vector of self copies moving each with 'mvs[i]'
				let mut sudokus = vec![self.clone(); n_moves];
				for i in 0..sudokus.len() { sudokus[i]._move(coord.0, coord.1, mvs[i]) }

				let pool = Pool::new(num_cpus::get().min(n_moves));
				pool.scoped(|scope| { // _scan all sdk's from coord
					for sdk in &mut sudokus {
						scope.execute(move|| { sdk._scan(coord.0, coord.1) })
					}
				});	
				pool.shutdown();
				
				if let Some(sdk) = sudokus.iter().find( |s| !s.solutions.is_empty() ) { // found any?
					*self = sdk.clone(); // set self & first solution
					self.board = self.solutions[0].clone(); 
					self.set_abort(true)
				} else {
					return Err("unsolvable board")
				}			
			} else {
				return Err("unsolvable board")
			}
		} else {
			return Err("board is already solved")
		}
		Ok( Instant::now() - t )
	}

	pub fn solve(&mut self) -> Result<Duration, &'static str> {

		let t = Instant::now();

		if let Ok(coord) = self.find_first_empty() {

			let (mvs, n_moves) = self.gen_moves(coord.0, coord.1);
			
			if !mvs.is_empty() {
				self.init(); // solutions mutex
				let solutions : ArcMutex<Vec<Board>> = ArcMutex::new(self.solutions.clone());

				let pool = Pool::new(num_cpus::get().min(n_moves));

				pool.scoped(|scope| { // _scan all sdk's from coord

					for i in 0..n_moves {

						let sdk = ArcMutex::new(self.clone());	// working copy of sudoku
						let mvs = mvs.clone();
						let solutions = solutions.clone();

						scope.execute(move|| { 
							sdk.get_ref()._move(coord.0, coord.1, mvs[i]);
							sdk.get_ref()._scan(coord.0, coord.1);

							if ! sdk.get_ref().solutions.is_empty() {
								solutions.get_ref().push(sdk.get_ref().solutions[0].clone())
							}
						})
					}
				});	
				pool.shutdown();
				
				if !solutions.get_ref().is_empty() { // found any?
					self.solutions = solutions.get_ref().clone(); // set self & first solution
					self.board = self.solutions[0].clone(); 
					self.set_abort(true)
				} else {
					return Err("unsolvable board")
				}			
			} else {
				return Err("unsolvable board")
			}
		} else {
			return Err("board is already solved")
		}
		Ok( Instant::now() - t )
	}

}
