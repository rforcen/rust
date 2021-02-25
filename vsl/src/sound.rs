
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Instant, Duration};
use rodio::static_buffer::StaticSamplesBuffer;
use std::f32::{consts::PI, MAX};
use std::io::prelude::*;
use std::{fs, io};

extern crate rayon;
use rayon::prelude::*;

#[path = "compiler.rs"] mod compiler;
use compiler::*;

pub fn _play_file(file_name : String) {

    let vsl = fs::read_to_string(file_name).expect("file not found");
   
    let mut compiler = Compiler::new(&vsl[..]);
   
    if compiler.compile()  {
        // compiler.print_code();

        let sample_rate = compiler.params.sample_rate;
        let secs = compiler.params.seconds;

        let t = Instant::now();

        print!("generating {} samples...", sample_rate as f32 * secs); io::stdout().flush().unwrap();
    
        let samples = compiler.gen_samples();

        println!("done in {:?}", Instant::now()-t);
        
        _play_samples(samples, sample_rate, secs);
    } else {
        println!("syntax error");
    }
}

pub fn _play_samples(samples: Vec<f32>, sample_rate : u32, secs : f32 ) {

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    // scale
    let mut samples = samples.clone();
    let (mut min, mut max) = (samples[0], samples[0]);
    for s in &samples {
        min = min.min(*s);      max = max.max(*s);
    }
    let d = (max-min).abs();
    if d != 0. { for s in samples.iter_mut() { *s /= d }  }

    // play
    sink.set_volume(0.6);

    let samples = Box::leak(Box::new(samples));
    let b = StaticSamplesBuffer::new(1, sample_rate as u32, samples);
    sink.append(b);
    sink.play();
    
    thread::sleep(Duration::from_secs(secs as u64)); // in ms   
}

pub fn _write_samples(samples : &Vec<f32>, sample_rate : u32) {
    // write
    use hound::{WavSpec, WavWriter, SampleFormat};
    let spec = WavSpec {
        channels: 1,
        sample_rate: sample_rate,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let mut writer = WavWriter::create("default.wav", spec).unwrap();
    for s in samples { writer.write_sample(*s).unwrap() }
    writer.finalize().unwrap();  
    println!("generated default.wav file");  
}

pub fn _test_sound() {
    

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    let secs = 4000; // in ms
    let sample_rate = 44100;
     
    let generator = |i| { // f32 generator
        
        let t = i as f32 * 2. * PI / sample_rate as f32;

        let osc = |x : f32| (x * t).sin();
        let sin = | x: f32| x.sin();
        let sec = |s : f32| s * 2. * PI; 

        let phi = 1.61803398874989;
        let f0 = 440.;
        let f1 = f0 * phi;
        let f14 = f1/4.;
        let s3=sec(3.);
        let s2=sec(2.); 
        let k0=0.6;
        let k1=0.2;

        // ∿ = sin, ~ = wave
        // default: let ts3=τ+s3; ~f0  ∿( f0/ts3 + {k0,f1}  ∿( f1/ts3 + {k1, f14}  { f14/(τ+s2) } ) );

        let ts3 = t+s3;

        osc(f0) * sin( f0/ts3 + k0 * osc(f1) * sin( f1/ts3 + k1*osc(f14) * osc(f14/(t+s2)) ) )
    };

    // generate samples & min,max
    let min = Arc::new(Mutex::new(MAX));
    let max = Arc::new(Mutex::new(-MAX));

    let mut samples : Vec<f32> = (0..sample_rate * secs/1000).into_par_iter().map(
        |i| {
            let g = generator(i);
            if let Ok(mut min) = min.lock() { *min = min.min(g) }
            if let Ok(mut max) = max.lock() { *max = max.max(g) }
            g
        }
    ).collect();

    // scale
    let d = (*max.lock().unwrap() - *min.lock().unwrap()).abs();
    if d != 0. { for s in samples.iter_mut() { *s /= d } }
   
    

    // play
    sink.set_volume(0.3);

    let samples = Box::leak(Box::new(samples));
    let b = StaticSamplesBuffer::new(1, sample_rate as u32, samples);
    sink.append(b);
    sink.play();
    thread::sleep(Duration::from_millis(secs as u64)); // in ms

    _write_samples(samples, sample_rate);   
}