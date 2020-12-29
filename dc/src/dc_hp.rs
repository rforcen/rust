/*
	high performance Domain Coloring w/fastvect
*/
#![allow(dead_code)]

use num::complex::Complex as complex;
use std::f32::consts::{PI, E};
use rayon::prelude::*;
use rayon::iter::plumbing::*;
use std::sync::Arc;
use std::sync::Mutex;
use num_cpus;

#[path = "fastvect.rs"] mod fastvect;

pub type Cf32 = complex<f32>;


pub struct DomainColoring_HP {
	pub w : u32,
	pub h : u32,
	pub image : fastvect::FastVect<u32>,
}

/* impl ParallelIterator for DomainColoring_HP {
    type Item = fastvect::FastVect<u32>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where   C: UnindexedConsumer<Self::Item>,
    {
        let DomainColoring_HP { w, h, image } = self;
		// DomainColoring::gen_pixel(fz, index, w, h)
		(0..w*h).into_par_iter().map(|i| DomainColoring_HP::gen_pixel(fz, index, w, h)).drive_unindexed(consumer)
    }
} */

impl DomainColoring_HP {
	pub fn new(w : u32, h:u32) -> Self {
		Self{ w, h, image: fastvect::FastVect::<u32>::new((w * h) as usize) }
	}
		
	pub fn get_pixel_u8vec(&self, index : usize) -> [u8; 3] {
		let px = self.image[index];
		[((px & 0x00ff_0000) >> 16) as u8, ((px & 0x0000_ff00) >> 8) as u8, ((px & 0x0000_00ff)) as u8]
	}

	pub fn get_size(&self) -> usize { (self.w*self.h) as usize }
	
	fn pow3(x:f32) -> f32 { x * x * x }

	fn hsv_2_rgb(_h:f32,  s:f32,  v:f32) -> u32 { // convert hsv to int with alpha 0xff00000
		let (mut r, mut g, mut b, mut h) = (0_f32, 0_f32, 0_f32, _h as f32);
		if s == 0. { r = v; g = v; b = v }
		else {
			if h == 1. { h = 0. }
			let z : f32= (h * 6_f32).floor();
			let i = z as i32;
			let f : f32 = h * 6_f32 - z;
			let (p, q, t) = (v * (1_f32 - s) as f32, v * (1_f32 - s * f) as f32, v * (1_f32 - s * (1. - f)) as f32);

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
		0xff00_0000  | (((r * 255_f32) as u32) << 16) | (((g * 255_f32) as u32) << 8) | ((b * 255_f32) as u32)
	}
	
	pub fn generate(&mut self, fz: fn(Cf32) -> Cf32) {
		
		let (pi2, limit) = (PI * 2., PI);
		let (rmi, rma, imi, ima) = (-limit, limit, -limit, limit);
		
		let mut index : usize = 0;

		for th in 0..self.h {
			let im = ima - (ima - imi) * th as f32 / (self.h - 1) as f32;

			for i in 0..self.w {
				let re = rma - (rma - rmi) * i as f32 / (self.w - 1) as f32;

				let v = fz(Cf32::new(re, im));

				let mut hue = v.arg(); // calc hue, arg->phase -pi..pi
				if hue < 0.0 { hue += pi2 }   
				hue /= pi2;

				let (m, mut ranges,  mut rangee) = (v.norm(),  0.0, 1.0);
				while m > rangee {
					ranges = rangee;
					rangee *= E;
				}
				let k = (m - ranges) / (rangee - ranges);
				let kk : f32;
				if k < 0.5  { kk = k * 2. }
				else 		{ kk = 1. - (k - 0.5) * 2. }

                let sat = 0.4 + (1. - Self::pow3(1. - kk)) * 0.6;
                let val = 0.6 + (1. - Self::pow3(1. - (1. - kk))) * 0.4;

				// let hsv = Hsv::new(RgbHue::from_radians(hue), sat, val);
				
				self.image[index] = Self::hsv_2_rgb(hue, sat, val);
				index+=1;
			}
		}

	}

	

	fn gen_pixel(fz: fn(Cf32) -> Cf32, index : usize, w : usize, h : usize) -> u32 {
		
	
		let (pi2, limit) = (PI * 2., PI);
		let (rmi, rma, imi, ima) = (-limit, limit, -limit, limit);

		let (i, th) = (index % w,  index / w);
				
		let im = ima - (ima - imi) * th as f32 / (h - 1) as f32;
		let re = rma - (rma - rmi) * i as f32 / (w - 1) as f32;

		let v = fz(Cf32::new(re, im));

		let mut hue = v.arg(); // calc hue, arg->phase -pi..pi
		if hue < 0.0 { hue += pi2 }   
		hue /= pi2;

		let (m, mut ranges,  mut rangee) = (v.norm(),  0_f32, 1_f32);
		while m > rangee {
			ranges = rangee;
			rangee *= E;
		}
		let k : f32 = (m - ranges) / (rangee - ranges);
		let kk : f32;
		if k < 0.5  { kk = k * 2. }
		else 		{ kk = 1. - (k - 0.5) * 2. }

		let sat : f32 = 0.4 + (1. - Self::pow3(1. - kk)) * 0.6;
		let val : f32 = 0.6 + (1. - Self::pow3(1. - (1. - kk))) * 0.4;

		Self::hsv_2_rgb(hue, sat, val)				
	}

	
	pub fn generate_parallel_org(&mut self, fz: fn(Cf32) -> Cf32) { // does not improve dc.rs but similar timing
		let (w, h) = (self.w as usize, self.h as usize);
				
		let vr : Vec<fastvect::FastVect::<u32>> = (0..h).into_par_iter().map(
			|th| {
				let mut image = fastvect::FastVect::<u32>::new(w);
				let thw = th * w;
				for i in 0..w {
					image[i] = Self::gen_pixel(fz, thw + i, w, h);
				}
				image
			}
		).collect();

		for (th, v) in vr.iter().enumerate() { // copy to self.image
			let thw = th * w;
			for i in 0..w { self.image[thw + i] = v[i] }
		}

	}

	pub fn generate_parallel_pi(&mut self, fz: fn(Cf32) -> Cf32) { // worst than map
		let (w, h) = (self.w as usize, self.h as usize);

		let image = Arc::new(Mutex::new(self.image)).clone();  
		
		(0..h).into_par_iter().for_each(
			|th| {		
				let mut image = image.lock().unwrap(); // very expensive...

				for i in th*w .. th*(1+w) {
					image[i] = Self::gen_pixel(fz, i, w, h);
				}				
			}
		);

	}
	pub fn generate_parallel_ncpu(&mut self, fz: fn(Cf32) -> Cf32) { // worst than map
		let (w, h) = (self.w as usize, self.h as usize);

		let image = Arc::new(Mutex::new(self.image)).clone();  

		let nths =  num_cpus::get();
		let d = self.image.len()/nths;
		let sz = self.image.len();
		
		(0..nths).into_par_iter().for_each(
			|th| {		
				let mut image = image.lock().unwrap(); // very expensive...

				let from = th*d;
				let to = {
					let ni = (th+1)*d;
					if ni > sz { sz }
					else { ni  }
				};
				
				for i in from..to {
					image[i] = Self::gen_pixel(fz, i, w, h);
				}				
			}
		);

	}
	
}


/* 

mod dc_hp;

fn write_png_hp(name : &str, d : &dc_hp::DomainColoring_HP) {
	let mut imgbuf = ImageBuffer::< Rgb<u8>, Vec<u8> >::new(d.w, d.h);
	
	for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
		*pixel = Rgb(d.get_pixel_u8vec((y * d.w + x) as usize));
	}
	imgbuf.save(name).unwrap();
}

fn test_dc_hp() {
	
	let m = 2;
	let (w, h)=(1024 * m, 768 * m);

	let mut dc = dc_hp::DomainColoring_HP::new(w, h);

	println!("Domain coloring HP {} x {}", w, h);

	let fnz = dcz02;

	let t = Instant::now();		dc.generate(fnz);			 		println!("forloop      : {:?}", Instant::now() - t);
	let t = Instant::now(); 	dc.generate_parallel_org(fnz);		println!("parallel org : {:?}", Instant::now() - t);
	let t = Instant::now(); 	dc.generate_parallel_pi(fnz);	 	println!("parallel pi  : {:?}", Instant::now() - t);
	let t = Instant::now(); 	dc.generate_parallel_ncpu(fnz);	 	println!("parallel ncpu: {:?}", Instant::now() - t);
	
	write_png_hp("dc_hp.png", &dc);		
}
 */