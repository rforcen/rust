// Compiler

#![allow(dead_code)]

use std::mem::swap;
use std::f32::{consts::{PI, E}};
const PHI : f32 = 1.618;
const MAX_STACK : usize = 256;

#[path = "scanner.rs"] mod scanner;
use scanner::*;

// table values
#[derive(Clone, Copy, Debug)]
enum IdentType { NumId, PARAM, FUNC }

#[derive(Clone, Debug)]
struct TableValues {
	id		: String,
	itype	: IdentType,
	value	: f32,
	address : u32, 
	param_ix: u32, 
	n_params: u32, 
	i_func  : u32,
    is_const: bool,
}

impl TableValues {
	fn new(id:String, itype:IdentType, is_const:bool) -> Self {
		Self { id, itype, is_const, value:0.0, address:0, param_ix:0, n_params:0, i_func:0}
	}
	fn new_func(id:String, address:u32) -> Self {
		Self { id, itype:IdentType::FUNC, is_const:false, value:0.0, address, param_ix:0, n_params:0, i_func:0}
	}
	fn new_param(id:String, param_ix:u32, i_func:u32) -> Self {
		Self { id, itype:IdentType::PARAM,  is_const:false, value:0.0, address:0, param_ix, n_params:0, i_func}		
	}
	fn default() -> Self {
		Self { id:String::default(), itype:IdentType::NumId, is_const:false, value:0.0, address:0, param_ix:0, n_params:0, i_func:0}
	}
}

// pcode
#[derive(Clone, Copy, Debug)]
enum Pcode {
	Arg0(Symbol),
	Arg1i(Symbol, u32),
	Arg2i(Symbol, u32, u32),
	Arg1f(Symbol, f32)
}

// block address
type FromTo = (u32, u32);
#[derive(Clone, Debug)]
struct BlockAddress {
	_const	: FromTo, 
	_let	: FromTo,
	_func	: FromTo,
	_code	: Vec<FromTo>,
    last_to	: u32,
}


impl BlockAddress {
	fn new() -> Self {
		let z = (0,0);
		Self { _const:z, _let:z, _func:z, _code:vec![z; 256], last_to:0 }
	}

	fn set_code(&mut self, chan : u32, t : u32) { self._code[chan as usize] = (self.last_to, t);   	self.last_to = t;  }
	fn set_const(&mut self, f : u32, t : u32)   { self._const = (f, t);       						self.last_to = t;  }
	fn set_let(&mut self, t : u32) 				{ self._let = (self.last_to, t);      				self.last_to = t;  }
    fn set_func(&mut self, t : u32) 			{ self._func = (self.last_to, t);			        self.last_to = t;  }

	fn get_const(&self) -> FromTo { self._const }
    fn get_let(&self) -> FromTo { self._let }
    fn get_code(&self, chan : u32) -> FromTo { self._code[chan as usize] }
    fn get_codes(&self) -> Vec<FromTo> { self._code.clone() }
}

#[derive(Clone, Debug)]
pub struct Params {
	pub sample_rate : u32,
	pub volume		: f32,
	pub seconds		: f32,
}

// compiler
#[derive(Clone, Debug)]
pub struct Compiler <'a> {
	scanner 	: Scanner<'a>,
	notation 	: Symbol,
	tab_values	: Vec<TableValues>,
	code 		: Vec<Pcode>,
	blk_addr	: BlockAddress, // from, to

	pub err			: bool,
	pub chan		: u32,
	pub params 		: Params, // seconds, sampla_rate, volume
}

// Compiler
impl<'a> Compiler<'a> {
	pub fn new(source : String) -> Self {
		Self { 	
			scanner 	: Scanner::new(source), 
			notation 	: Symbol::ALGEBRAIC, 
			err 		: false,
			chan		: 0,
			tab_values	: vec![],
			code		: vec![],
			blk_addr	: BlockAddress::new(),
			params		: Params{ seconds:1., sample_rate:44100, volume:0.5}
		}
	}

	fn sym(&self) -> Symbol {  self.scanner.sym }
	fn pc(&self) -> u32 { self.code.len() as u32 }
	fn check_sym(&mut self, sym : Symbol) { self.err = self.sym() != sym }

	pub fn code_len(&self) -> usize { self.code.len() } 

	pub fn error_message(&self) -> String {	self.scanner.get_error_msg() }

	pub fn compile(&mut self) -> bool {

		match self.scanner.getsym() {
            Symbol::ALGEBRAIC => {
            	self.scanner.getsym(); // getsym(SEMICOLON);
            	self.scanner.getsym();
            	self.notation = Symbol::ALGEBRAIC;
            	self.compile_algebraic();
			}
           	Symbol::RPN => {
            	self.notation = Symbol::RPN;
            	self.scanner.getsym();
            	self.scanner.getsym();
            	self.compile_rpn();
			}
			_ => {
            	self.notation = Symbol::ALGEBRAIC;
            	self.compile_algebraic();
			}
		}

		if !self.err { self.exec_const() }

		!self.err
	}

	fn compile_algebraic(&mut self) -> bool {
		self.parse_const();  // const let var0=expr, var1=expr;

		if !self.err {
			self.parse_let();
			self.parse_funcs();

			let mut ls : Symbol;
			loop {
				self.expr_0();  //  expr per channel;

				ls = self.sym();

				if !self.err && self.sym() == Symbol::SEMICOLON {
					self.blk_addr.set_code(self.chan, self.pc());
					self.chan+=1;
					self.check_sym(Symbol::SEMICOLON);
					self.scanner.getsym();
				}
				if ! (ls == Symbol::SEMICOLON && !self.err) { break }
			} 
		}
		!self.err
	}

	fn parse_id_eq_expr(&mut self, is_const : bool) {  // id=expr, id=expr;
		loop {
			if self.scanner.getsym() == Symbol::IDENT {
				let id = self.scanner.get_id();
				if self.scanner.getsym() == Symbol::EQ {
					self.scanner.getsym();
	
					match self.notation {
						Symbol::ALGEBRAIC 	=> { self.expr_0() }
						Symbol::RPN 		=> { self.rpn_expr() }
						_ => ()
					}
	
					self.generate_1(Symbol::POP, self.tab_values.len());
					self.tab_values.push(TableValues::new(id, IdentType::NumId, is_const));
					
	
				} else { self.err = true }
			} else { self.err = true }
			if !(self.sym() == Symbol::COMMA && !self.err) {break}
		} 
  
		if self.sym() == Symbol::SEMICOLON {	self.scanner.getsym(); } 
		else {  self.err = true	}
	}
  
	// generate code

	fn generate(&mut self, token : Symbol) {						self.code.push(Pcode::Arg0(token))		}
	fn generate_1(&mut self, token : Symbol, i : usize) {			self.code.push(Pcode::Arg1i(token, i as u32)) }
	fn generate_2(&mut self, token : Symbol, i0 : u32, i1 : u32) {  self.code.push(Pcode::Arg2i(token, i0, i1)) 	}
	fn generate_f(&mut self, token : Symbol, f : f32) {				self.code.push(Pcode::Arg1f(token, f))	}

	fn check_getsym(&mut self, sym : Symbol) {
		self.err = self.sym() != sym;
		self.scanner.getsym();
	}

	fn parse_const(&mut self) {
		if self.sym() == Symbol::CONST { self.parse_id_eq_expr(true) } // const sample_rate=expr, bits_sample=expr;
		self.blk_addr.set_const(0, self.pc());
	}
	fn parse_let(&mut self) {
		if self.sym() == Symbol::LET { self.parse_id_eq_expr(false); }
      	self.blk_addr.set_let(self.pc());
	}

	fn parse_funcs(&mut self) {
		while self.sym() == Symbol::FUNC {
			self.scanner.getsym();
	
			self.tab_values.push(TableValues::new_func(self.scanner.get_id(), self.pc()));
	
			let ixtv = self.tab_values.len() as u32;
			let i_func = ixtv - 1;
			let mut param_ix = 0;
	
			if self.scanner.getsym() == Symbol::OPAREN {
			  loop {
				self.scanner.getsym();
				self.tab_values.push(TableValues::new_param(self.scanner.get_id(), param_ix,  i_func));
				param_ix+=1;
				if self.scanner.getsym() != Symbol::COMMA {break}
			  } 
			  self.check_getsym(Symbol::CPAREN);
			}
			self.check_getsym(Symbol::RET);  // ->
	
			match self.notation {
			  Symbol::RPN 		=> { self.rpn_expr() }
			  Symbol::ALGEBRAIC => { self.expr_0() }
			  _ => ()
			}
			self.check_getsym(Symbol::SEMICOLON);
	
			self.tab_values.resize(ixtv as usize, TableValues::default());  // remove refs. to parameters
			self.tab_values.last_mut().unwrap().n_params = param_ix; 	 	// save # of args in
	
			self.generate_1(Symbol::RET, param_ix as usize);
		  }
		  self.blk_addr.set_func(self.pc());
		  // jump over fun def
	}

	

	fn compile_rpn(&mut self) -> bool { 
		self.parse_const();  // const let var0=expr, var1=expr;

		if !self.err {
			self.parse_let();
			self.parse_funcs();

			loop {
				self.rpn_expr();

				self.check_getsym(Symbol::SEMICOLON);
				self.blk_addr.set_code(self.chan, self.pc());
				self.chan+=1;
				if self.sym() == Symbol::SNULL { break }
			} 
		}
		return !self.err;
	}

	fn rpn_expr(&mut self) {
		loop {
			match self.sym() {
			  Symbol::NUMBER => self.generate_f(Symbol::PUSH_CONST, self.scanner.get_num()),
				//  't' special var is the parameter in eval call
			  Symbol::IDENT_t => self.generate(Symbol::PUSH_T),
			  Symbol::IDENT  => {
				let idix = self.get_ident_index();
				if idix != -1 {
				  let tv = self.tab_values[idix as usize].clone();
				  match tv.itype {
					IdentType::NumId => self.generate_1(Symbol::PUSH_ID, idix as usize),
					IdentType::PARAM => self.generate_2(Symbol::PARAM, tv.param_ix, tv.i_func),
					IdentType::FUNC  => self.generate_2(Symbol::FUNC, tv.address, tv.n_params),
				  }
				} else { self.err = true }
			  }
	
			  Symbol::SPI 	=> self.generate_f(Symbol::PUSH_CONST, PI),
			  Symbol::SPHI	=> self.generate_f(Symbol::PUSH_CONST, PHI),
			  Symbol::TILDE	=> self.generate(Symbol::SWAVE1),
			  Symbol::SEQUENCE => self.generate(Symbol::SEQUENCE),
			  Symbol::FREQ_MESH => {},
			  Symbol::BACKSLASH => {  // \{operator:+-*/} compress stack w/operator
				let operators = [Symbol::PLUS, Symbol::MINUS, Symbol::MULT, Symbol::DIV, Symbol::TILDE];
				self.generate(Symbol::BACKSLASH);
				let sym = self.scanner.getsym();
				if let Some(_) = operators.iter().position(|&x| x==self.sym()) {  self.generate(sym) }
				else {  self.err = true }
			  } 
	
			  Symbol::YINYANG | Symbol::MINUS |	Symbol::PLUS |  Symbol::DIV   |	Symbol::MULT  | Symbol::RATE  |	
			  Symbol::FSIN 	  | Symbol::FCOS  |	Symbol::FTAN |	Symbol::FASIN |	Symbol::FACOS | Symbol::FATAN |
			  Symbol::FEXP 	  | Symbol::FINT  | Symbol::FABS |	Symbol::FLOG  | Symbol::FLOG10| Symbol::FSQRT |
			  Symbol::SEC     | Symbol::OSC   | Symbol::ABS =>
				self.generate(self.sym()),
	
			//   Symbol::NOTE_CONST=> self.generate_2(NOTE_CONST, parser->get_i0(), parser->get_i1());
	
			  Symbol::SNULL => break,
			  _ => { self.err = true; break }
			}
	
			self.scanner.getsym();
			if  self.err || !(self.sym() != Symbol::SEMICOLON && self.sym() != Symbol::COMMA && self.sym() != Symbol::SNULL) { break }
		  } 
	}		

	

	fn starts_implicit_mult(&self) -> bool {
		let implicit_mult_start = [
				Symbol::IDENT, Symbol::IDENT_t, Symbol::OCURL,  Symbol::OSQARE, Symbol::OLQUOTE, Symbol::OPAREN,
				Symbol::SPI,   Symbol::SPHI,    Symbol::NUMBER, Symbol::RANDOM, Symbol::TILDE,   Symbol::SEQUENCE];

		implicit_mult_start.iter().any(|&i| i==self.sym()) ||  (self.sym() >= Symbol::FSIN && self.sym() <= Symbol::MAGNETICRING)
	}

	fn get_ident_index(&self) -> i32 { // Result<usize, bool>
		if let Some(ix) = self.tab_values.iter().position(|v| v.id == self.scanner.get_id()) {
			ix as i32
		} else {
			-1
		}
	}

	fn get_val(&self, id : String) -> f32 {
		if let Some(item) = self.tab_values.iter().find(|&v| v.id == id) {		item.value		} 
		else {	0.0 }
	}

	fn expr_0(&mut self) {
		if !self.err {
			let is_neg = self.sym() == Symbol::MINUS;
			if is_neg { self.scanner.getsym(); }
	
			self.expr_1();
	
			if is_neg { self.generate(Symbol::NEG) }
	
			let op_set = [Symbol::EQ, Symbol::NE, Symbol::LT, Symbol::LE, Symbol::GT, Symbol::GE, Symbol::PLUS, Symbol::MINUS];
			loop {
			  let sym_op = self.sym();
			  if op_set.iter().any(|&i| i==self.sym()) {
				self.scanner.getsym();
				self.expr_1();
				self.generate(sym_op);
			  }
			  if ! op_set.iter().any(|&i| i==self.sym()) {break}
			} 
		  }			
	}

	fn expr_1(&mut self) {
		if !self.err {
			self.expr_2();
			loop {
			  let sym_op = self.sym();
			  if self.starts_implicit_mult() {  // not operator-> implicit *, i,e.  2{440}
				self.expr_2();
				self.generate(Symbol::MULT);
			  } else {
				match self.sym() {
				 Symbol::MULT | Symbol::DIV  => {
					self.scanner.getsym();
					self.expr_2();
					self.generate(sym_op);
				  }
				  _ => break
				}
			  }
			  if !(self.sym() == Symbol::MULT || self.sym() == Symbol::DIV || self.starts_implicit_mult()) {break}
			} 
		  }		
	}
	fn expr_2(&mut self) {
		if !self.err {
			self.expr_3();
			loop {
			  if self.sym() == Symbol::POWER {
				self.scanner.getsym();
				self.expr_3();
				self.generate(Symbol::POWER);
			  }
			  if !(self.sym() == Symbol::POWER) {break}
			} 
		  }		
	}

	fn expr_3(&mut self) {
		if !self.err {
			match self.sym() {
			  Symbol::OPAREN => {
				self.scanner.getsym();
				self.expr_0();
				// check_sym(CPAREN);
				self.scanner.getsym();
			  }
			  Symbol::NUMBER => {
				self.generate_f(Symbol::PUSH_CONST, self.scanner.get_num());
				self.scanner.getsym();
			  }
			  Symbol::FLOAT => {
				self.generate_f(Symbol::PUSH_CONST, -32.);  // this is the floating_point=true value
				self.scanner.getsym();
			  }
	
			  Symbol::IDENT_t => { //  't' special var is the parameter in eval call
				self.generate(Symbol::PUSH_T);
				self.scanner.getsym();
			  }
	
			  Symbol::IDENT => {
				let idix = self.get_ident_index();
				if idix != -1 {
				  let tv = self.tab_values[idix as usize].clone();
				  match tv.itype {
					IdentType::NumId => self.generate_1(Symbol::PUSH_ID, idix as usize),
					IdentType::PARAM  => self.generate_2(Symbol::PARAM, tv.param_ix, tv.i_func),
					IdentType::FUNC	  => {
					  if tv.n_params != 0 {
						self.scanner.getsym(); // getsym(OPAREN);
						self.scanner.getsym();
						for _ in 0..tv.n_params-1 {
						  	self.expr_0();
							self.check_getsym(Symbol::COMMA);
						}
						self.expr_0();
						// check_sym(CPAREN);
					  }
					  self.generate_2(Symbol::FUNC, tv.address, tv.n_params);
					}
				  }
				} else { self.err=true  }
				self.scanner.getsym();
			  }
				
			  Symbol::MINUS => {
				self.scanner.getsym();
				self.expr_3();
				self.generate(Symbol::NEG);
			  }
			  Symbol::PLUS => {
				self.scanner.getsym();
				self.expr_3();  // +expr nothing to generate
			  }
			  Symbol::FACT => {
				self.scanner.getsym();
				self.expr_3();
				self.generate(Symbol::FACT);
			  }
			  Symbol::TILDE => {
				self.scanner.getsym();
				self.expr_3();
				self.generate(Symbol::SWAVE1);
			  }
			  Symbol::YINYANG => {
				self.scanner.getsym();
				self.expr_3();
				self.generate(Symbol::YINYANG);
			  }
	
			  Symbol::SEQUENCE  => {  // (from, to, inc)
				self.scanner.getsym(); // OPAREN);
				self.scanner.getsym();
				self.expr_0();
				self.check_getsym(Symbol::COMMA);
				self.expr_0();
				self.check_getsym(Symbol::COMMA);
				self.expr_0();
				self.check_getsym(Symbol::COMMA);
				self.generate(Symbol::SEQUENCE);
			  }
	
			  Symbol::FREQ_MESH => {  // (base, slope, islope, n)
				self.scanner.getsym(); // OPAREN);
				self.scanner.getsym();
				self.expr_0();
				self.check_getsym(Symbol::COMMA);
				self.expr_0();
				self.check_getsym(Symbol::COMMA);
				self.expr_0();
				self.check_getsym(Symbol::COMMA);
				self.expr_0();
				self.check_getsym(Symbol::CPAREN);
				// self.generate_2(Symbol::FREQ_MESH, int(freq_mesh.size()));
				// freq_mesh.push_back(FreqMesh());
			  }
	
			  Symbol::RANDOM => {
				self.generate_f(Symbol::PUSH_CONST, 0.); // random
				self.scanner.getsym();
			  }
	
			  Symbol::OCURL => {  // {hz}, {amp,hz}, {amp, hz, phase}
				self.scanner.getsym();
				self.expr_0();
				if self.sym() == Symbol::COMMA {
				  self.scanner.getsym();
				  self.expr_0();
				  if self.sym() == Symbol::COMMA {
					self.scanner.getsym();
					self.expr_0();
					self.generate(Symbol::SWAVE);
				  } else {
					self.generate(Symbol::SWAVE2);
				  }
				} else {
				  self.generate(Symbol::SWAVE1);
				}
				self.check_getsym(Symbol::CCURL);
				}
	
			  Symbol::OSQARE => {  // []==sec
				self.scanner.getsym();
				self.expr_0();
				
				self.generate(Symbol::SEC);
				self.check_getsym(Symbol::CSQUARE);
			  }
	
			  Symbol::VERT_LINE => {  // |abs|
				self.scanner.getsym();
				self.expr_0();
	
				self.generate(Symbol::ABS);
				self.check_getsym(Symbol::VERT_LINE);
			  }
	
			  Symbol::OLQUOTE => {  // «f»  -> exp(f*t)
				self.scanner.getsym();
				self.expr_0();
				self.check_getsym(Symbol::CLQUOTE);
	
				self.generate(Symbol::PUSH_T);
				self.generate(Symbol::MULT);
				self.generate(Symbol::FEXP);
			  }
	
			  Symbol::BACKSLASH => {  // \s:e\ -> lap(start, end)
				self.scanner.getsym();
				if self.sym() == Symbol::COLON {  // \:e\ -> lap(0, end)
				  self.generate_f(Symbol::PUSH_CONST, 0.0);
				} else { self.expr_0(); }
				self.scanner.getsym();  // :
				self.expr_0();
				self.check_getsym(Symbol::BACKSLASH);  // '\'
				self.generate(Symbol::LAP);
			  }
	
			  Symbol::RATE |  Symbol::FSIN |  Symbol::FCOS |  Symbol::FTAN |
			  Symbol::FASIN|  Symbol::FACOS|  Symbol::FATAN|  Symbol::FEXP |
			  Symbol::FINT |  Symbol::FABS |  Symbol::FLOG |  Symbol::FLOG10|
			  Symbol::FSQRT|  Symbol::SEC  |  Symbol::OSC  | Symbol::ABS =>
			   {
				let tsym = self.sym();
				self.scanner.getsym();
				self.expr_3();
				self.generate(tsym);
			  }
	
			  Symbol::SPI => {
				self.scanner.getsym();
				self.generate_f(Symbol::PUSH_CONST, PI);
			  }
	
			  Symbol::SPHI => {
				self.scanner.getsym();
				self.generate_f(Symbol::PUSH_CONST, PHI);
			  }
	
			  Symbol::SWAVE => {  // wave(amp, hz, phase)
				self.scanner.getsym(); //Symbol::OPAREN);
				self.scanner.getsym();
				self.expr_0();
				self.check_getsym(Symbol::COMMA);
				self.expr_0();
				self.check_getsym(Symbol::COMMA);
				self.expr_0();
				self.check_getsym(Symbol::CPAREN);
	
				self.generate(Symbol::SWAVE);
			  }
	
			  Symbol::NOTE_CONST => {
				// generate(Symbol::NOTE_CONST, parser->get_i0(), parser->get_i1());
				self.scanner.getsym();
			  }
	
			  // 2 parameter funcs.
			  Symbol::NOTE 	|    // note(note#,oct)
			  Symbol::TONE	|    // tone(note#,oct)
			  Symbol::LAP	|     // lap(time1,time2)
			  Symbol::HZ2OCT =>  // hz2oct(hz,oct)
			  {
				let tsym = self.sym();
				self.scanner.getsym(); //Symbol::OPAREN);
				self.scanner.getsym();
				self.expr_0();
				self.check_getsym(Symbol::COMMA);
				self.expr_0();
				self.check_getsym(Symbol::CPAREN);
				self.generate(tsym);
			  }
	
			  Symbol::SAW => {  // saw(freq, alpha)
				self.scanner.getsym();
				self.scanner.getsym();
				self.expr_0();
				if self.sym() == Symbol::COMMA {
				  self.scanner.getsym();
				  self.expr_0();
				  self.scanner.getsym();
				  self.generate(Symbol::SAW);
				} else {
				  self.scanner.getsym();
				  self.generate(Symbol::SAW1);
				}
				}
	
			  Symbol::MAGNETICRING => {  // MagnetRing(Vol, Hz, Phase, on_count, off_count)
				self.scanner.getsym();
				self.scanner.getsym();
				self.expr_0();
				self.scanner.getsym();
				self.expr_0();
				self.scanner.getsym();
				self.expr_0();
				self.scanner.getsym();
				self.expr_0();
				self.scanner.getsym();
				self.expr_0();
				self.scanner.getsym();
				self.generate(Symbol::MAGNETICRING);
			  }
	
			  Symbol::SNULL => {}

			  _ => { self.err = true } // syntax error
			}
		  }		
	}

	// execution section

	pub fn t_inc(&self) -> f32 { 2. * PI / self.params.sample_rate as f32 }

	pub fn num_samples(&self) -> usize { (self.params.sample_rate as f32 * self.params.seconds) as usize }

	pub fn samples_size(&self) -> usize { self.num_samples() * self.chan as usize}

	pub fn execute(&mut self, t : f32) -> Vec<f32> {
		self.exec_let(t);
		(0..self.chan as usize).map(
			|ch| self.execute_range(t, self.blk_addr._code[ch].0 as usize, self.blk_addr._code[ch].1 as usize)
		).collect()
	}

	fn set_params(&mut self) {
		if let Some(item) = self.tab_values.iter().find(|&v| v.id == "seconds") 	{	self.params.seconds = item.value }
		if let Some(item) = self.tab_values.iter().find(|&v| v.id == "sample_rate") {	self.params.sample_rate = item.value as u32}
		if let Some(item) = self.tab_values.iter().find(|&v| v.id == "volume") 	 	{	self.params.volume = item.value }
	}

	pub fn exec_const(&mut self) {
		self.execute_range(0., self.blk_addr._const.0 as usize, self.blk_addr._const.1 as usize);
		self.set_params(); // wave params: sample_rate, volumen, seconds
	}
	pub fn exec_let(&mut self, t : f32) {
		self.execute_range(t, self.blk_addr._let.0 as usize, self.blk_addr._let.1 as usize);
	}

	pub fn exec_chan(&mut self, t : f32, ch : usize) -> f32 {
		self.execute_range(t, self.blk_addr._code[ch].0 as usize, self.blk_addr._code[ch].1 as usize)
	}

	fn execute_range(&mut self, t : f32, from_pc : usize, to_pc : usize ) -> f32 {
		fn factorial(x : f32) -> f32 { if x > 1. { x * factorial(x-1.) } else { 1. } }

		let mut n_params : Vec<u32> = vec![];
		let mut sp_base  : Vec<u32> = vec![];

		let mut stack = vec![0_f32; MAX_STACK];
		let mut sp = 0;
		let mut pc = from_pc;

		while pc < to_pc {
			match self.code[pc] {
				Pcode::Arg0(c) => {
					match c {
						Symbol::PUSH_T 	=> { stack[sp]=t; sp+=1 }
						Symbol::PLUS	=> { sp-=1; stack[sp-1] += stack[sp] }
						Symbol::MINUS	=> { sp-=1; stack[sp-1] -= stack[sp] }
						Symbol::MULT	=> { sp-=1; stack[sp-1] *= stack[sp] }
						Symbol::DIV		=> { sp-=1; stack[sp-1] /= stack[sp] }
						Symbol::EQ		=> { sp-=1; stack[sp-1] = if stack[sp-1] == stack[sp] {1.} else {0.} }
						Symbol::NE		=> { sp-=1; stack[sp-1] = if stack[sp-1] != stack[sp] {1.} else {0.} }
						Symbol::LT		=> { sp-=1; stack[sp-1] = if stack[sp-1] <  stack[sp] {1.} else {0.} }
						Symbol::LE		=> { sp-=1; stack[sp-1] = if stack[sp-1] <= stack[sp] {1.} else {0.} }
						Symbol::GT		=> { sp-=1; stack[sp-1] = if stack[sp-1] >  stack[sp] {1.} else {0.} }
						Symbol::GE		=> { sp-=1; stack[sp-1] = if stack[sp-1] >= stack[sp] {1.} else {0.} }
						Symbol::POWER	=> { sp-=1; stack[sp-1] = stack[sp-1].powf(stack[sp]) }

						Symbol::FACT	=> { stack[sp-1] = factorial(stack[sp-1]) }
						Symbol::NEG		=> { stack[sp-1] = -stack[sp-1] }
						Symbol::RATE	=> {}
						Symbol::FSIN	=> { stack[sp-1] = stack[sp-1].sin() }
						Symbol::FCOS	=> { stack[sp-1] = stack[sp-1].cos() }
						Symbol::FTAN	=> { stack[sp-1] = stack[sp-1].tan() }
						Symbol::FASIN	=> { stack[sp-1] = stack[sp-1].asin() }
						Symbol::FACOS	=> { stack[sp-1] = stack[sp-1].acos() }
						Symbol::FATAN	=> { stack[sp-1] = stack[sp-1].atan() }
						Symbol::FEXP	=> { stack[sp-1] = stack[sp-1].exp() }
						Symbol::FINT	=> { stack[sp-1] = stack[sp-1].abs() }
						Symbol::FABS	=> { stack[sp-1] = stack[sp-1].abs() }
						Symbol::FLOG	=> { stack[sp-1] = stack[sp-1].log(E) }
						Symbol::FLOG10	=> { stack[sp-1] = stack[sp-1].log10() }
						Symbol::FSQRT	=> { stack[sp-1] = stack[sp-1].sqrt() }
						Symbol::SEC		=> { stack[sp-1] = 2. * PI * stack[sp-1] }
						Symbol::OSC		=> { stack[sp-1] = (t * stack[sp-1]).sin() }
						Symbol::ABS		=> { stack[sp-1] = stack[sp-1].abs() }

						Symbol::SWAVE1	=> { stack[sp-1] = (t * stack[sp-1]).sin() } // wave(hz)							
						Symbol::SWAVE2	=> { stack[sp-2] *= (t * stack[sp-1]).sin(); sp-=1 } // wave(amp, hz)
						Symbol::SWAVE 	=> { stack[sp-3] = stack[sp-3] * (t * stack[sp-2] + stack[sp-1]).sin(); sp-=2 }  // wave(amp, freq, phase)

						Symbol::YINYANG	=> {
							let f = stack[sp-1];
							let k = 6. * PI;
							stack[sp-1] = (t * f).sin() * (f / (t + k)).sin();
						}
						Symbol::BACKSLASH => {  // \{}operator
							pc+=1;
							let mut res = 0.0;

							if sp > 1 {
								if let Pcode::Arg0(c) = self.code[pc] {
									if c == Symbol::TILDE {
										sp-=1; 
										loop { 
											res += (t * stack[sp]).sin(); 
											if sp == 0 { break }
											sp-=1 
										}
									} else {
										res = stack[sp-1];

										sp-=2; 
										pc+=1;
										
										loop {
											match self.code[pc] {
												Pcode::Arg0(c) => {
													match c {
														Symbol::PLUS 	=> res += stack[sp],
														Symbol::MINUS 	=> res -= stack[sp],
														Symbol::MULT	=> res *= stack[sp],
														Symbol::DIV		=> res /= stack[sp],
														_ => { self.err = true }
													}
												}
												_ => { self.err = true }

											}
											if sp == 0 { break }
											sp-=1;
										}
									}
								}
								
								stack[0] = res;
								sp = 1;
							}				
					  	}
			
						Symbol::SEQUENCE => {
							let n = stack[sp-1];
							let mut end = stack[sp-2];
							let mut ini = stack[sp-3];
							if n < MAX_STACK as f32 - 10. && end != ini && sp > 2 {
								if end < ini { swap(&mut ini, &mut end) }
								let inc = (end - ini) / (n - 1.);
								sp -= 3;
								
								let mut i = ini;
								while i < end { stack[sp]=i; sp+=1; i+=inc }
								stack[sp]=end; sp+=1
							}
						} 

						_ => { break }
					}
				}
				Pcode::Arg1i(c, i) => {
					match c {
						Symbol::PUSH_ID => { stack[sp]=self.tab_values[i as usize].value; sp+=1 }
						Symbol::POP		=> { sp-=1; self.tab_values[i as usize].value = stack[sp] }
						
						Symbol::RET		=> {
							let nr = i as usize;
							pc = (stack[sp-2]-1.) as usize; // as pc++
							stack[(sp-(nr+2)) as usize] = stack[sp-1];
							sp -= nr + 2 - 1;
							sp_base.pop();
							n_params.pop();
						}

						_ => { break }
					}
				}
				Pcode::Arg2i(c, i0, i1) => {
					match c {
						Symbol::PARAM	=> { // push param
							stack[sp] = stack[(sp_base.last().unwrap() - 1 - n_params.last().unwrap() + i0) as usize];
							sp += 1;
						}
						Symbol::FUNC	=> { // pc, nparams
							stack[sp]=(pc+1) as f32;
							sp+=1;
							n_params.push(i1);
							sp_base.push(sp as u32);
							pc = (i0-1) as usize; // as it will be inc +1
						}

						_ => { break }
					}
				}
				Pcode::Arg1f(c, f) => {
					match c {
						Symbol::PUSH_CONST => { stack[sp] = f; sp+=1 }

						_ => { break }
					}
				}
			}
			pc+=1
		}

		if sp==1 { stack[0] } else { 0.0 }
	}

	pub fn print_code(&self) {
		println!("symbol table:");
		for (i,v) in self.tab_values.iter().enumerate() {
			print!("{}: {}, ", i, v.id)
		}
		println!("\ncode:");
		for (pc, c) in self.code.iter().enumerate() {
			match c {
				Pcode::Arg0(c) => println!("{} {:?}", pc, c),
				Pcode::Arg1i(c, i) => println!("{} {:?} {}", pc, c, i),
				Pcode::Arg2i(c, i0, i1) => println!("{} {:?} {} {}", pc, c, i0, i1),
				Pcode::Arg1f(c, f) => println!("{} {:?} {}", pc, c, f),
			}			
		}
	}


	// test
	pub fn _test_compiler(&mut self) {
		self.scanner._test_scanner();
	}

}