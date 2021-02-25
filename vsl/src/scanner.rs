// parser
#![allow(dead_code, non_camel_case_types)]

use std::collections::HashMap;

type Pair<'a> = (&'a str, Symbol);
type SymHash<'a> = HashMap<&'a str, Symbol>;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Symbol {
    // list of symbols
    SNULL,
    CONST,
    LET,
    RPN,
    FUNC,
    RET,
    PARAM,
    ALGEBRAIC,
    NUMBER,
    IDENT,
    STRING,
    IDENT_t,
    PLUS,
    MINUS,
    MULT,
    DIV,
    OPAREN,
    CPAREN,
    OCURL,
    CCURL,
    OSQARE,
    CSQUARE,
    BACKSLASH,
    RANDOM,
    VERT_LINE,
    OLQUOTE,
    CLQUOTE,
    YINYANG,
    SEQUENCE,
    FREQ_MESH,
    FACT,
    TILDE,
    POWER,
    PERIOD,
    SEMICOLON,
    COMMA,
    COLON,
    EQ,
    GT,
    GE,
    LT,
    LE,
    NE,
    SPI,
    SPHI,
    FSIN,
    FCOS,
    FTAN,
    FEXP,
    FLOG,
    FLOG10,
    FINT,
    FSQRT,
    FASIN,
    FACOS,
    FATAN,
    FABS,
    SWAVE,
    SWAVE1,
    SWAVE2,
    TONE,
    NOTE,
    SEC,
    OSC,
    ABS,
    SAW,
    SAW1,
    LAP,
    HZ2OCT,
    MAGNETICRING,
    PUSH_CONST,
    PUSH_T,
    PUSH_ID,
    PUSH_STR,
    POP,
    NEG,
    FLOAT,
    RATE,
    NOTE_CONST,
    // notes
    N_DO,
    N_RE,
    N_MI,
    N_FA,
    N_SOL,
    N_LA,
    N_SI,
    FLAT,
    SHARP,
}


const CHARS 	: [Pair; 35] = [("+", Symbol::PLUS),     ("*", Symbol::MULT),      ("·", Symbol::MULT),
		("/", Symbol::DIV),     ("(", Symbol::OPAREN),    (")", Symbol::CPAREN),
		("{", Symbol::OCURL),   ("}", Symbol::CCURL),     ("[", Symbol::OSQARE),
		("]", Symbol::CSQUARE), ("\\", Symbol::BACKSLASH), ("?", Symbol::RANDOM),
		("!", Symbol::FACT),    ("^", Symbol::POWER),     (".", Symbol::PERIOD),
		(",", Symbol::COMMA),   (":", Symbol::COLON),     (";", Symbol::SEMICOLON),
		("=", Symbol::EQ),      ("~", Symbol::TILDE),     ("π", Symbol::SPI),
		("Ø", Symbol::SPHI),    ("|", Symbol::VERT_LINE), ("‹", Symbol::OLQUOTE),
		("›", Symbol::CLQUOTE), ("♪", Symbol::NOTE),      ("⬳", Symbol::SAW),
		("∿", Symbol::FSIN),    ("τ", Symbol::IDENT_t),   ("☯", Symbol::YINYANG),
		("§", Symbol::SEQUENCE),("✬", Symbol::FREQ_MESH), ("➡", Symbol::RET),
		("♭", Symbol::FLAT),    ("♯", Symbol::SHARP)];

const WORDS 	: [Pair; 34] = [("sin", Symbol::FSIN),     ("cos", Symbol::FCOS),
		("tan", Symbol::FTAN),     ("exp", Symbol::FEXP),
		("log", Symbol::FLOG),     ("log10", Symbol::FLOG10),
		("int", Symbol::FINT),     ("sqrt", Symbol::FSQRT),
		("asin", Symbol::FASIN),   ("acos", Symbol::FACOS),
		("atan", Symbol::FATAN),   ("abs", Symbol::FABS),
		("pi", Symbol::SPI),       ("phi", Symbol::SPHI),
		("wave", Symbol::SWAVE),   ("wave1", Symbol::SWAVE1),
		("wave2", Symbol::SWAVE2), ("tone", Symbol::TONE),
		("note", Symbol::NOTE),    ("sec", Symbol::SEC),
		("osc", Symbol::OSC),      ("saw", Symbol::SAW),
		("saw1", Symbol::SAW1),    ("lap", Symbol::LAP),
		("hz2oct", Symbol::HZ2OCT), ("magneticring", Symbol::MAGNETICRING),
		("rate", Symbol::RATE),

		("t", Symbol::IDENT_t),    ("const", Symbol::CONST),
		("rpn", Symbol::RPN),      ("algebraic", Symbol::ALGEBRAIC),
		("let", Symbol::LET),      ("float", Symbol::FLOAT),
		("func", Symbol::FUNC)];

const TWO_CH 	: [Pair; 4] = [(">=", Symbol::GE), ("<=", Symbol::LE), ("<>", Symbol::NE), ("->", Symbol::RET)];

const INITIAL 	: [Pair; 3] = [("-", Symbol::MINUS), (">", Symbol::GT), ("<", Symbol::LT)];
const NOTES 	: [Pair; 7] = [("do", Symbol::N_DO), ("re", Symbol::N_RE),   ("mi", Symbol::N_MI),
		 ("fa", Symbol::N_FA), ("so", Symbol::N_SOL), ("la", Symbol::N_LA),
		 ("si", Symbol::N_SI)];

#[derive(Clone, Debug)]			
pub struct Scanner <'a> {
	source 		: String,
	reserved 	: SymHash<'a>,
	ident		: String,
	nval		: f32,
	pub sym		: Symbol,
	ch			: char,
	ixs			: usize,
	line_no		: usize,
}

impl<'a> Scanner <'a> {
	pub fn new(source : String) -> Self {
		Self { source, 
			reserved : Self::consts_to_hash(), 
			ident : String::default(), nval : 0.0, sym : Symbol::SNULL, ch : ' ', ixs : 0, line_no : 0 }
	}

	pub fn get_id(&self) -> String { self.ident.clone() }
	pub fn get_num(&self) -> f32 { self.nval }

	pub fn get_error_msg(&self) -> String {
		format!("error in line: {}, position: {}, near char: {}", self.line_no+1, self.ixs+1, self.ch)
	}

	fn getch(&mut self) -> char {
		if let Some(ch) = self.source.chars().nth(self.ixs) {
			self.ch = ch;
			self.ixs+=1;
			if ch=='\n' || ch=='\r' { self.line_no+=1 }
		} else {
			self.ch='\0';
		}
		self.ch
	}

	fn ungetch(&mut self) {
		if self.ixs > 0 {
			self.ixs -= 1;
			self.ch = self.source.chars().nth(self.ixs-1).unwrap();
		}
	}

	fn skip_blanks(&mut self) {
		while self.ch!='\0' && self.ch <= ' ' { self.getch(); }
	  }
  
	fn skip_to_eol(&mut self) {
		while self.ch!='\0' && (self.ch != '\n' || self.ch == '\r') { self.getch(); }  // skip line comment
	}
  
	fn skip_multiline_comment(&mut self) {
		while self.ch != '/' {
		  self.getch();
		  while self.ch != '\0' && self.ch != '*' { self.getch(); }
		  self.getch();
		}
		self.getch();  // skip last '/'
	  }

	fn skip_blank_comments(&mut self) {  // skip blanks & comments // /**/
		loop {
		  self.skip_blanks();
  
		  if self.ch == '/' {  // skip comment
			if self.getch() == '/' { self.skip_to_eol() }
			else { 
				if self.ch == '*' {  // /**/
			  		self.skip_multiline_comment();
				} else {
			  		self.ungetch();
			  		break
				}
			}
		  } else {	break }
		}

		self.skip_blanks();
	}

	fn index_sym(&mut self) { // sym = reserved[ident]
		self.sym = *self.reserved.get(&self.ident[..]).unwrap()
	}
	
	pub fn getsym(&mut self) -> Symbol {
		self.sym = Symbol::SNULL;
		self.ident.clear();
		self.nval = 0_f32;

		self.skip_blank_comments();

		// scan symbol
		if self.ch.is_alphabetic() { // ident
			while self.ch.is_alphanumeric() || self.ch=='_' {
				self.ident.push(self.ch);
				self.getch();
			}
			
			if self.ident == "t" { self.sym=Symbol::IDENT_t }
			else { // func ?
				if self.is_reserved_word(&self.ident) {
					self.index_sym()
				} else { // ident
					self.sym = Symbol::IDENT
				}				
			}
		} else if self.ch.is_digit(10) || self.ch=='.' { // number
			while self.ch.is_digit(10) || self.ch=='.' || self.ch=='e' || self.ch=='E' {
				self.ident.push(self.ch);
				self.getch();
			}
			self.sym = Symbol::NUMBER;
			self.nval = self.ident.parse::<f32>().unwrap(); // atof
		} else {
			self.ident.push(self.ch);

			if self.is_reserved_word(&self.ident) {	self.index_sym() }  // 1 ch sym
			else { 
				self.getch();
				self.ident.push(self.ch);

				if self.is_reserved_word(&self.ident) { // 2 ch sym
					self.index_sym()
				} else {
					self.sym = Symbol::SNULL
				}
			}				
				
			self.getch();
		}
		self.sym
	}

	fn consts_to_hash() -> SymHash<'a> { // convert CHARS, WORDS, TWO_CH to hashmap
		let mut hm = CHARS.to_vec().into_iter().collect::<SymHash>(); 
		hm.extend(WORDS.to_vec().into_iter().collect::<SymHash>());
		hm.extend(TWO_CH.to_vec().into_iter().collect::<SymHash>());
		hm.extend(INITIAL.to_vec().into_iter().collect::<SymHash>());
		hm
	}

	fn is_reserved_word(&self, w : &'a str) -> bool {
		self.reserved.contains_key(w)
	}


	// test 	
	pub fn _test_reserved(&self) {
		for r in &self.reserved {
			println!("{:?} {}", r, self.is_reserved_word(r.0));
			assert_eq!(self.is_reserved_word(r.0), true)
		}
	}

	pub fn _test_scanner(&mut self) {
		println!("input string: {}, {}", self.source.len(), self.source);

		while self.getsym() != Symbol::SNULL {
			print!("{:?} ", self.sym);
		}
		println!("\n------");
	}

}