// queens module
// 46 : [4, 29, 0, 2, 5, 1, 6, 9, 3, 41, 32, 36, 15, 7, 19, 8, 22, 24, 10, 27, 11, 30, 33, 38, 40, 37, 43, 45, 42, 39, 14, 44, 13, 21, 23, 17, 12, 16, 35, 26, 20, 18, 25, 28, 34, 31]
// 55 : [39, 13, 15, 0, 33, 8, 1, 26, 44, 51, 24, 18, 14, 48, 5, 43, 2, 49, 36, 23, 42, 30, 9, 7, 54, 45, 20, 38, 53, 21, 47, 37, 6, 11, 19, 40, 4, 10, 31, 28, 3, 16, 41, 12, 27, 35, 17, 29, 46, 25, 22, 52, 32, 34, 50]
// 55 : [0, 29, 1, 4, 2, 30, 27, 10, 5, 19, 16, 34, 7, 21, 25, 52, 9, 37, 51, 43, 54, 39, 53, 17, 3, 41, 35, 11, 8, 12, 18, 6, 14, 20, 47, 49, 40, 28, 50, 44, 42, 48, 31, 13, 15, 23, 32, 24, 38, 45, 22, 36, 33, 26, 46]

#![allow(unused_imports)]
#![allow(dead_code)]


use druid::{ kurbo::{Rect, Ellipse}, widget::prelude::*, {Color, Point}};
use druid::{
    AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, ExtEventSink, Lens, LocalizedString,
    Selector, Target, Widget, WidgetExt, WindowDesc,
};
use std::{{thread, time}, sync::{Arc, Mutex, mpsc::{channel, Sender, Receiver}}};

use num_cpus;

//extern crate scoped_pool;
use scoped_pool::{Pool, Scope};

use std::ops::{Index, IndexMut};


pub fn rand<T>(n : i32) -> i32 {
	use rand::Rng;
	let mut rng = rand::thread_rng();
		
	rng.gen::<i32>().abs() % n
}

pub fn rand_usize(n : usize) -> usize {
	use rand::Rng;
	let mut rng = rand::thread_rng();
		
	if n!=0 { rng.gen::<usize>() % n as usize } else { 0 }
}


// Board
#[derive(Clone, Debug, Data, Default, PartialEq)]
pub struct Board {
	n : i32,
	#[data(ignore)]	board : Vec<i32>,
	#[data(ignore)]	ld: Vec<bool>,
	#[data(ignore)]	rd: Vec<bool>,
	#[data(ignore)]	cl: Vec<bool>
}


impl Board {
	fn new(n : i32) -> Self{ 
		let n2 = n as usize * 2;
		Self {n, board:vec![0; n as usize], ld:vec![false; n2], rd:vec![false; n2], cl:vec![false; n2]} 
	}
	
	fn set(&mut self, col : i32, val: i32) {  // set
		self.board[col as usize] = val;

		self.ld[(val - col + self.n - 1) as usize] = true;
		self.rd[(val + col) as usize] = true;
		self.cl[val as usize] = true;
	}
	fn get(&self, col :i32) -> i32 {
		self.board[col as usize]
	}
	fn reset(&mut self, col : i32, val : i32) {
		self.board[col as usize] = 0;

		self.ld[(val - col + self.n - 1) as usize] = false;
		self.rd[(val + col) as usize] = false;
		self.cl[val as usize] = false;
	}
	fn is_valid_position(&self, col : i32, i : i32) -> bool {
		!self.ld[(i - col + self.n - 1) as usize] 	&& 
		!self.rd[(i + col) as usize] 				&& 
		!self.cl[i as usize]
	}
	fn is_not_valid_position(&self, col : i32, i : i32) -> bool {
		self.ld[(i - col + self.n - 1) as usize] ||
		self.rd[(i + col) as usize] 			 ||
		self.cl[i as usize]
	}

	pub fn print(&self) { self.board.iter().for_each(|i| print!("{} ", i))	}

	pub fn to_string(&self) -> String {	self.board.iter().map(|i| format!("{} ", i)).collect()	}

	pub fn draw(&self) {
		self.board.iter().for_each(
			|i| println!("{}", (0..self.n).map(|ix| if ix==*i {" O "} else {" . "}).collect::<String>() )
		)
	}
	pub fn draw_string(&self) -> String {
		self.board.iter().map(	|i|  (0..self.n).map(|ix| if ix==*i {" O "} else {" . "} ).collect::<String>() + "\n" ).collect()
	}
}

#[derive(Debug, Clone, Data, Default)]
pub struct Queen {
	pub n 				: i32,
	#[data(ignore)]	pub board 			: Board,
	pub m_abort 		: bool,
	pub count_evals 	: Arc<Mutex<u128>>,

	#[data(ignore)]	pub solutions 		: Vec<Board>,
	pub current_sol 	: usize,
	pub count_solutions : Arc<Mutex<usize>>,
	max_solutions 		: usize, // if 0 -> all
}

impl Index<usize> for Queen { // index a=v[i]
	type Output = i32;
    fn index(&self, idx: usize) -> &i32 {	&self.board.board[idx]    }
}

impl Queen {
	pub fn new(n : i32) -> Self {
		Self { n, board : Board::new(n), 
			m_abort : false, count_evals : Arc::new(Mutex::new(0)), 
			solutions : vec![], current_sol : 0 , count_solutions : Arc::new(Mutex::new(0)), max_solutions : 0 }
	}

	pub fn clear(&mut self) {
		self.clear_counters();
		self.board=Board::new(self.n);		
	}
	pub fn clear_counters(&mut self) {
		self.m_abort=false;
		self.count_evals = Arc::new(Mutex::new(0));
		self.solutions.clear();
		self.current_sol = 0;
		self.count_solutions = Arc::new(Mutex::new(0));
	}

	pub fn get_evals(&self) -> u128   {	*self.count_evals.lock().unwrap()	}
	fn add_evals(&self, n : u128) {	*self.count_evals.lock().unwrap() += n	}
	fn add_eval_n(&self) { 	*self.count_evals.lock().unwrap() += self.n as u128 }

	pub fn set_max_solutions(&mut self, ms : usize) { self.max_solutions = ms }

	pub fn is_valid(&self) -> bool {
		let mut ok = true;
		
		for i in 0..self.n-1 {
			for j in i+1..self.n  {
				if self[i as usize] == self[j as usize] 		  { ok = false; 	break }  // horizontal -> ci=cj
				if i - self[i as usize] == j - self[j as usize]   { ok = false;		break }  // vertical  / ri-ci = rj-cj
				if (self[i as usize] - self[j as usize]).abs() == (i - j).abs() { ok = false;		break } // vertical \ |ci-cj| = |i-j|
			}
			if !ok { break }
		}		
		ok
	}

	pub fn save_solution(&mut self) {
		if self.is_valid() {
			if self.max_solutions != 0 {
				if let Ok(mut cs) = self.count_solutions.lock() { 
					if *cs >= self.max_solutions { self.m_abort  = true }
					else { 
						*cs += 1;
						self.solutions.push(self.board.clone())
					}
				}
			} else { // 0 -> all
				if let Ok(mut cs) = self.count_solutions.lock() { 
					*cs += 1;
					self.solutions.push(self.board.clone())
				}
			}
		} else {
			self.m_abort = true;
		}
	}

	pub fn sort_solutions_unique(&mut self) {
		self.solutions.sort_by_key(|k| k.board.clone());
		self.solutions.dedup(); // remove dups
	}

	pub fn prev_solution(&mut self) {
		if self.has_solutions() {
			if self.current_sol == 0 { self.current_sol = self.solutions.len()-1 }
			else { self.current_sol -= 1 }			
			self.set_solution(self.current_sol);
		}
	}
	pub fn next_solution(&mut self) {
		if self.has_solutions() {
			self.current_sol += 1;
			if self.current_sol >= self.solutions.len() {	self.current_sol = 0	}
			self.set_solution(self.current_sol);
		}
	}

	pub fn get_solution(&self, ix : usize) -> Vec<i32> {
		self.solutions[ix].board.clone()
	}

	pub fn set_solution(&mut self, ix : usize) {
		if ix < self.solutions.len() {
			self.board = self.solutions[ix].clone();
		}
	}

	pub fn has_solutions(&self) -> bool { ! self.solutions.is_empty() }

	pub fn set_random_solution(&mut self) -> usize {
		let rs = rand_usize(self.solutions.len());
		self.set_solution(rs);
		rs
	}

	pub fn scan(&mut self, col : i32) {
		if !self.m_abort {
			if col >= self.n { self.save_solution(); }
			else {
				for i in 0..self.n {
					if self.board.is_valid_position(col, i) {
						self.board.set(col, i);
				
						self.scan(col + 1);  // recur to place rest
				
						self.board.reset(col, i);  // unmove
					}
				}
				self.add_eval_n();
			}
		}
	}

	pub fn scan_first(&mut self, col : i32) {
		if !self.m_abort {
			if col >= self.n { self.save_solution(); self.m_abort = true; }
			else {
				for i in 0..self.n {
					if self.board.is_valid_position(col, i) {
						self.board.set(col, i);
								
						self.scan_first(col + 1);  // recur to place rest
				
						self.board.reset(col, i);  // unmove
					}
				}
				self.add_eval_n();
			}
		}
	}
	

	pub fn find_solutions(&mut self) {
		self.clear();
		self.scan(0);
	}

	pub fn find_first_solution(&mut self) {
		self.clear();
		self.board.set(0, self.n/2); // start from mid board -> closest solution
		
		self.scan_first(1);
	}
		
	pub fn set(&mut self, other : Self) {
		self.board=other.board;
		self.solutions=other.solutions;
		self.count_evals=other.count_evals;
	}

	// multithread section
	fn get_nth(&self) -> usize { num_cpus::get() }
	
	
	pub fn find_first_solution_mt_fast(&mut self) {
		let nth = self.get_nth(); // num_cpus::get() as usize; 

		self.clear();		// generate a vec of queens from n/2, n/2+i+2

		let mut queens = vec![self.clone(); nth];

		let n2 = self.n/2;
		for i in 0..nth { // col0:i, col1:i+n/2+2
			queens[i].board.set(0, i as i32); 
			queens[i].board.set(1, (n2 + i as i32 + 1) % self.n) 
		}
		
		let pool = Pool::new(nth);
		pool.scoped(|scope| {
			for q in &mut queens {		
				scope.execute(move || q.scan_first(2) );	
			}
		});
		pool.shutdown();
		
		self.set( queens.iter().find(|q| !q.solutions.is_empty() ).unwrap().clone() ) // set THE solution found
	}


	// scan with external stop control on running : &Arc<Mutexbool>> 

	fn get_running(running : &Arc<Mutex<bool>>) -> bool { // get Arc<Mutex<bool>> value
		if let Ok(_running) = running.lock() { *_running } else { false } 
	}

	pub fn scan_first_mt(&mut self, col : i32, running : &Arc<Mutex<bool>> ) {
	
		self.m_abort = ! Self::get_running(running);
		
		if !self.m_abort { 
			if col >= self.n { // found THE solution -> save & stop
				self.save_solution(); 

				*running.lock().unwrap() = false; // lock in this scope
				self.m_abort = true;
			} else {
				for i in 0..self.n {
					
					if self.m_abort { break }

					if self.board.is_valid_position(col, i) {
						self.board.set(col, i);
								
						self.scan_first_mt(col + 1, running);  // recur to place rest
				
						self.board.reset(col, i);  // unmove
					}					
				}
				self.add_eval_n();
			}
		}
	}

	pub fn scan_all_mt(&mut self, col : i32, running : &Arc<Mutex<bool>> ) {

		self.m_abort = ! Self::get_running(running);

		if !self.m_abort {
			if col >= self.n { // found solution -> save 
				self.save_solution(); 
			
				if self.m_abort { *running.lock().unwrap() = false }
			} else {
				for i in 0..self.n {
					
					if self.m_abort { break }

					if self.board.is_valid_position(col, i) {
						self.board.set(col, i);
								
						self.scan_all_mt(col + 1, running);  // recur to place rest
				
						self.board.reset(col, i);  // unmove
					}					
				}
				self.add_eval_n();
			}
		}
	}

	// proof that a clone of Arc<Mutex> modifies value in a different thread
	// affecting all threads with this parameter

	
	pub fn test01(&self) {

		use std::sync::atomic::{AtomicUsize, Ordering};

		fn set(running : &Arc<Mutex<bool>>, val : bool) {
			if let Ok(mut lr) = running.lock() { *lr = val }  // when this is modified -> wait value is affected
		}
		fn wait(running : &Arc<Mutex<bool>>, sum : &Arc<Mutex<u32>>, asum : &Arc<AtomicUsize>) {			
			while if let Ok(mutex) = running.try_lock() { *mutex } else { false } {
				*sum.lock().unwrap() += 1;
				asum.fetch_add(1, Ordering::SeqCst);
			}
		}

		const N : usize = 80;
		let running = Arc::new(Mutex::new(true)); // common running mutex, first found quit all			
		let sum = Arc::new(Mutex::new(0_u32)); 
		let asum = Arc::new(AtomicUsize::new(0));

		println!("**** testing arc<mutex> with values: running = {}, sum = {}, asum = {}", 
			running.lock().unwrap(), sum.lock().unwrap(), asum.load(Ordering::SeqCst));

		let pool = Pool::new(N);
		pool.scoped(|scope| {		
		
			print!("waiting..."); // run N wating threads
			for i in 0..N {
				print!("{} ", i);
				
				let running = running.clone();				
				let sum = sum.clone();				
				let asum = asum.clone();				
							
				scope.execute(move || {
					wait(&running, &sum, &asum); // -> set(&rcl, false) to release
					print!("r{} ", i)
				});
			}

			// thread::sleep(time::Duration::from_millis(1000));

			println!("\nreleasing..., sum={}, asum={}", *sum.lock().unwrap(), asum.load(Ordering::SeqCst));
			
			let rcl = Arc::clone(&running);
			scope.execute(move || {	set(&rcl, false) });			
			
		});
		pool.shutdown();
		println!("\ntest completed !!!!!!!!!!")
	}

	pub fn ff_thread(mut self) -> Receiver<Queen>  { // find first solution in a non blocking thread sending result once found
		let (tx, rx) = channel();
		
		thread::spawn(move || {
			println!("searching solution...");
			self.find_first_solution_mt();	// find
			tx.send(self).unwrap();			// send			
		});
		
		rx
	}
	pub fn find_first_solution_mt(&mut self) {
		let nth = self.get_nth(); // num_cpus::get() as usize; 

		self.clear();		// generate a vec of queens from n/2, n/2+i+2

		let mut queens = vec![self.clone(); nth];

		let n2 = self.n/2;
		for i in 0..nth { // col0:i, col1:i+n/2+2
			queens[i].board.set(0, i as i32); 
			queens[i].board.set(1, (n2 + i as i32 + 1) % self.n) 
		}
		
		let pool = Pool::new(nth);
		pool.scoped(|scope| {
			let running = Arc::new(Mutex::new(true)); // common running mutex, first found quit all

			for q in &mut queens {		
				let running = Arc::clone(&running);
				scope.execute(move || q.scan_first_mt(2, &running) );	
			}
		});
		pool.shutdown();
		

		self.set( queens.iter().find(|q| !q.solutions.is_empty() ).unwrap().clone() ) // set THE solution found
	}

	pub fn find_first_solution_mt_control(&mut self, running : &Arc<Mutex<bool>>) {
		let nth = self.get_nth(); // num_cpus::get() as usize; 

		self.clear();		// generate a vec of queens from n/2, n/2+i+2

		let mut queens = vec![self.clone(); nth];


		for i in 0..nth { // col0:i, col1:i+n/2+2
			queens[i].board.set(0, i as i32); 
		}
		
		let pool = Pool::new(nth);

		pool.scoped(|scope| {
			for q in &mut queens {		
				let running = Arc::clone(&running);
				scope.execute(move || q.scan_first_mt(1, &running) );	
			}
		});

		pool.shutdown();

		if let Some(fq) = queens.iter().find(|q| !q.solutions.is_empty()) {
			self.set( fq.clone() ) // set THE solution found
		}
	}

	pub fn find_all_solutions_mt_control(&mut self, running : &Arc<Mutex<bool>>) {
		let nth = self.get_nth(); // num_cpus::get() as usize; 

		self.clear();

		let mut queens = vec![self.clone(); nth]; // 1 queen per thread
		for i in 0..nth { queens[i].board.set(0, i as i32) } // q[i][0]=i
		
		let pool = Pool::new(nth);

		pool.scoped(|scope| {
			for q in &mut queens {		
				let running = Arc::clone(&running);
				scope.execute(move || q.scan_all_mt(1, &running) );	
			}
		});

		pool.shutdown();

		// aggregate solutions
		for q in queens { self.solutions.append(&mut q.solutions.clone())	}
		
	}

	pub fn print_solutions(&self) {
		self.solutions.iter().for_each(|s| println!("{}", s.to_string()) )
	}

	pub fn draw(&self, ctx: &mut PaintCtx) {
		let n = self.n;
		let nq = n as f64;

		let (w, h) = (ctx.size().width, ctx.size().height);
		let (nw, nh) = (w / nq, h / nq);

		let (sz, x0, y0) = if w > h { (nh, (w - h) / 2., 0.0) } else { (nw, 0.0, (h - w) / 2.) };

		let mut ff = true;

		ctx.fill(Rect::new(0., 0., w, h), &Color::WHITE); // cls

		for i in 0..n  {
			for j in 0..n {
				let (xp, yp) =(x0 + i as f64 * sz, y0 + (nq - j as f64 - 1.) * sz);
				
				if ff { ctx.fill(Rect::new(xp, yp, xp+sz, yp+sz), &Color::from_rgba32_u32(0xdd_dd_dd_ff)) } 
				ff = !ff;

				// if has queen
				if self[i as usize]==j {
					let (sz2, sz21, sz22) = (sz / 2., sz / 2.2, sz / 4.);
					
					ctx.fill(Ellipse::new(Point::new(sz2+xp, sz2+yp), (sz21, sz21), 0.), &Color::RED);
					ctx.fill(Ellipse::new(Point::new(sz2+xp, sz2+yp), (sz22, sz22), 0.), &Color::YELLOW);
				}
			}
			if (n & 1) == 0 { ff = !ff }
		}		
	}

	// transformations
	pub fn transformations(&mut self) {

		fn translate_vert(q : &mut Queen) {  // up
			for i in 0..q.n { q.board.set(i, (q[i as usize] + 1) % q.n) }

			q.save_solution()
		}
		fn translate_horz(q : &mut Queen) {  // right
			let mut v = vec![0; q.n as usize];

			for i in 0..q.n-1 { v[i as usize + 1] = q[i as usize] }
			v[0] = q[q.n as usize - 1];
			q.board.board = v;

			q.save_solution()
		}
		fn rotate90(q : &mut Queen) {
			let mut rot_queens = vec![0; q.n as usize];

			for i in 0..q.n {
			  rot_queens[i as usize] = 0;
			  for j in 0..q.n {  // find i
				if q[j as usize] == i {
				  rot_queens[i as usize] = q.n - j - 1;
				  break
				}
			  }
			}
			q.board.board = rot_queens;

			q.save_solution()
		}

		fn mirror_horz(q : &mut Queen) {
			for i in 0..q.n { q.board.set(i, (q.n - 1) - q[i as usize]) }

			q.save_solution()
		}
		fn mirror_vert(q : &mut Queen) {
			for i in 0..q.n/2 {
				let tmp = q[i as usize];
				q.board.set(i, q[(q.n - 1 - i) as usize]);
				q.board.set(q.n - 1 - i, tmp)
			}
			
			q.save_solution()
		}

		self.clear_counters();

		for _mv in 0..2 {
			for _mh in 0..2 {
				for _r90 in 0..4 {
					for _tv in 0..self.n {  // translations
						for _th in 0..self.n {
							translate_vert(self);	 // tV
						}
						translate_horz(self);  // tH
					}
					rotate90(self); // R90
				}
				mirror_horz(self); // mH
			}
			mirror_vert(self); // mV
		}

		self.sort_solutions_unique();		
	}
}
