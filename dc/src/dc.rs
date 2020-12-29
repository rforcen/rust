/*
	Domain Coloring
*/
#![allow(dead_code)]

use num::complex::Complex as complex;
use std::f32::consts::{PI, E};
use rayon::prelude::*;
use std::convert::TryInto;
use image::{ImageBuffer, Rgb, Rgba};

#[path = "zvm.rs"] mod zvm;
use zvm::*;

const PI2 : f32 = PI * 2.0;

pub type Cf32 = complex<f32>;
pub type ZFN = fn(Cf32) -> Cf32;

pub struct DomainColoring {
	pub w : u32,
	pub h : u32,
	pub image : Vec<u32>,
	pub zvm : ZVm,
}


impl DomainColoring {
	pub fn new(w : u32, h : u32, zexpr : &str) -> Self {
		Self{ w, h, image: vec![], zvm : ZVm::new(zexpr) }
	}
			
	pub fn get_pixel_rgb(&self, index : usize) -> [u8; 3] {
		// let px = self.image[index];
		// [((px & 0x00ff_0000) >> 16) as u8, ((px & 0x0000_ff00) >> 8) as u8, ((px & 0x0000_00ff)) as u8]
		(self.image[index].to_be_bytes()[0..3]).try_into().expect("image should have u32 type!")
	}

	pub fn get_pixel_rgba(&self, index : usize ) -> [u8; 4] {
		self.image[index].to_be_bytes()
	}

	pub fn get_size(&self) -> usize { (self.w*self.h) as usize }
	
	fn pow3(x:f32) -> f32 { x * x * x }

	// todo: https://www.vagrearg.org/content/hsvrgb
	fn hsv_2_rgb(_h:f32,  s:f32,  v:f32) -> u32 { // convert hsv to int with alpha 0xff00000
		let (mut r, mut g, mut b, mut h) = (0_f32, 0_f32, 0_f32, _h);
		if s == 0_f32 { r = v; g = v; b = v }
		else {
			if h == 1_f32 { h = 0_f32 }
			let z : f32 = (h * 6_f32).floor();
			let i : i32 = z as i32;
			let f : f32 = h * 6_f32 - z;
			let (p, q, t) = (v * (1_f32 - s) as f32, v * (1_f32 - s * f) as f32, v * (1_f32 - s * (1_f32 - f)) as f32);

			match i {
				0 =>  {  r = v;  g = t;   b = p; }
				1 =>  {  r = q;  g = v;   b = p; }
				2 =>  {  r = p;  g = v;   b = t; }
				3 =>  {  r = p;  g = q;   b = v; }
				4 =>  {  r = t;  g = p;   b = v; }
				5 =>  {  r = v;  g = p;   b = q; }
				_ =>  {}
			}
		}
		u32::from_ne_bytes([0xff_u8, (r * 255_f32) as u8, (g * 255_f32) as u8, (b * 255_f32) as u8])
		// 0xff00_0000  | (((r * 255_f32) as u32) << 16) | (((g * 255_f32) as u32) << 8) | ((b * 255_f32) as u32)
	}
	
	fn gen_pixel(zvm : &ZVm, index : usize, w : usize, h:usize) -> u32 {
			
		let limit = PI;
		let (rmi, rma, imi, ima) = (-limit, limit, -limit, limit);

		let (i, j) = (index % w,  index / w);
				
		let im = ima - (ima - imi) * j as f32 / (h - 1) as f32;
		let re = rma - (rma - rmi) * i as f32 / (w - 1) as f32;

		let v = zvm.eval(Cf32::new(re, im));

		let mut hue = v.arg(); // calc hue, arg->phase -pi..pi
		if hue < 0.0_f32 { hue += PI2 }   
		hue /= PI2;

		let (m, mut ranges,  mut rangee) = (v.norm(),  0_f32, 1_f32);
		while m > rangee {
			ranges = rangee;
			rangee *= E;
		}
		let k : f32 = (m - ranges) / (rangee - ranges);
		let kk : f32;
		if k < 0.5_f32  { kk = k * 2.0_f32 }
		else 		    { kk = 1.0_f32 - (k - 0.5_f32) * 2.0_f32 }

		let sat : f32 = 0.4_f32 + (1.0_f32 - Self::pow3(1. - kk)) * 0.6_f32;
		let val : f32 = 0.6_f32 + (1.0_f32 - Self::pow3(1. - (1. - kk))) * 0.4_f32;

		Self::hsv_2_rgb(hue, sat, val)				
	}

	pub fn write_png(&self, name : &str) {
		let mut imgbuf = ImageBuffer::< Rgb<u8>, Vec<u8> >::new(self.w, self.h);
		
		for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
			*pixel = Rgb(self.get_pixel_rgb((y * self.w + x) as usize));
		}
		imgbuf.save(name).unwrap();
	}
		
	
	pub fn generate_parallel(&mut self) {
		let (w, h, zvm, size) = (self.w as usize, self.h as usize, self.zvm.clone(), (self.w * self.h) as usize);

		self.image = (0..size).into_par_iter().map(
			|index| Self::gen_pixel(&zvm, index, w, h)
		).collect();
	}

	pub fn generate_singleth(&mut self) {
		let (w, h, zvm, size) = (self.w as usize, self.h as usize, self.zvm.clone(), (self.w * self.h) as usize);

		self.image = (0..size).into_iter().map(
			|index| Self::gen_pixel(&zvm, index, w, h)
		).collect();
	}
		
	
}
