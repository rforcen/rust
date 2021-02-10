/*
	zExpression zvm
*/

#![allow(dead_code)]

use std::f32::consts::{PI, E};
use num::complex::Complex as complex;

use druid::{
    AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, ExtEventSink, Lens, LocalizedString,
    Selector, Target, Widget, WidgetExt, WindowDesc,
};



#[derive(Debug, Copy, Clone, PartialEq)]
enum Symbols {
    SNULL = 0, NUMBER = 1, IDENTi = 2, IDENTz = 3, PLUS = 5, MINUS = 6,
    MULT = 7, DIV = 8, OPAREN = 9, CPAREN = 10, POWER = 12, PERIOD = 13, COMMA=14,
    
    // function names
    FSIN = 90, FCOS = 91, FTAN = 92, FEXP = 93, FLOG = 94, FLOG10 = 95,
    FINT = 96, FSQRT = 97, FASIN = 98, FACOS = 99, FATAN = 100, FABS = 101,
    FC = 102, SPI = 103, SPHI = 104,
    PUSHC = 112, PUSHZ = 113, PUSHI = 114, PUSHCC = 115,
    NEG = 116,
    
    END = 200
}

const FUNC_NAMES : [&'static str; 15] = ["sin", "cos", "tan", "exp", "log", "log10", "int", "sqrt", "asin",	"acos", "atan", "abs", "c", "pi", "phi"];

const PHI : f32 = 0.618033988_f32;

pub type CF32 = complex<f32>;
pub type ZFN = fn(CF32) -> CF32;

#[derive(Clone, Debug, Data)]
pub struct ZVm {
	pub source	:String,
	#[data(ignore)]ch		:char,
	#[data(ignore)]ixs		:usize,
	#[data(ignore)]sym		:Symbols,
	#[data(ignore)]ident	:String,
	#[data(ignore)]nval		:f32,
	#[data(ignore)]err 		:bool,
	#[data(ignore)]code		:Vec<u32>,
}
impl Default for ZVm { fn default() -> Self { ZVm::new("") } }


impl ZVm {
	pub fn new(source : &str) -> Self {
		let mut s=Self {
			source	:String::from(source), 
			ch		:' ', 
			ixs		:0, 
			sym		:Symbols::SNULL, 
			ident	:"".to_string(), 
			nval	:0_f32, 
			err		:false, 
			code	:vec![], 
		};
		s.compile();
		s
	}

	fn u32_to_f32(u : u32) -> f32 {	f32::from_ne_bytes(u.to_ne_bytes()) }
	fn f32_to_u32(f : f32) -> u32 {	u32::from_ne_bytes(f.to_ne_bytes()) }
	fn u32_2_sym(c : u32) -> Symbols { unsafe { ::std::mem::transmute(c as u8) } }
	fn usize_2_sym(c : usize) -> Symbols { unsafe { ::std::mem::transmute(c as u8) } }

	fn getch(&mut self) -> char {
		self.ch='\0';
		if self.ixs < self.source.len() {
			self.ch = self.source.chars().nth(self.ixs).unwrap();
			self.ixs+=1;
		} 
		self.ch
	}

	fn getsym(&mut self) -> Symbols {
		self.sym = Symbols::SNULL;
		self.ident.clear();
		self.nval = 0_f32;

		// skip whites
		while self.ch != '\0' && self.ch <= ' ' { let _ = self.getch(); }

		// scan symbol
		if self.ch.is_alphabetic() { // ident
			while self.ch.is_alphanumeric() || self.ch=='_' {
				self.ident.push(self.ch);
				self.getch();
			}
			
			if self.ident == "z" { self.sym=Symbols::IDENTz }
			else if self.ident == "i" { self.sym = Symbols::IDENTi }
			else { // func ?
				let index = FUNC_NAMES.iter().position(|&r| r == self.ident);
				if index.is_some() { // sym = FSIN + index
					self.sym = unsafe { ::std::mem::transmute(Symbols::FSIN as u8 + index.unwrap() as u8) }
				} else { // error
					self.sym = Symbols::SNULL
				}				
			}
		} else if self.ch.is_digit(10) { // number
			while self.ch.is_digit(10) || self.ch=='.' || self.ch=='e' || self.ch=='E' {
				self.ident.push(self.ch);
				self.getch();
			}
			self.sym = Symbols::NUMBER;
			self.nval = self.ident.parse::<f32>().unwrap(); // atof
		} else {
			self.sym = 
				match self.ch {
					'+' => Symbols::PLUS,
					'-' => Symbols::MINUS,
					'*' => Symbols::MULT,
					'/' => Symbols::DIV,
					'(' => Symbols::OPAREN,
					')' => Symbols::CPAREN,
					'^' => Symbols::POWER,
					'.' => Symbols::PERIOD,
					',' => Symbols::COMMA,
					_   => Symbols::SNULL,				
				};
			self.getch();
		}
		self.sym
	}

	fn getsym_check(&mut self, chk_sym : Symbols) -> Symbols {
		if self.getsym() != chk_sym { self.err = true; self.sym = Symbols::SNULL }
		self.sym
	}
	fn sym_check(&mut self, chk_sym : Symbols) -> Symbols {
		if self.sym != chk_sym { self.err = true; self.sym = Symbols::SNULL }
		else { self.getsym(); }
		self.sym
	}

	fn getsym_not_null(&mut self) -> Symbols {
		if self.getsym() == Symbols::SNULL { self.err = true }
		self.sym
	}

	fn gen(&mut self,  sym : Symbols) {
		
		match sym {
			Symbols::PUSHC => {
				self.code.push(sym as u32);
				self.code.push(Self::f32_to_u32(self.nval)); // self.nval
			}
			_ => {	
				self.code.push(sym as u32) ;
			}
		}
	}

	fn c_e3(&mut self) {
		

		if !self.err {
			
			match self.sym {
				Symbols::OPAREN => {
					self.getsym();
					self.c_e0();
					self.sym_check(Symbols::CPAREN);
				}
				Symbols::NUMBER => {
					self.gen(Symbols::PUSHC); // nval
					self.getsym();
				}
				Symbols::IDENTi => {
					self.gen(Symbols::PUSHI);
					self.getsym();
				}
				Symbols::IDENTz => {
					self.gen(Symbols::PUSHZ);
					self.getsym();
				}
				Symbols::PLUS => {
					self.getsym();
					self.c_e3();
				}
				Symbols::MINUS => {
					self.getsym();
					self.c_e3();
					self.gen(Symbols::NEG);
				}
				Symbols::FSIN  | Symbols::FCOS | Symbols::FTAN |  Symbols::FASIN |
                Symbols::FACOS | Symbols::FATAN| Symbols::FEXP |  Symbols::FINT  |
                Symbols::FABS  | Symbols::FLOG | Symbols::FLOG10| Symbols::FSQRT => {
					let tsym = self.sym;
                    self.getsym_check(Symbols::OPAREN);
                    self.c_e3();
                    self.gen(tsym);
				}
				Symbols::FC => {
					self.getsym_check(Symbols::OPAREN);
					self.getsym();
                    self.c_e3();
                    self.sym_check(Symbols::COMMA);
                    self.c_e3();
                    self.sym_check(Symbols::CPAREN);
                    self.gen(Symbols::FC);
				},
				Symbols::SPI => {
                    self.getsym();
                    self.nval=PI; self.gen(Symbols::PUSHC);
				},
                Symbols::SPHI => {
                    self.getsym();
                    self.nval=PHI; self.gen(Symbols::PUSHC);
				},

				Symbols::SNULL => {},
					
				_ => { 
					self.err=true;					
				}
			}
		}
	}

	fn c_e2(&mut self) {
		
		if !self.err {
			
			self.c_e3();

			loop {
				match self.sym {
					Symbols::POWER => {
						self.getsym_not_null();
						self.c_e2();
						self.gen(Symbols::POWER);
					},
					_ => { break }
				}
			}
		}
	}

	fn c_e1(&mut self) {
		if !self.err {

			self.c_e2();
			
			loop {
				match self.sym {
					Symbols::MULT => {
						self.getsym_not_null();
						self.c_e2();
						self.gen(Symbols::MULT);
					},
					Symbols::DIV => {
						self.getsym_not_null();
						self.c_e2();
						self.gen(Symbols::DIV);
					}
					_ => { break }
				}
			}
		}
	}

	fn c_e0(&mut self) {
		
		if !self.err {
			
			self.c_e1();
			
			loop {
				match self.sym {
					Symbols::PLUS => {
						self.getsym_not_null();
						self.c_e1();
						self.gen(Symbols::PLUS);
					},
					Symbols::MINUS => {
						self.getsym_not_null();
						self.c_e1();
						self.gen(Symbols::MINUS);
					}
					_ => { break }
				}
			}
		}
	}

	pub fn compile(&mut self) {
		self.err = false;

		self.getsym();
		self.c_e0();

		if self.err { self.code.clear() }
		self.gen(Symbols::END);
	}
	
	pub fn eval(&self, z: CF32) -> CF32 {
		
		if self.err { return CF32::new(0., 0.) }

		let mut pc : usize = 0;
		let mut sp : usize = 0;
		let mut stack : Vec<CF32> = vec![CF32::new(0.,0.); 16];

		loop {
			match Self::u32_2_sym(self.code[pc]) {
				Symbols::PUSHC => {
					pc+=1;
					stack[sp] = CF32::new(Self::u32_to_f32( self.code[pc] ), 0.);
					sp+=1
				}
				Symbols::PUSHZ => {
					stack[sp] = z;
					sp+=1
				}
				Symbols::PUSHI => {
					stack[sp] = CF32::new(0., 1.);
					sp+=1
				}
				Symbols::PLUS  => {	sp-=1;	let zz = stack[sp];	stack[sp - 1] += zz;	}
				Symbols::MINUS => {	sp-=1;	let zz = stack[sp];	stack[sp - 1] -= zz;	}
				Symbols::MULT  => {	sp-=1;	let zz = stack[sp];	stack[sp - 1] *= zz;	}
				Symbols::DIV   => {	sp-=1;	let zz = stack[sp];	stack[sp - 1] /= zz;	}
				Symbols::POWER => {	
					sp-=1;	
					let zz = stack[sp]; 
					let zz = stack[sp-1].powc(zz);	
					stack[sp - 1] += zz;	
				}
				Symbols::NEG   => {	sp-=1;	let zz = stack[sp - 1];	stack[sp - 1] = -zz; }

				Symbols::FSIN   => { stack[sp - 1] = stack[sp - 1].sin()	}
				Symbols::FCOS   => { stack[sp - 1] = stack[sp - 1].cos()	}
				Symbols::FTAN   => { stack[sp - 1] = stack[sp - 1].tan()	}
				Symbols::FASIN  => { stack[sp - 1] = stack[sp - 1].asin()	}
				Symbols::FACOS  => { stack[sp - 1] = stack[sp - 1].acos()	}
				Symbols::FATAN  => { stack[sp - 1] = stack[sp - 1].atan()	}
				Symbols::FEXP   => { stack[sp - 1] = stack[sp - 1].exp()	}
				Symbols::FLOG   => { stack[sp - 1] = stack[sp - 1].log(E)	}
				Symbols::FLOG10 => { stack[sp - 1] = stack[sp - 1].log(10.)}
				Symbols::FSQRT  => { stack[sp - 1] = stack[sp - 1].sqrt()	}
				Symbols::FABS   => { stack[sp - 1] = CF32::new(stack[sp - 1].arg(), 0.) }
				Symbols::FC     => { sp-=1; stack[sp - 1] = CF32::new(stack[sp - 1 ].re, stack[sp].re)	}
						
				Symbols::END | _ => { break }
			}
			pc+=1;
		}

		if sp!=0 { stack[ sp - 1 ]   }
		else     { CF32::new(0., 0.) }
	}

	
	pub fn ok(&self) -> bool { !self.err }

	#[allow(dead_code)]
	pub fn walk(&mut self) {
		println!("{}\n------", self.source);
		loop {
			match self.getsym() {
				Symbols::SNULL => break,
				_ => println!("{:?}\t\t{}", self.sym, self.nval),
			}		
		}
		println!("------");
		println!("{:?}", self.code);

		let mut i : usize =0;
		loop {
			

			let sym : Symbols = unsafe { ::std::mem::transmute(self.code[i] as u8) };
			match sym {
				Symbols::PUSHC => {
					println!("{:?} {}", sym, Self::u32_to_f32( self.code[i+1] ));
					i+=1					
				},
				Symbols::END => { 
					println!("{:?}", sym);
					break
				},
				_ => {
					println!("{:?}", sym);		
				}
			}
			i+=1;			
		}
	}
}