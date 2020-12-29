/*
	mandelbrot fractal
*/

use num::complex::*;
use rayon::prelude::*;
use std::convert::TryInto;
use image::{ImageBuffer, Rgb, Rgba};

pub type ComplexF32 = Complex<f32>;

const FIRE_PALETTE : [u32; 256] = [0, 0, 4, 12, 16, 24, 32, 36, 44, 48, 56, 64, 68, 76, 80, 88, 96, 100, 108, 116, 120, 128, 132,
	140, 148, 152, 160, 164, 172, 180, 184, 192, 200, 1224, 3272, 4300, 6348, 7376, 9424, 10448,
	12500, 14548, 15576, 17624, 18648, 20700, 21724, 23776, 25824, 26848, 28900, 29924, 31976,
	33000, 35048, 36076, 38124, 40176, 41200, 43248, 44276, 46324, 47352, 49400, 51452, 313596,
	837884, 1363196, 1887484, 2412796, 2937084, 3461372, 3986684, 4510972, 5036284, 5560572,
	6084860, 6610172, 7134460, 7659772, 8184060, 8708348, 9233660, 9757948, 10283260, 10807548,
	11331836, 11857148, 12381436, 12906748, 13431036, 13955324, 14480636, 15004924, 15530236,
	16054524, 16579836, 16317692, 16055548, 15793404, 15269116, 15006972, 14744828, 14220540,
	13958396, 13696252, 13171964, 12909820, 12647676, 12123388, 11861244, 11599100, 11074812,
	10812668, 10550524, 10288380, 9764092, 9501948, 9239804, 8715516, 8453372, 8191228, 7666940,
	7404796, 7142652, 6618364, 6356220, 6094076, 5569788, 5307644, 5045500, 4783356, 4259068,
	3996924, 3734780, 3210492, 2948348, 2686204, 2161916, 1899772, 1637628, 1113340, 851196,
	589052, 64764, 63740, 62716, 61692, 59644, 58620, 57596, 55548, 54524, 53500, 51452, 50428,
	49404, 47356, 46332, 45308, 43260, 42236, 41212, 40188, 38140, 37116, 36092, 34044, 33020,
	31996, 29948, 28924, 27900, 25852, 24828, 23804, 21756, 20732, 19708, 18684, 16636, 15612,
	14588, 12540, 11516, 10492, 8444, 7420, 6396, 4348, 3324, 2300, 252, 248, 244, 240, 236, 232,
	228, 224, 220, 216, 212, 208, 204, 200, 196, 192, 188, 184, 180, 176, 172, 168, 164, 160, 156,
	152, 148, 144, 140, 136, 132, 128, 124, 120, 116, 112, 108, 104, 100, 96, 92, 88, 84, 80, 76,
	72, 68, 64, 60, 56, 52, 48, 44, 40, 36, 32, 28, 24, 20, 16, 12, 8, 0, 0];


pub struct Mandelbrot {
	pub w : u32,
	pub h : u32,
	pub iters : u32,
	pub center: ComplexF32,
	pub range : ComplexF32,
	pub image : Vec<u32>,
}


impl Mandelbrot {
	pub fn new( w : u32, h: u32, center : ComplexF32, range : ComplexF32, iters : u32) -> Self {
		
		Self{w, h, iters, center, range, image: vec![]}
	}

	pub fn generate(&mut self)  {
		
		fn do_scale(cr : ComplexF32, range : ComplexF32, i  : u32, j : u32, w : u32, h : u32) -> ComplexF32 {
            cr + ComplexF32::new((range.im - range.re) * i as f32 / w as f32,
							  (range.im - range.re) * j as f32 / h as f32)
		}
		
		let pal_len = FIRE_PALETTE.len();
		let scale = 0.8_f32;
		let ratio = self.w as f32 / self.h as f32;
		let cr = ComplexF32::new(self.range.re, self.range.re);
		
		self.image = (0..self.w * self.h).into_par_iter().map (
			|index| {
				let (i, j) = (index % self.w,  index / self.w);
				
				let c0 = (scale * ratio) * do_scale(cr, self.range, i, j, self.w, self.h) - self.center;
				let mut z = c0;

				let mut ix : u32 = 0;
				while ix < self.iters {
					z = z * z + c0;  // z*z is the typical 2nd order fractal
					if z.norm() > 2.0 { break }

					ix+=1;
				}
				
				if ix >= self.iters { 0 } else { FIRE_PALETTE[(pal_len * ix as usize / 50) % pal_len] }
			}
		).collect();
	}

	pub fn get_pixel_rgb(&self, index : usize) -> [u8; 3] {
		(self.image[index].to_ne_bytes()[0..3]).try_into().expect("image should have u32 type!")
	}

	pub fn write_png(&self, name : &str) {
		let mut imgbuf = ImageBuffer::< Rgb<u8>, Vec<u8> >::new(self.w, self.h);
		
		for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
			*pixel = Rgb(self.get_pixel_rgb((y * self.w + x) as usize));
		}
		imgbuf.save(name).unwrap();
	}
}