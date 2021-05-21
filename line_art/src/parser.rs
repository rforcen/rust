// Parser
#![allow(dead_code)]


use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use crate::scanner::*;

#[derive(Debug, Clone, Copy)]
enum PCode {
    Pushc(f32),
    PushId, // k
    Plus,
    Minus,
    Mul,
    Div,
    Power,
    Neg,

    Sin,
    Cos,
    Tan,
    Log,
    Log10,
    Exp,
}
pub struct Parser<'a> {
    scanner: Scanner<'a>,
    sym: Token,
    error: bool,
    range_k: std::ops::Range<i32>,
    gr_type: Token, // Lines/Circle/Ellipse
    code: Vec<PCode>,
    xcode: Vec<PCode>,
    ycode: Vec<PCode>,
    x1code: Vec<PCode>,
    y1code: Vec<PCode>,
    rcode: Vec<PCode>,
}
impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> Self {
        Self {
            scanner: Scanner::new(expr),
            sym: Token::Null,
            error: false,
            range_k: std::ops::Range::<i32> { start: 0, end: 0 },
            gr_type: Token::Null,
            code: vec![],
            xcode: vec![],
            ycode: vec![],
            x1code: vec![],
            y1code: vec![],
            rcode: vec![],
        }
    }
    pub fn get_sym(&mut self) -> Token {
        self.sym = self.scanner.get_token();
        self.sym
    }
    // check and get next sym
    fn check_sym(&mut self, s: Token) {
        self.error = self.sym != s;
        self.get_sym();
    }
    fn get_check(&mut self, s: Token) {
        self.get_sym();
        self.error = self.sym != s;
    }
    fn test_sym(&mut self, s: Token) {
        self.error = self.sym != s;
    }
    pub fn get_sym_set(&mut self, token_set: &[Token]) {
        for t in token_set.into_iter() {
            if self.get_sym() != *t {
                self.error = true;
                break;
            }
        }
        self.get_sym();
    }
    pub fn compile(&mut self) -> bool {
        match self.get_sym() {
            Token::Lines => {
                self.get_check(Token::Comma);
                self.gr_type = Token::Lines;
                self.compile_lines()
            }
            Token::Circles => {
                self.get_check(Token::Comma);
                self.gr_type = Token::Circles;
                self.compile_circles()
            }
            _ => false,
        }
    }
    fn gen(&mut self, pcode: PCode) {
        self.code.push(pcode)
    }
    fn get_range(&mut self) {
        self.get_sym(); // range: start..end
        let neg = if self.sym == Token::Minus {
            self.get_sym();
            true
        } else {
            false
        };

        self.test_sym(Token::Number);
        let from = self.scanner.get_num() as i32;
        self.get_sym();
        self.test_sym(Token::Period);
        self.get_sym();
        self.test_sym(Token::Period);
        self.get_sym();
        self.test_sym(Token::Number);
        let to = self.scanner.get_num() as i32;
        self.range_k = std::ops::Range::<i32> {
            start: if neg { -from } else { from },
            end: to,
        };
        self.get_sym();
        self.test_sym(Token::Comma);
    }
    fn compile_lines(&mut self) -> bool {
        // from..to, (f1, t1),(f2,t2)
        self.get_range();
        self.get_sym();
        self.check_sym(Token::Oparen);

        self.exp0(); // f1
        self.xcode = self.code.clone();
        self.code.clear();
        self.check_sym(Token::Comma);

        self.exp0(); // t1
        self.ycode = self.code.clone();
        self.code.clear();
        self.check_sym(Token::Cparen);

        self.get_sym();
        self.check_sym(Token::Comma);
        self.exp0(); // f2
        self.x1code = self.code.clone();
        self.code.clear();

        self.check_sym(Token::Comma);

        self.exp0(); // t2
        self.y1code = self.code.clone();
        self.code.clear();
        self.check_sym(Token::Cparen);

        !self.error
    }

    fn compile_circles(&mut self) -> bool {
        static DEF_TOKENS: [Token; 5] = [
            Token::Id,
            Token::Oparen,
            Token::Id,
            Token::Cparen,
            Token::Equal,
        ];

        self.get_range();

        self.get_sym_set(&DEF_TOKENS); // X(k)=
        self.exp0();
        self.test_sym(Token::Comma);
        self.xcode = self.code.clone();
        self.code.clear();

        self.get_sym_set(&DEF_TOKENS); // Y(K)=
        self.exp0();
        self.test_sym(Token::Comma);
        self.ycode = self.code.clone();
        self.code.clear();

        self.get_sym_set(&DEF_TOKENS); // R(K)=
        self.exp0();
        self.rcode = self.code.clone();
        self.code.clear();
        !self.error
    }
    pub fn exp0(&mut self) {
        let is_neg = if self.sym == Token::Minus {
            self.get_sym();
            true
        } else {
            false
        };
        self.exp01();

        if is_neg {
            self.gen(PCode::Neg)
        }

        loop {
            match self.sym {
                Token::Plus => {
                    self.get_sym();
                    self.exp01();
                    self.gen(PCode::Plus);
                }
                Token::Minus => {
                    self.get_sym();
                    self.exp01();
                    self.gen(PCode::Minus);
                }
                _ => break,
            }
        }
    }
    fn exp01(&mut self) {
        fn implicit_mult(t: Token) -> bool {
            static IMPLICIT_MULT_SYM: [Token; 10] = [
                Token::Id,
                Token::Number,
                Token::Pi,
                Token::Oparen,
                Token::FuncSin,
                Token::FuncCos,
                Token::FuncTan,
                Token::FuncLog,
                Token::FuncLog10,
                Token::FuncExp,
            ];
            IMPLICIT_MULT_SYM.iter().any(|&i| i == t)
        }

        self.exp02();
        loop {
            if implicit_mult(self.sym) {
                self.exp02();
                self.gen(PCode::Mul)
            } else {
                match self.sym {
                    Token::Mult => {
                        self.get_sym();
                        self.exp02();
                        self.gen(PCode::Mul)
                    }
                    Token::Div => {
                        self.get_sym();
                        self.exp02();
                        self.gen(PCode::Div)
                    }
                    _ => break,
                }
            }
        }
    }
    fn exp02(&mut self) {
        self.exp03();
        if self.sym == Token::NumberSup {
            self.gen(PCode::Pushc(self.scanner.get_num()));
            self.gen(PCode::Power);
            self.get_sym();
        } else {
            loop {
                if self.sym == Token::Power {
                    self.get_sym();
                    self.exp03();
                    self.gen(PCode::Power);
                } else {
                    break;
                }
            }
        }
    }
    fn exp03(&mut self) {
        match self.sym {
            Token::Oparen => {
                self.get_sym();
                self.exp0();
                self.check_sym(Token::Cparen);
            }
            Token::Number | Token::NumberSup => {
                self.gen(PCode::Pushc(self.scanner.get_num()));
                self.get_sym();
            }
            Token::Pi => {
                self.gen(PCode::Pushc(PI));
                self.get_sym();
            }

            Token::Id => {
                // only id is 'k'
                self.gen(PCode::PushId);
                self.get_sym();
            }

            Token::Minus => {
                self.get_sym();
                self.exp03();
                self.gen(PCode::Neg);
            }

            Token::FuncSin => {
                self.get_sym();
                self.exp03();
                self.gen(PCode::Sin);
            }
            Token::FuncCos => {
                self.get_sym();
                self.exp03();
                self.gen(PCode::Cos);
            }
            Token::FuncTan => {
                self.get_sym();
                self.exp03();
                self.gen(PCode::Tan);
            }
            Token::FuncLog => {
                self.get_sym();
                self.exp03();
                self.gen(PCode::Log);
            }
            Token::FuncLog10 => {
                self.get_sym();
                self.exp03();
                self.gen(PCode::Log10);
            }
            Token::FuncExp => {
                self.get_sym();
                self.exp03();
                self.gen(PCode::Exp);
            }

            Token::Null => {}
            _ => self.error = true,
        }
    }

    // execute_circle on k -> x,y,r
    fn exec(&self, code: &Vec<PCode>, k: f32) -> f32 {
        let mut stack = vec![0.; 16];
        let mut sp = 0;

        for c in code {
            match c {
                PCode::Pushc(x) => {
                    stack[sp] = *x;
                    sp += 1
                }
                PCode::PushId => {
                    stack[sp] = k;
                    sp += 1
                }
                PCode::Plus => {
                    sp -= 1;
                    stack[sp - 1] += stack[sp]
                }
                PCode::Minus => {
                    sp -= 1;
                    stack[sp - 1] -= stack[sp]
                }
                PCode::Mul => {
                    sp -= 1;
                    stack[sp - 1] *= stack[sp]
                }
                PCode::Div => {
                    sp -= 1;
                    stack[sp - 1] /= stack[sp]
                }
                PCode::Power => {
                    sp -= 1;
                    stack[sp - 1] = stack[sp - 1].powf(stack[sp])
                }
                PCode::Neg => stack[sp - 1] = -stack[sp - 1],
                PCode::Sin => stack[sp - 1] = stack[sp - 1].sin(),
                PCode::Cos => stack[sp - 1] = stack[sp - 1].cos(),
                PCode::Tan => stack[sp - 1] = stack[sp - 1].tan(),
                PCode::Log => stack[sp - 1] = stack[sp - 1].ln(),
                PCode::Log10 => stack[sp - 1] = stack[sp - 1].log10(),
                PCode::Exp => stack[sp - 1] = stack[sp - 1].exp(),
            }
        }
        if sp == 1 {
            stack[0]
        } else {
            0.0
        }
    }
    fn execute_circle(&self, k: f32) -> (f32, f32, f32) {
        // circles
        (
            self.exec(&self.xcode, k),
            self.exec(&self.ycode, k),
            self.exec(&self.rcode, k),
        )
    }
    fn execute_line(&self, k: f32) -> (f32, f32, f32, f32) {
        (
            self.exec(&self.xcode, k),
            self.exec(&self.ycode, k),
            self.exec(&self.x1code, k),
            self.exec(&self.y1code, k),
        )
    }
    fn generate_lines(&self) -> Vec<(f32, f32, f32, f32)> {
        (self.range_k.start..self.range_k.end)
            .map(|k| self.execute_line(k as f32))
            .collect()
    }
    fn generate_circles(&self) -> Vec<(f32, f32, f32)> {
        (self.range_k.start..self.range_k.end)
            .map(|k| self.execute_circle(k as f32))
            .collect()
    }
    fn generate_lines_svg(
        &self,
        path: &str,
        width: f32,
        height: f32,
        scale_factor: f32, /* 3 or 6 */
        x_offset: f32,
        y_offset: f32,
    ) {
        let lines = self.generate_lines();

        let mut buff_write = BufWriter::new(File::create(path).unwrap());

        let scale = width / scale_factor;

        buff_write
            .write(
                &format!(
                    "
        <svg width='{w}' height='{h}' fill='none' stroke='blue' stroke-width='{sw}' >
\t<rect width='{w}' height='{h}' style='fill:white' />\n\n",
                    w = width,
                    h = height,
                    sw = 0.3
                )
                .as_bytes(),
            )
            .unwrap();

        for line in &lines {
            buff_write
                .write(
                    &format!(
                        "\t<line x1='{:.0}' y1='{:.0}' x2='{:.0}' y2='{:.0}'/>\n",
                        (line.0 + x_offset + 1.) * scale,
                        height / 2. - (1. + line.1 + y_offset) * scale,
                        (line.2 + x_offset + 1.) * scale,
                        height / 2. - (1. + line.3 + y_offset) * scale
                    )
                    .as_bytes(),
                )
                .unwrap();
        }

        buff_write.write(&format!("</svg>").as_bytes()).unwrap();

        buff_write.flush().unwrap();
    }

    pub fn generate_circles_svg(
        &self,
        path: &str,
        width: f32,
        height: f32,
        scale_factor: f32, /* 3 or 6 */
    ) {
        let circs = self.generate_circles();

        let mut buff_write = BufWriter::new(File::create(path).unwrap());

        let scale = width / scale_factor;

        buff_write
            .write(
                &format!(
                    "
        <svg width='{w}' height='{h}' fill='none' stroke='blue' stroke-width='{sw}' >
\t<rect width='{w}' height='{h}' style='fill:white' />\n\n",
                    w = width,
                    h = height,
                    sw = 0.3
                )
                .as_bytes(),
            )
            .unwrap();

        for circ in &circs {
            buff_write
                .write(
                    &format!(
                        "\t<circle cx='{:.0}' cy='{:.0}' r='{:.0}'/>\n",
                        circ.0 * scale + width / 2.,
                        height / 2. - circ.1 * scale,
                        circ.2 * scale
                    )
                    .as_bytes(),
                )
                .unwrap();
        }

        buff_write.write(&format!("</svg>").as_bytes()).unwrap();

        buff_write.flush().unwrap();
    }

    pub fn print_code(&self) {
        print!("x.code=");
        for c in &self.xcode {
            print!("{:?} ", c)
        }
        print!("\ny.code=");
        for c in &self.ycode {
            print!("{:?} ", c)
        }
        print!("\nx1.code=");
        for c in &self.x1code {
            print!("{:?} ", c)
        }
        print!("\ny1.code=");
        for c in &self.y1code {
            print!("{:?} ", c)
        }
        print!("\n\nr.code=");
        for c in &self.rcode {
            print!("{:?} ", c)
        }
        println!("")
    }

    pub fn generate_svg(
        &self,
        path: &str,
        width: f32,
        height: f32,
        scale_factor: f32, /* 3 or 6 */
        x_offset: f32,
        y_offset: f32,
    ) {
        match self.gr_type {
            Token::Lines => {
                self.generate_lines_svg(path, width, height, scale_factor, x_offset, y_offset)
            }
            Token::Circles => self.generate_circles_svg(path, width, height, scale_factor),
            _ => {}
        }
    }
}
