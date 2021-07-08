// fastsin
use std::f64::consts::PI;

pub struct FastSin {
    y0: f64,
    y1: f64,
    y2: f64,
    p: f64,

    w: f64,
    b: f64,
    a: f64,
    x: f64,
    n: i32,
}

impl FastSin {
    // a: amp, w:freq, b:phase
    pub fn new(a: f64, w: f64, b: f64) -> Self {
        Self {
            a,
            w,
            b,
            x: 0.,
            y0: (-2. * w + b).sin(),
            y1: (-w + b).sin(),
            y2: 0.,
            p: 2.0 * w.cos(),
            n: -1,
        }
    }
    pub fn with_sample_rate(mut self, rate: f64) -> Self {
        Self::new(self.a, Self::freq2inc(self.w, rate), self.b)
    }
    pub fn next(&mut self) -> f64 {
        self.n += 1;
        self.x = self.n as f64 * self.w;
        self.y2 = self.p * self.y1 - self.y0;
        self.y0 = self.y1;
        self.y1 = self.y2;
        self.a * self.y2 // mutl by amp.
    }
    pub fn sin(&mut self, x: f64) -> f64 {
        self.n += 1;
        self.x = self.n as f64 * self.w;
        self.a * (self.w * self.n as f64 + self.b + x).sin()
    }

    pub fn freq2inc(freq: f64, samp: f64) -> f64 {
        freq * 2. * PI / samp
    }
}

pub mod test {
    use super::*;
    #[test]
    fn basic_wave() {
        let mut fs = FastSin::new(1., 440., 0.).with_sample_rate(44100.);
        for i in 0..440 {
            println!("{}", fs.next())
        }
    }
    use std::time::Instant;

    // fastsin is 6 x times faster than .sin()
    pub fn speed01() {
        let iters = 100_000_000;
        let t = Instant::now();
        let mut fs = FastSin::new(1., 440., 0.).with_sample_rate(44100.);
        let mut s = 0.;
        for i in 0..iters {
            s += fs.next();
        }
        println!("lap fastsin: {:?} -> {s}", Instant::now() - t, s = s);

        let t = Instant::now();
        s = 0.;
        for i in 0..iters {
            s += (i as f64).sin();
        }
        println!("lap .sin(): {:?} -> {s}", Instant::now() - t, s = s)
    }

    #[test]
    fn spped_test() {
        speed01()
    }
}
