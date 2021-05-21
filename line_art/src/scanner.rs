// scanner
#![allow(dead_code)]


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Null,

    Id,
    Number,
    NumberSup,
    Oparen,
    Cparen,
    Plus,
    Minus,
    Mult,
    Div,
    Power,
    Equal,
    Pi,
    Comma,
    Period,

    Lines,
    Circles,
    Ellipses,

    FuncSin, // funcs start at sin
    FuncCos,
    FuncTan,
    FuncLog,
    FuncLog10,
    FuncExp,
}

pub struct Scanner<'a> {
    ss: std::str::Chars<'a>,
    num: f32,
    id: String,
    ch: char,
}
impl<'a> Scanner<'a> {
    pub fn new(expr: &'a str) -> Self {
        let mut s = Self {
            ss: expr.chars(),
            num: 0.,
            id: String::new(),
            ch: 0 as char,
        };
        s.get_ch();
        s
    }
    pub fn get_num(&self) -> f32 {
        self.num
    }
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    fn get_ch(&mut self) -> char {
        self.ch = if let Some(c) = self.ss.next() {
            c
        } else {
            0 as char
        };
        self.ch
    }
    pub fn get_token(&mut self) -> Token {
        let mut ch = self.ch;

        // skip white's
        while ch.is_ascii_whitespace() {
            ch = self.get_ch();
        }

        match ch {
            _ch if _ch.is_ascii_digit() => {
                let mut snum = String::default();
                while ch.is_ascii_digit() {
                    //} || ch == '.' || ch == 'e' || ch == 'E' {
                    snum.push(ch);
                    ch = self.get_ch();
                }
                self.num = if let Ok(n) = snum.parse::<f32>() {
                    n
                } else {
                    0.
                };
                Token::Number
            }
            _ch if _ch.is_numeric_supercript().is_some() => {
                self.num = 0.;
                while let Some(n) = ch.is_numeric_supercript() {
                    self.num = self.num * 10. + n as f32;
                    ch = self.get_ch();
                }
                Token::NumberSup
            }
            _ch if _ch.is_ascii_alphabetic() => {
                if ch.is_ascii_alphabetic() {
                    self.id = String::default();
                    while ch.is_alphabetic() {
                        self.id.push(ch);
                        ch = self.get_ch();
                    }
                }
                // func | reserved word | Id

                match &*self.id {
                    "sin" => Token::FuncSin,
                    "cos" => Token::FuncCos,
                    "tan" => Token::FuncTan,
                    "log" => Token::FuncLog,
                    "log10" => Token::FuncLog10,
                    "exp" => Token::FuncExp,

                    "lines" => Token::Lines,
                    "circles" => Token::Circles,
                    "ellipses" => Token::Ellipses,

                    _ => Token::Id,
                }
            }
            _ => {
                self.get_ch();
                match ch {
                    '(' => Token::Oparen,
                    ')' => Token::Cparen,
                    '+' => Token::Plus,
                    '−' | '-' => Token::Minus,
                    '*' | '∗' => Token::Mult,
                    '/' => Token::Div,
                    '=' => Token::Equal,
                    'π' => Token::Pi,
                    '^' => Token::Power,
                    ',' => Token::Comma,
                    '.' => Token::Period,
                    _ => Token::Null,
                }
            }
        }
    }
}

// ch extensions
#[allow(non_camel_case_types)]
pub trait char_ext {
    fn is_numeric_supercript(&self) -> Option<usize>;
}
impl char_ext for char {
    fn is_numeric_supercript(&self) -> Option<usize> {
        static BIX: [usize; 10] = [0, 3, 5, 7, 9, 12, 15, 18, 21, 24];
        match "⁰¹²³⁴⁵⁶⁷⁸⁹".find(*self) {
            Some(ix) => BIX.iter().position(|&x| x == ix),
            None => None,
        }
    }
}
