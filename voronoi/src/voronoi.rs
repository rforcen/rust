#[path = "point.rs"]mod point;
use point::*;

use rayon::prelude::*;
use image::{ImageBuffer, Rgb};
use std::convert::TryInto;

#[derive(Debug)]
pub struct Voronoi {
	w : u32,
	h : u32,
	n_points : u32,
	points  : Vec<Point>,
	image	: Vec<u32>,
}

impl Voronoi {
	pub fn new(w : u32, h: u32, n_points : u32) -> Self {
		fn rnd_u32(d : u32) -> u32 { rand::random::<u32>() % d }

		let points = (0..n_points).into_par_iter().map(
			|_| Point::new(rnd_u32(w), rnd_u32(h), rnd_u32(0xffff_ffff) ) 
		).collect();

		
		Voronoi { w, h, n_points, points, image: vec![] }
	}

	pub fn generate(&mut self)  {
		fn distance_sqr(x:u32, y:u32, p:&Point) -> u32 {
			fn sqr(x:u32) -> u32 { x * x }
			sqr(x - p.x) + sqr(y - p.y)
		}
		self.image = (0 .. self.w * self.h).into_par_iter().map(
			|index| {
				let (i, j) = (index % self.w, index / self.w);
				let mut ind		 :usize = 0;
				let mut is_center:bool  = false;

				let mut dist = distance_sqr(i, j, &self.points[0]);

				for it in 1..self.points.len() {
					let d = distance_sqr(i, j, &self.points[it]);

					if d < 4 { is_center=true; break }
					
					if d < dist { 
						dist = d;
						ind = it;
					}
				}
				if is_center {0} else { self.points[ind].color }
			}
		).collect()
	}

	pub fn write_png(&self, name : &str) {
		pub fn u32_to_rgb(px : u32) -> [u8; 3] {
			(px.to_be_bytes()[0..3]).try_into().expect("image should have u32 type!")
		}
		let mut imgbuf = ImageBuffer::< Rgb<u8>, Vec<u8> >::new(self.w, self.h);
		
		for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
			*pixel = Rgb(u32_to_rgb(self.image[(y * self.w + x) as usize]));
		}
		imgbuf.save(name).unwrap();
	}
	
}