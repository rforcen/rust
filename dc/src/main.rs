/*
	Domain Coloring
*/

#![allow(dead_code)]
#![allow(unused_imports)]

use std::time::Instant;
use image::{ImageBuffer, Rgb, Rgba};
	
mod dc;

const PREDEF_FUNCS : [&str; 19] = [
	"acos(c(1,2)*log(sin(z^3-1)/z))", 			"c(1,1)*log(sin(z^3-1)/z)", 					"c(1,1)*sin(z)",
	"z + z^2/sin(z^4-1)", 						"log(sin(z))", 									"cos(z)/(sin(z^4-1))", 
	"z^6-1",									"(z^2-1) * (z-c(2,1))^2 / (z^2+c(2,1))", 		"sin(z)*c(1,2)", 
	"sin(1/z)", "sin(z)*sin(1/z)",				"1/sin(1/sin(z))", "z", "(z^2+1)/(z^2-1)", 		"(z^2+1)/z", "(z+3)*(z+1)^2",
	"(z/2)^2*(z+c(1,2))*(z+c(2,2))/z^3", 		"(z^2)-0.75-c(0,0.2)",							"z * sin( c(1,1)/cos(3/z) + tan(1/z+1) )"];

fn test_dc() {	

	let m = 2;
	let (w, h, func)=(1024 * m, 768 * m, PREDEF_FUNCS[18]);

	let mut dc = dc::DomainColoring::new(w, h, func);

	println!("Domain coloring for {}, size: {} x {} = {} pixels", func, w, h, w*h);

	let t = Instant::now();		dc.generate_singleth();	 	println!("single thrd    : {:?}", Instant::now() - t);
	let t = Instant::now(); 	dc.generate_parallel();	 	println!("parallel       : {:?}", Instant::now() - t);
	
	dc.write_png("dc.png");		
}
 

fn main() {
	test_dc();
}