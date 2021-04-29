// lissajous.rs

use crate::music_freq::freq2color;
use line_graphs::LinesVec;
use nalgebra::{Point2, Point3};

const MAX_VALUES: usize = 360;

pub struct Lissajous {
    n: usize,
    m: usize,
    iyo: usize,
    scale: f32,
    vsin: Vec<f32>,
}

impl Lissajous {
    pub fn new() -> Self {
        Self {
            n: 0,
            m: 0,
            iyo: 0,
            scale: 1.,
            vsin: Self::calc_vsin(),
        }
    }
    pub fn with_params(mut self, n: usize, m: usize) -> Self {
        self.n = n;
        self.m = m;
        self
    }
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
    pub fn with_preset(mut self, preset: usize) -> Self {
        static PRESET: [[usize; 2]; 23] = [
            [2, 3],
            [60, 98],
            [108, 176],
            [131, 212],
            [72, 118],
            [90, 146],
            [115, 187],
            [315, 203],
            [125, 203],
            [327, 66],
            [131, 293],
            [131, 272],
            [131, 65],
            [60, 288],
            [108, 65],
            [315, 250],
            [240, 203],
            [311, 203],
            [235, 203],
            [275, 246],
            [243, 184],
            [287, 268],
            [166, 69],
        ];
        let preset = preset % PRESET.len();
        self.m = PRESET[preset][0];
        self.n = PRESET[preset][1];
        self
    }
    fn calc_vsin() -> Vec<f32> {
        const SIF: f32 = 0.01745240643728466;
        const COF: f32 = 0.9998476951563913;
        let (mut si, mut co) = (-SIF, COF);
        let radius = 0.5;
        let x = -radius;
        let mut vsin = vec![0.; MAX_VALUES + 1];

        for i in 0..=MAX_VALUES / 2 {
            si = si * COF + co * SIF;
            co = co * COF - si * SIF;
            let s = radius * si;
            vsin[i] = x + radius + s;
            vsin[i + MAX_VALUES / 2] = x + radius - s;
        }
        vsin
    }
    pub fn generate_lines(&mut self, w: usize) -> LinesVec {
        let r = w as f32;

        let (mut ixo, mut iyo) = (0, self.iyo);
        let mut vlin = vec![];
        vlin.reserve(MAX_VALUES * 2);

        for j in (0..MAX_VALUES * 2).rev() {
            let color = freq2color(j as f64);

            let ix = (ixo + self.m) % MAX_VALUES;
            let iy = (iyo as usize + self.n) % (MAX_VALUES - 1);

            vlin.push((
                Point2::new(self.vsin[ix] * r, self.vsin[iy] * r) * self.scale,
                Point3::from(color),
            ));

            ixo = ix;
            iyo = iy;
        }
        self.iyo = iyo;
        vlin
    }
}
