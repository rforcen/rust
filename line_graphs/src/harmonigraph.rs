// harmonigraph.rs
use crate::color_interp::*;
use line_graphs::*;
use nalgebra::{Point2, Point3};
use rayon::prelude::*;

const MAX_ITER: usize = 8000;

pub struct HarmoniGraph {
    f1: f32,
    f2: f32,
    f3: f32,
    f4: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    r: f32,
    t_inc: f32,
    scale: f32,
}

impl HarmoniGraph {
    pub fn new() -> Self {
        Self {
            f1: 0.,
            f2: 0.,
            f3: 0.,
            f4: 0.,
            p1: 0.,
            p2: 0.,
            p3: 0.,
            p4: 0.,
            r: 0.001,
            t_inc: 0.0051,
            scale: 1.,
        }
    }
    pub fn with_preset(mut self, preset: usize) -> Self {
        static PRESET: [[u32; 4]; 9] = [
            [5, 3, 10, 4],
            [17, 17, 34, 7],
            [19, 19, 44, 14],
            [48, 34, 33, 27],
            [9, 22, 6, 44],
            [17, 5, 3, 25],
            [37, 34, 68, 14],
            [30, 30, 60, 40],
            [4, 12, 16, 8],
        ];

        let preset = preset % PRESET.len();
        self.f1 = PRESET[preset][0] as f32;
        self.f2 = PRESET[preset][1] as f32;
        self.f3 = PRESET[preset][2] as f32;
        self.f4 = PRESET[preset][3] as f32;

        self
    }
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
    pub fn generate_lines(&mut self, w: u32) -> LinesVec {
        let calc_coord = |t: f32| -> (f32, f32) {
            let x = (-self.r * t).exp()
                * ((self.f1 * t + self.p1).sin() + (self.f2 * t + self.p2).sin());
            let y = (-self.r * t).exp()
                * ((self.f3 * t + self.p3).sin() + (self.f4 * t + self.p4).sin());
            (x, y)
        };

        let w4 = self.scale * w as f32 / 4.;

        (0..MAX_ITER)
            .into_par_iter()
            .map(|i| {
                let (x, y) = calc_coord(i as f32 * self.t_inc);
                (
                    Point2::new(x * w4, y * w4),
                    Point3::from(default_interpolate(i as f32 / MAX_ITER as f32)),
                )
            })
            .collect()
    }
    pub fn next(&mut self) {
        self.p1 += 0.04;
        self.p3 += 0.04;
    }
}
