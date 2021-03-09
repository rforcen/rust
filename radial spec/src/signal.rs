// signal.rs
// plot using : cargo build; target/debug/radialspec | gnuplot -p > fft.png; open fft.png

use hound::SampleFormat;
extern crate rgsl; 
use rgsl::types::interpolation::{Spline, InterpType};
use rgsl::fft::radix2::forward;


#[path="music_freq.rs"] mod music_freq;

// plot using : cargo build; target/debug/radialspec | gnuplot -p > fft.png; open fft.png

const OCT_RANGE: std::ops::RangeInclusive<i8> = -3..=3;

pub struct Signal {
    samples         : Vec<f32>,
    channels        : usize,
    sample_rate     : usize,
    n_fft           : usize,
    err_msg         : String,
    top_freq        : f32,
	smooth_spec		: (Vec<f32>,Vec<f32>),
    pub musical_matrix  : Vec<Vec<f32>>,
}

impl Signal {

    pub fn new() -> Self { Self { 
		samples:vec![], channels:0, sample_rate:0, 
		n_fft:0, top_freq:0., err_msg:String::default(), 
		smooth_spec:(vec![], vec![]),
        musical_matrix: vec![]} }

    pub fn set_top_freq(&mut self, top_freq:f32) { self.top_freq = top_freq }

   
        
    fn nearest_pow2(n : usize) -> usize {
        let mut p = 1;
        while p <= n { p <<= 1 }
        p>>1
    }

    fn index_2_freq(&self, i : usize) -> f32    { i as f32 * self.sample_rate as f32 / self.n_fft as f32 }

    fn freq_2_index(&self, freq : f32) -> usize { (freq / (self.sample_rate as f32 / self.n_fft as f32)) as usize }

    fn db(val : f32, range : f32) -> f32 { if val == 0.0 { -80.0 } else { 20. * (val / range).log10() }  }

    fn max_freq(&self, v : &Vec<f32>) -> f32 {
        let index_of_max = v
            .iter()
            .enumerate()
            .fold((0, 0.0), |max, (ind, &val)| if val > max.1 {(ind, val)} else {max});
        self.index_2_freq(index_of_max.0) 
    }

    fn remove_mic_noise(&self, y_fft : &mut Vec<f32>) {
        let mic_threshold = 8.; // in hz freq range to remove from 50hz
        let mic_range = 50.-mic_threshold .. 50.+mic_threshold;

        let _ = y_fft.iter_mut() // remove mic noise ~50hz and scale
            .enumerate()
            .map(|(i, x)| 
                if mic_range.contains(&self.index_2_freq(i)) { *x/30. } else { *x }  )
            .collect::<Vec<f32>>();
    }

    fn scale_01(v : &mut Vec<f32>) {
        let max = v.iter().fold(-f32::MAX, |max, x| x.max(max) ); // scale 0..1
        v.iter_mut().for_each(|i| *i /= max);
    }

    fn range_freq(&self) -> Vec<f32> {
        (0..self.freq_2_index(self.top_freq))
                    .map(|i| self.index_2_freq(i))
                    .collect() // x-axis freq series
    }

    pub fn spec_string(&self) -> String {
        format!("channels: {}\nsample rate: {}\n# samples: {}", self.channels, self.sample_rate, self.samples.len())
    }
    
    fn gnuplot(&self, coords : Vec<(f32, f32)>) {
        // generate gnuplot 
        println!("  set terminal png size 4096, 1280

                set ylabel \"scaled fft\"
                set xlabel \"frequency in Hz\"

                set yrange [0:1]

                plot \'-\' with lines", 
        );
        for c in coords { println!("{} {}", c.0, c.1) }
        println!("e")
    }

    fn peaks(vy : &Vec<f32>) -> Vec<usize> {
        let mut peaks = vec![];
        for i in 1..vy.len()-1 {
            if vy[i-1] < vy[i] && vy[i+1] < vy[i] { peaks.push(i) }
        }
        peaks
    }
    fn peaks_coords(vy : &Vec<f32>) -> (Vec<usize>, Vec<f32>) {
        let mut y = vec![];
        let mut x = vec![];

        for i in 1..vy.len()-1 {
            if vy[i-1] < vy[i] && vy[i+1] < vy[i] { 
                x.push(i);
                y.push(vy[i]);
             }
        }
        (x, y)
    }
    fn holes(vy : &Vec<f32>) -> Vec<usize> {
        let mut holes = vec![];
        for i in 1..vy.len()-1 {
            if vy[i-1] > vy[i] && vy[i+1] > vy[i] { holes.push(i) }
        }
        holes
    }

    fn count_peaks(vy : &Vec<f32>) -> usize {
        let mut peaks = 0;
        for i in 1..vy.len()-1 {
            if vy[i-1] < vy[i] && vy[i+1] < vy[i] { peaks+=1 }
        }
        peaks
    }

    fn smooth(vy : &mut Vec<f32>) {
        for i in 1..vy.len()-1 {
            if vy[i-1] > vy[i] && vy[i+1] > vy[i] { 
                let min = vy[i-1].min(vy[i+1]);
                vy[i]=min + (vy[i-1] - vy[i+1]).abs()/2. 
            }
        }
    }
    fn smooth_holes(vy : &mut Vec<f32>) {
        let holes = Self::peaks(vy);
        for i in 1..holes.len()-1 {            
            if vy[holes[i-1]] > vy[holes[i]] && vy[holes[i+1]] > vy[holes[i]] { 
                let min = vy[holes[i-1]].min(vy[holes[i+1]]);
                vy[holes[i]]=min + (vy[holes[i-1]] - vy[holes[i+1]]).abs()/2. 
            }
        }
    }

    
    fn samps_2_complex(&self, samples:&Vec<f32>) -> Vec<f64>  { // (samps[i], 0)
        let rng = 0..samples.len()*2;

        match self.channels {
            1 => rng.map(|i| if i&1==0 { samples[i/2] as f64 } else {0.} ).collect(),
            2 => rng.step_by(2).map(|i| if i & 1==0 { ((samples[i/2] + samples[(i+1)/2])/2.) as f64} else {0.}).collect(),        
            _ => rng.step_by(self.channels)
                    .map(|i|   if i & 1==0 { ((i..i+self.channels).fold(0., |s, i| s+samples[i/2]) / self.channels as f32) as f64 } else {0.} )
                    .collect()
        }
    }

    
    fn calc_fft(&self, samps : &Vec<f32>) -> Vec<f32> {
        fn norm(x:f64, y:f64) -> f32 { (x*x + y*y).sqrt() as f32}

        let mut fft_data = self.samps_2_complex(samps); // create a (samples, 0) vec of n_fft*2 size
        fft_data.resize(self.n_fft*2, 0.);
        forward(&mut fft_data[..], 1, self.n_fft); // do fft & normalize (abs) back to vec<f32>
        (0..fft_data.len()).step_by(2).map(|i| norm(fft_data[i], fft_data[i+1])).collect()
    }

    fn max_fft_signal(&mut self) -> f32 { // max fft value of a 1hz sin signal
        use std::f32::consts::PI;
        let t_inc = 2. * PI / self.sample_rate as f32;

        let samples = (0..self.n_fft).map(|x| (x as f32 * t_inc).sin() ).collect::<Vec<f32>>().to_vec();

        let y_fft = self.calc_fft(&samples);
        y_fft.iter().fold(-f32::MAX, |max, x| max.max(*x))
    }

    fn musical_matrix(&mut self, fft : &Vec<f32>) { // create from fft

        self.musical_matrix = vec![vec![0_f32; 12]; OCT_RANGE.len()]; // -3..=3 x 12 notes

        let mut max=0_f32;
        fft.iter()
            .enumerate()
            .for_each(|(i,x)| {
                let (oct, note) = music_freq::freq2oct_note(self.index_2_freq(i) as f64);
                if OCT_RANGE.contains(&oct) {
                    self.musical_matrix[(oct-OCT_RANGE.start()) as usize][note as usize] += x;
                    max = max.max(self.musical_matrix[(oct-OCT_RANGE.start()) as usize][note as usize]);
                }
            });
        
        // scale
        self.musical_matrix.iter_mut().for_each(|ov| ov.iter_mut().for_each(|x| *x= *x / max ) );
        // self.musical_matrix.iter().for_each(|m|  println!("{:.2?}", m));
    }
    
    pub fn smooth_spec(&mut self) -> (Vec<f32>,Vec<f32>) {
		self.smooth_spec.0.clear();
		self.smooth_spec.1.clear();

        self.n_fft=4096;
        let max_fft = self.max_fft_signal();

        let mut fft_sum = vec![0_f32; self.n_fft]; // accumulated fft

        for samples in self.samples.chunks(self.n_fft * self.channels) {
            let y_fft = self.calc_fft(&samples.to_vec());
            for (i, y) in y_fft.iter().enumerate() { fft_sum[i]+=y } // accumulate fft's
        }

        fft_sum.iter_mut().for_each(|x| *x /= max_fft); // scale to max_fft
        
        self.remove_mic_noise(&mut fft_sum);
        self.musical_matrix(&fft_sum);
        
        // generate interpolation polynomial from peaks
        let mut acc = rgsl::types::interpolation::InterpAccel::new();
        
        let coords = Self::peaks_coords(&fft_sum); // all peaks, now get only to self.top_freq index

        let xa : Vec<f64> = coords.0.iter()
            .filter(|&&x| x < self.freq_2_index(self.top_freq))
            .map(|x| self.index_2_freq(*x) as f64).collect(); // to vec of freqs in top_freq range
        let ya : Vec<f64>= coords.1[0..xa.len()].iter().map(|x| *x as f64).collect(); // convert to Vec<f64>
        
        let mut spline = Spline::new(InterpType::cspline(), xa.len()).unwrap();

        if spline.init(&xa, &ya) == rgsl::Value::Success {

            let n_points = 500;
            let coords : Vec<(f32,f32)>= (0..n_points).map(|i| {
                let x = xa[0] + i as f64 * ((*xa.last().unwrap() - xa[0]) / n_points as f64);
                (x as f32, spline.eval(x, &mut acc) as f32)
            }).collect();

			self.smooth_spec.0 = coords.iter().map(|x| x.0).collect();
			self.smooth_spec.1 = coords.iter().map(|x| x.1).collect();
        }      
		self.smooth_spec.clone()
    }

    pub fn read_wav(&mut self, file_name : &String) -> bool {
        let _reader = match hound::WavReader::open(file_name) {
            Ok(reader) => {
                self.sample_rate = reader.spec().sample_rate as usize;
                self.channels = reader.spec().channels as usize;
            
                self.samples = match reader.spec().sample_format {
                    SampleFormat::Int   => { 
                        let idiv = match reader.spec().bits_per_sample {
                            8 =>        0x7f_u32,
                            16 =>      0x7fff_u32,
                            24 =>   0x7f_ffff_u32,
                            32 => 0x7fff_ffff_u32,
                            _  => 1,
                        } as f32;
                        reader.into_samples::<i32>().map(|s| if let Ok(s) = s {s as f32 / idiv} else {0.0}).collect() 
                    },
                    SampleFormat::Float => { reader.into_samples::<f32>().map(|s| s.unwrap()).collect() },
                };
                return true
            },
            Err(e) => { 
                self.err_msg = format!("Error: {:?}", e);
                return false 
            }
        };
    }
}
