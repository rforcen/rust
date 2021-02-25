// vsl.rs

use std::io::prelude::*;
use std::{fs, io};
use scoped_pool::{Pool};
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use std::thread;
use rodio::static_buffer::StaticSamplesBuffer;
use num_cpus;


#[path = "compiler.rs"] mod compiler;
use compiler::*;

pub struct Vsl<'a> {
	compiler 	: Compiler<'a>,
	samples		: Vec<f32>,
}

impl<'a> Vsl<'a> {
	pub fn from_file(file_name : &str) -> Self {
		let vsl_source = fs::read_to_string(file_name).expect(&format!("file {} not found", file_name)[..]);
   
		let compiler =  Compiler::new(vsl_source);
		let mut vsl = Vsl { compiler : compiler, samples : vec![] };
	   
		if vsl.compiler.compile()  {
	
			let sample_rate = vsl.compiler.params.sample_rate;
			let secs = vsl.compiler.params.seconds;
	
			let t = Instant::now();
	
			print!("syntax ok, generating {} samples...", sample_rate as f32 * secs); io::stdout().flush().unwrap();
		
			vsl.gen_samples_mt();
	
			println!("done in {:.1?}", Instant::now()-t);
		} else {
			println!("syntax error, {}", vsl.compiler.error_message());
		}
		vsl
	}

	pub fn gen_samples_mt(&mut self) { // multi threaded version of gen_samples -> # cores times faster

		fn from_to_size(th : usize, nth : usize, size : usize) -> (usize, usize, usize) { // from, to, size
			let from = th * size / nth;
			let to = if th == nth-1 { size } else { (th+1) * size / nth };
			(from, to, to - from)
		}

		self.compiler.exec_const();

		let nth 	= num_cpus::get();
		let t_inc 	= self.compiler.t_inc();
		let n_samps = self.compiler.num_samples();
		let n_chan 	= self.compiler.chan as usize;

		let mut _samples = Arc::new(Mutex::new( vec![0.; self.compiler.samples_size()] ));

		let pool = Pool::new(nth);
		pool.scoped( |scope| {
			
			for th in 0..nth {
				let (from, to, size) = from_to_size(th, nth, n_samps);

				let _samples = Arc::clone(&_samples);
				let mut this = self.compiler.clone();

				scope.execute(move || { 
					
					let mut samp : Vec<f32> = vec![];
					samp.reserve_exact(size * n_chan);

					for sn in from..to {
						samp.extend( this.execute(sn as f32 * t_inc) );						
					}
					// threads are executed in random order so copy into 'th' region of _samples
					_samples.lock().unwrap() [ from*n_chan .. to*n_chan ].copy_from_slice(&samp[..])
				})
			}
		});
		pool.shutdown();

		self.samples = vec![0.; self.compiler.samples_size()]; // make a copy of result
		self.samples.copy_from_slice(&*_samples.lock().unwrap());
	}

	pub fn _gen_samples(&mut self) { // single threaded mode
		self.samples = vec![];
		let size = self.compiler.samples_size();
		self.samples.reserve_exact(size);

		self.compiler.exec_const();

		for i in 0..self.compiler.num_samples() {
			let t = i as f32 * self.compiler.t_inc();
			self.compiler.exec_let(t);

			for ch in 0..self.compiler.chan as usize {
				self.samples.push( self.compiler.exec_chan(t, ch) )
			}
		}
	}

	pub fn play(&mut self) {
		if !self.compiler.err {
			let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
			let sink = rodio::Sink::try_new(&stream_handle).unwrap();
		
			// scale
			let (mut min, mut max) = (self.samples[0], self.samples[0]);
			for s in &self.samples {
				min = min.min(*s);      max = max.max(*s);
			}
			let d = (max-min).abs();
			if d != 0. { for s in self.samples.iter_mut() { *s /= d }  }
		
			// play
			sink.set_volume(self.compiler.params.volume);
		
			let samples = Box::leak(Box::new(self.samples.clone()));
			let b = StaticSamplesBuffer::new(self.compiler.chan as u16, self.compiler.params.sample_rate as u32, samples);
			sink.append(b);
			sink.play();
			
			thread::sleep(Duration::from_secs(self.compiler.params.seconds as u64)); // in ms   
		}
	}
}