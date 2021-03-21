// lib.rs

use std::io::Write;                                                                                                                                                                  
use std::fs::File;   

pub fn write_bin(file_name : &str, image : &[u8]) {
	File::create(file_name).expect("Unable to create file")
	.write_all(image).expect("Unable to write data"); // showbinimage.py w w mandelbrot.bin     
}