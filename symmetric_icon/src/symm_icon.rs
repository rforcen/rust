// Symmetric Icons

#![allow(dead_code)]

use array2d::*;

// lambda, alpha, beta, gamma, omega, symmetry, scale
const PRESETS: [[f32; 7]; 36] = [
    [1.56, -1., 0.1, -0.82, -0.3, 3., 1.7],    	[-1.806, 1.806, 0., 1.5, 0., 7., 1.1],
    [2.4, -2.5, -0.9, 0.9, 0., 3., 1.5],    	[-2.7, 5., 1.5, 1., 0., 4., 1.],
    [-2.5, 8., -0.7, 1., 0., 5., 0.8],    		[-1.9, 1.806, -0.85, 1.8, 0., 7., 1.2],
    [2.409, -2.5, 0., 0.9, 0., 4., 1.4],    	[-1.806, 1.807, -0.07, 1.08, 0., 6., 1.2],
    [-2.34, 2.2, 0.4, 0.05, 0., 5., 1.2],    	[-2.57, 3.2, 1.2, -1.75, 0., 36., 1.2],
    [-2.6, 4., 1.5, 1., 0., 12., 1.1],		    [-2.2, 2.3, 0.55, -0.90, 0., 3., 1.3],
    [-2.205, 6.01, 13.5814, -0.2044, 0.011, 5., 0.8],
    [-2.7, 8.7, 13.86, -0.13, -0.18, 18., 0.8],  [-2.52, 8.75, 12., 0.04, 0.18, 5., 0.8],
    [2.38, -4.18, 19.99, -0.69, 0.095, 17., 1.], [2.33, -8.22, -6.07, -0.52, 0.16, 4., 0.8],
    [-1.62, 2.049, 1.422, 1.96, 0.56, 6., 1.],   [-1.89, 9.62, 1.95, 0.51, 0.21, 3., 0.6],
    [-1.65, 9.99, 1.57, 1.46, -0.55, 3., 0.8],   [-2.7, 5., 1.5, 1., 0., 6., 1.],
    [-2.08, 1., -0.1, 0.167, 0., 7., 1.3],	    [1.56, -1., 0.1, -0.82, 0.12, 3., 1.6],
    [-1.806, 1.806, 0., 1., 0., 5., 1.1],	    [1.56, -1., 0.1, -0.82, 0., 3., 1.3],
    [-2.195, 10., -12., 1., 0., 3., 0.7],	    [-1.86, 2., 0., 1., 0.1, 4., 1.2],
    [-2.34, 2., 0.2, 0.1, 0., 5., 1.2],		    [2.6, -2., 0., 0.5, 0., 5., 1.3],
    [-2.5, 5., -1.9, 1., 0.188, 5., 1.],		[2.409, -2.5, 0., 0.9, 0., 23., 1.2],
    [2.409, -2.5, -0.2, 0.81, 0., 24., 1.2],	[-2.05, 3., -16.79, 1., 0., 9., 1.],
    [-2.32, 2.32, 0., 0.75, 0., 5., 1.2],	    [2.5, -2.5, 0., 0.9, 0., 3., 1.3],
    [1.5, -1., 0.1, -0.805, 0., 3., 1.4],
];

const MAX_XY : f32 = 1e5;
const DEFAULT_SPEED : u32 = 100;
const MAX_COLORS : u32 = 2111;
const COLOR_SPEED : u32 = 3071;

pub struct SymmetricIcons {
  lambda 	: f32,
  alpha		: f32, 
  beta		: f32,
  gamma		: f32,
  omega		: f32,
  symmetry	: u32,
  scale		: f32,

  w         : usize,
  h		    : usize,
  color_set : u32,
  iter		: u32,
  speed		: u32,

  apcx		: f32,
  apcy		: f32,
  rad		: f32,

  color_list: Vec<u32>,
  icon      : Array2D<u32>,
  image	    : Array2D<u32>,

  x			: f32,
  y			: f32,

  k			: u32,
}


impl SymmetricIcons {
	pub fn new(w : usize, h : usize, color_set : u32) -> Self { 
		let mut s = Self {
			lambda 		: 0.0,
			alpha		: 0.0, 
			beta		: 0.0,
			gamma		: 0.0,
			omega		: 0.0,
			symmetry	: 0,
			scale		: 0.0,

			w        	: w,
			h			: h,
            color_set   : color_set,
			iter		: 0,
		
			speed		: DEFAULT_SPEED,
			apcx		: 0.0,
			apcy		: 0.0,
			rad			: 0.0,
            
            color_list	: vec![],
			icon      	: Array2D::filled_with(0_u32, w, h),
			image		: Array2D::filled_with(0_u32, w, h),

			x			: 0.0,
			y			: 0.0,
			k			: 0,
		};
		s.set_preset(0);
		s
	}

	pub fn set_size(&mut self, w : usize, h : usize) {
		self.w = w;
		self.h = h;
		self.image = Array2D::filled_with(0_u32, w, h); 
		self.icon = Array2D::filled_with(0_u32, w, h); 
		self.iter = 0;
		
		self.color_list = vec![];
		
		self.reset();
	}

	pub fn set_preset(&mut self, i : usize) {
		let p = PRESETS[i % PRESETS.len()];

		self.lambda = p[0];
        self.alpha = p[1];
        self.beta = p[2];
        self.gamma = p[3];
        self.omega = p[4];
        self.symmetry = p[5] as u32;
        self.scale = if p[6] == 0. {1.} else {p[6]};

        self.reset();
	}

    pub fn set_parameters(&mut self, lambda : f32, alpha: f32, beta : f32, gamma : f32, omega : f32, symmetry : f32, scale : f32) {
        self.lambda = lambda;
        self.alpha = alpha;
        self.beta = beta;
        self.gamma = gamma;
        self.omega = omega;

        self.symmetry = if symmetry < 1. { 1 } else { symmetry as u32 };
        self.scale = if scale == 0. {1.} else { scale };

        self.reset();
    }

	fn make_color(r : u32, g : u32, b : u32) -> u32 { (b << 16) | (g << 8) | r | 0xff00_0000 } 
    fn make_colora(a : u32, r : u32, g : u32, b : u32) -> u32 { (a << 24) | (b << 16) | (g << 8) | r   }

	fn get_rainbow(x : u32, y : u32) -> u32 {
        match x {
            0 => Self::make_color(0, y, 255),
            1 => Self::make_color(0, 255, 255 - y),
            2 => Self::make_color(y, 255, 0),
            3 => Self::make_color(255, 255 - y, 0),
            4 => Self::make_color(255, 0, y),
            5 => Self::make_color(255 - y, 0, 255),
            _ => Self::make_color(0,0,0), // black
        }
    }

	fn set_colors(&mut self, param_int : u32) {
        let mut colors = vec![0_u32; (MAX_COLORS+1) as usize];

        match param_int {
        0 => {
            for i in 0..64 { colors[i] = Self::make_color(0, 0, 4 * i as u32) }
            for i in 0..256 {
                let local_color = Self::make_color(255, i, 255);
                for j in 0..3 { colors[(1344 + j + 3 * i) as usize] = local_color }                
            }
        }
        1 => {
            for i in 0..64 { colors[i] = Self::make_color(0, 4 * i as u32, 4 * i as u32) }
            for i in 0..256  {
                    let local_color = Self::make_color(i, i, 255);
                    for j in 0..3 { colors[(1344 + j + 3 * i) as usize] = local_color }
                }
            }
        2 => {
            for i in 0..64 { colors[i] = Self::make_color(0, 4 * i as u32, 0) }
            for i in 0..256 {
                let local_color = Self::make_color(i, 255, 255);
                for j in 0..3 { colors[(1344 + j + 3 * i) as usize] = local_color }
            }
        }
        3 => {
            for i in 0..64 { colors[i] = Self::make_color(4 * i as u32, 4 * i as u32, 0) }
            for i in 0..256 {
                let local_color = Self::make_color(i, 255, i);
                for j in 0..3 { colors[(1344 + j + 3 * i) as usize] = local_color }
            }
        }
        4 => {
            for i in 0..64 { colors[i] = Self::make_color(4 * i as u32, 0, 0) }
            for i in 0..256 {
                let local_color = Self::make_color(255, 255, i);
                for j in 0..3 { colors[(1344 + j + 3 * i) as usize] = local_color }
            }
        }   
        5 => {
            for i in 0..64 { colors[i] = Self::make_color(4 * i as u32, 0, 4 * i as u32) }
            for i in 0..256 {
                let local_color = Self::make_color(255, i, i);
                for j in 0..3 { colors[(1344 + j + 3 * i) as usize] = local_color }         
            }
        }      
       
        6 =>  for i in 0..256 { colors[(i + 64)] = Self::make_colora(255, 255 - i as u32, 255 - i as u32, 255) },
        7 =>  for i in 0..256 { colors[(i + 64)] = Self::make_color(255 - i as u32, 255, 255) },
        8 =>  for i in 0..256 { colors[(i + 64)] = Self::make_color(255 - i as u32, 255, 255 - i as u32) },
        9 =>  for i in 0..256 { colors[(i + 64)] = Self::make_color(255, 255, 255 - i as u32) },
        10 => for i in 0..256 { colors[(i + 64)] = Self::make_color(255, 255 - i as u32, 255 - i as u32)} ,
        11 => for i in 0..256 { colors[(i + 64)] = Self::make_color(255, 255 - i as u32, 255)},
        
        _  => ()
        }
        
        if param_int > 5 {
            for i in 0..64 { colors[i] = Self::make_color(4 * i as u32, 4 * i as u32, 4 * i as u32) }
            for j in 0..5 {
                for i in 0..256 {
                    colors[(320 + j * 256 + i)] = Self::get_rainbow((param_int + j as u32) % 6, i as u32)
                }
            }
            for i in 0..256 {
                let local_color = Self::get_rainbow((param_int - 1) % 6, i);
                colors[(1600 + 2 * i as usize)] = local_color;
                colors[(1601 + 2 * i as usize)] = local_color;
            }
        } else { // <= 5
            for j in 0..5 {
                for i in 0..256 {
                    colors[64 + j * 256 + i] = Self::get_rainbow((param_int + j as u32) % 6, i as u32);
                }
            }
        }
		
        self.color_list = colors
    }

	

	fn reset(&mut self) {
        self.speed = DEFAULT_SPEED;

        self.apcx = self.w as f32 / 2.;
        self.apcy = self.h as f32 / 2.;
        self.rad = if self.apcx > self.apcy {self.apcy} else {self.apcx};
        self.k = 0;
        self.x = 0.01;
        self.y = 0.003;
        self.iter = 0;

        self.icon = Array2D::filled_with(0_u32, self.w, self.h);
        self.image = Array2D::filled_with(0_u32, self.w, self.h);
        
        self.set_colors(self.color_set);

        for m in 0..self.w {
            for n in 0..self.h {
				let color = self.get_color(self.icon[(m, n)]);
				self.set_point_color(m, n, color);
			}
		}        
    }

	fn set_point_color(&mut self, x : usize, y : usize, color : u32) {
        self.image[(x, y)] = color;
    }

    fn get_color(&mut self, col : u32) -> u32 {
        let col = col & 0x00ffffff;
        if col * self.speed > MAX_COLORS {
            while (col * self.speed > COLOR_SPEED) && (self.speed > 3) { self.speed-=1 }
            self.color_list[MAX_COLORS as usize]
        } else {
        	self.color_list[(col * self.speed) as usize]
		}
    }

	fn set_point(&mut self, x : usize, y : usize) {
        let icon = self.icon[(x,y)];

        let color = self.get_color(icon);
        self.image[(x,y)] = color;
        self.icon[(x,y)] += 1;
        if icon >= 12288 { self.icon[(x,y)] = 8192 }
    }
    

	pub fn generate(&mut self, mod_disp : u32) -> bool { // geenrate icon, runs in a thread in 'start'
		self.iter+=1;

		
		if self.x.abs() > MAX_XY || self.y.abs() > MAX_XY {
			self.reset(); // prevent overflow
		}

		// generate new x,y
		let sq = self.x * self.x + self.y * self.y; // sq=x^2+y^2

		let mut tx = self.x;
		let mut ty = self.y; // tx=pow, ty=pow

		for _m in 1..self.symmetry - 2 + 1 {
			let sqx = tx * self.x - ty * self.y;
			let sqy = ty * self.x + tx * self.y;
			tx = sqx;
			ty = sqy;
		}

		let sqx = self.x * tx - self.y * ty;
		let tmp = self.lambda + self.alpha * sq + self.beta * sqx;
		let x_new = tmp * self.x +self.gamma * tx - self.omega * self.y;
		let y_new = tmp * self.y - self.gamma * ty + self.omega * self.x;

		self.x = x_new;
		self.y = y_new;

		if self.k > 50 {
			self.set_point((self.apcx + self.x * self.rad / self.scale) as usize,
				           (self.apcy + self.y * self.rad / self.scale) as usize);
		} else {
			self.k += 1;
		}

		self.iter % mod_disp == 0
	}

    pub fn build(&mut self, preset : usize, n_iters : usize) -> (&[u8], (usize, usize)) {
        self.set_preset(preset);
        for _ in 0..n_iters { self.generate(1); }
        ( self.get_image(), self.get_size() )
    }
    pub fn get_size(&self) -> (usize, usize) {
        ( self.w, self.h )
    }

    pub fn write(&self, name : &str) { // showbinimage.py 800 800 symm_icon.bin
        use std::fs::File;
        use std::io::prelude::*;

        File::create(name).expect("create failed")
                .write_all(self.get_image()).expect("write failed");
    }

    pub fn get_image(&self) -> &[u8] { // convert Vec<u32> to [u8]
        let v = self.image.as_row_major();
        unsafe {
            std::slice::from_raw_parts(
                v.as_ptr() as *const u8,
                v.len() * std::mem::size_of::<u32>(),
            )
        }
    }
}


pub fn _test_symmetric_icon() {
    let n = 2048;
    let mut symicn = SymmetricIcons::new(n, n, 0);
    symicn.set_preset(9);

    for _i in 0..900_000 {
        symicn.generate(5000);
    }
    symicn.write("symm_icon.bin");

    use std::process::Command;
    let n = &n.to_string()[..];
    Command::new("/usr/local/bin/showbinimage.py")
        .args(&[n, n, "symm_icon.bin"])
        .output().expect("can't execute command");
}

pub fn _test_array2d() { // Array2D is faster 1.67 vs 2.14 (1.3 times faster)
    use std::time::Instant;

    const N : usize = 100_000_000;
    const SZ : usize = 1200;
    let v = vec![0_usize; SZ*SZ];
    let a = Array2D::filled_with(0_usize, SZ, SZ);

    let t = Instant::now();
    for _ in 0..N {
        for i in 0..SZ {
            for j in 0..SZ {
                let crd = i*SZ+j;
                let x = v[crd];
                let _xx = x+1;
            }
        }
    }
    println!("lap vec     : {:?}", Instant::now()-t);

    let t = Instant::now();

    for _ in 0..N {
        for r in 0..SZ {
            for c in 0..SZ {
                let x = a[(r,c)];
                let _xx = x+1;
            }
        }
    }
    println!("lap array2d : {:?}", Instant::now()-t);

}