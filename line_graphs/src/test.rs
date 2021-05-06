// tests
#![allow(unused_imports)]

// mod line_art;
use crate::examples::*;
use crate::parser::*;
use crate::scanner::*;
use crate::support::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn is_numeric_supercript() {
        assert!(!"⁰¹²³⁴⁵⁶⁷⁸⁹"
            .chars()
            .map(|ch| ch.is_numeric_supercript())
            .collect::<Vec<_>>()
            .iter()
            .any(|&x| x.is_none()))
    }
    #[test]
    fn scan_numsup() {
        let exp_res = vec![8, 6, 12, 2, 5, 12, 2, 4];
        let mut res = vec![];
        let mut scan = Scanner::new(OLIVE_BRANCH);
        loop {
            let t = scan.get_token();
            if t == Token::NumberSup {
                println!("{:?} {}", t, scan.get_num());
                res.push(scan.get_num() as usize);
            }
            if t == Token::Null {
                break;
            }
        }
        println!("{:?}", res);
        assert!(!res.is_empty());
        assert_eq!(
            exp_res.iter().zip(&res).filter(|&(a, b)| a == b).count(),
            res.len()
        )
    }
    #[test]
    fn scan_complete() {
        let mut scan = Scanner::new(OLIVE_BRANCH);
        loop {
            let t = scan.get_token();
            match t {
                Token::Number | Token::NumberSup => println!("{:?} {}", t, scan.get_num()),
                Token::Id => println!("{:?} {}", t, scan.get_id()),
                Token::Null => break,
                _ => println!("{:?}", t),
            }
        }
    }
    #[test]
    pub fn scan_all() {
        let mut scan = Scanner::new("r rr2 123⁰ ⁰⁰¹()²+-/*π=");
        let mut res = vec![];
        loop {
            let t = scan.get_token();
            res.push(t.clone());
            if t == Token::Null {
                break;
            }
        }

        let exp_res = vec![
            Token::Id,
            Token::Id,
            Token::Number,
            Token::Number,
            Token::NumberSup,
            Token::NumberSup,
            Token::Oparen,
            Token::Cparen,
            Token::NumberSup,
            Token::Plus,
            Token::Minus,
            Token::Div,
            Token::Mult,
            Token::Pi,
            Token::Equal,
            Token::Null,
        ];
        assert!(!res.is_empty());
        assert_eq!(
            exp_res.iter().zip(&res).filter(|&(a, b)| a == b).count(),
            res.len()
        )
    }

    #[test]
    pub fn parse_f01() {
        let mut parser = Parser::new(TEST_EXP);
        assert!(parser.compile());
        parser.print_code()
    }

    #[test]
    fn test_svg() {
        // <svg height="600" width="600" xmlns="http://www.w3.org/2000/svg">
        // <circle cx="130" cy="100" fill="transparent" r="50" stroke="black" stroke-width="3"/>
        //</svg>
        let mut buff_write = BufWriter::new(File::create("image.svg").unwrap());

        let (width, height) = (1920, 1080);
        buff_write
            .write(
                &format!(
                    "<svg width='{w}' height='{h}' fill='white'>
\t<rect width='{w}' height='{h}' style='fill:white' />\n\n",
                    w = width,
                    h = height
                )
                .as_bytes(),
            )
            .unwrap();

        for r in (10..width / 4).rev().step_by(3) {
            buff_write
            .write(
                &format!(
                    "\t<circle cx='{}' cy='{}' r='{}' stroke='black' stroke-width='{}' fill='none'/>\n",
                    width/2, height/2, r, 0.3
                )
                .as_bytes(),
            )
            .unwrap();
        }

        buff_write.write(&format!("</svg>").as_bytes()).unwrap();

        buff_write.flush().unwrap();
    }

    #[test]
    fn test_svg_generateion() {
        let mut parser = Parser::new(BUTTERFLY1);
        assert!(parser.compile());
        parser.generate_svg("butterfly1.svg", 2000., 2000., 3., 0., 0.)
    }

    #[test]
    fn gen_bird_circles() {
        let mut parser = Parser::new(BIRD_CIRC);
        assert!(parser.compile());
        parser.generate_svg("bird_circ.svg", 2000., 2000., 6., 0., 0.)
    }

    #[test]
    fn read_from_file() {
        let file_la = "la/butterfly1.la";
        if let Ok(expr) = read_file(file_la) {
            let mut parser = Parser::new(&*expr);
            assert!(parser.compile());
            parser.generate_svg("svg/butterfly1.svg", 2000., 2000., 3., 0., 0.)
        } else {
            panic!("can't read input file: {}", file_la)
        }
    }

    use std::{
        env::current_dir,
        fs::{create_dir, read_dir, read_to_string},
    };
    #[test]
    fn generate_all_svg() {
        // traverse home/la folder
        let cwd = current_dir().unwrap();

        let mut la_dir = cwd.clone();
        let mut svg_dir = cwd.clone();
        la_dir.push("la");
        svg_dir.push("svg");

        let _ = create_dir(svg_dir.clone());

        for entry in read_dir(la_dir).unwrap() {
            let la_file = entry.unwrap().path();
            let mut svg = svg_dir.clone();
            svg.push(la_file.file_stem().unwrap().to_str().unwrap());
            svg.set_extension("svg");
            let buff = read_to_string(&la_file).unwrap();
            println!("{:?}->{:?}\n{}", la_file, svg, buff);
            let mut la_compiler = Parser::new(&*buff);
            assert!(la_compiler.compile());
            la_compiler.generate_svg(svg.to_str().unwrap(), 2000., 2000., 4., 0., 0.)
        }
    }
}
