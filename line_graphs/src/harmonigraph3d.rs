#![allow(dead_code)]
// harmonigraph3d.rs
use line_graphs::{LinesVec3D, PI2};
use nalgebra::Point3;
use rayon::prelude::*;
use crate::color_interp::*;

pub struct HarmoniGraph3D {
    p1: f32,
    p2: f32,
    p3: f32,
    f1: f32,
    f2: f32,
    color_mode: usize,
    scale: f32,
}

impl HarmoniGraph3D {
    pub fn new() -> Self {
        Self {
            p1: 0.,
            p2: 0.,
            p3: 0.,
            f1: 0.,
            f2: 0.,
            color_mode: 0,
            scale: 1.,
        }
    }
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
    pub fn with_preset(mut self, preset: usize) -> Self {
        static PRESETS: [[f32; 6]; 12] = [
            [1., 2., 3., 0.46, 1.5e-8, 1.],
            [2., 3., 4., 0.25, 0., 2.],
            [1., 3., 1., 0.25, 1.5e-8, 4.],
            [2., 1., 3., 0.37, 0., 2.],
            [2., 2., 11., 0.25, 1.5e-8, 2.],
            [1., 1., 1.01, 0.25, 0.29, 0.],
            [4., 4., 22., 0.25, 0., 4.],
            [1., 1., 4., 0.43, 1.5e-8, 3.],
            [1., 2., 0., 0.1, 0.3, 2.],
            [6., 3., 2., 0.21, 0.3, 0.],
            [1., 2., -1., 0.1, 0.3, 2.],
            [6., -3., 2., 0.21, 0.3, 0.],
        ];
        let preset = preset % PRESETS.len();
        self.p1 = PRESETS[preset][0];
        self.p2 = PRESETS[preset][1];
        self.p3 = PRESETS[preset][2];
        self.f1 = PRESETS[preset][3];
        self.f2 = PRESETS[preset][4];
        self.color_mode = PRESETS[preset][5] as usize;

        self
    }
    pub fn generate_lines(&mut self) -> LinesVec3D {
        let calc_distance = |x: f32, y: f32, z: f32| (x * x + y * y + z * z).sqrt();
        let ramainder = |x: f32| x.rem_euclid(1.);


        let sp_space = 3e-5;
        let size = 1.;

        let n_coords = (size / sp_space) as usize;
        let distance = size;

        let calc_coord = |ns: usize| -> (f32, f32, f32, (f32, f32, f32)) {
            let period = (ns + 1) as f32 * 0.02;
            let range = 1. - (ns + 1) as f32 * sp_space;

            let (x, y, z) = (
                (period * self.p2).sin() * range * self.scale,
                (period * self.p1 + self.f1 * PI2).sin() * range * self.scale,
                (period * self.p3 + self.f2 * PI2).sin() * range * self.scale,
            );

            let ratio = match self.color_mode {
                0 => ns as f32 / n_coords as f32,
                1 => ramainder(z / (size * 2.) + 0.5),
                2 => calc_distance(x, y, z) / distance * 1.5,
                3 => ramainder(period / PI2),
                4 => ramainder(period / (PI2 / 8.)),
                _ => 1.,
            };

    

            (x, y, z, interpolate(0xff_00_00, 0x00_00_ff, ratio)) // coord & color(r,g,b)
        };

        (0..n_coords) // parallel generator
            .into_par_iter()
            .map(|i| {
                let (x, y, z, (r, g, b)) = calc_coord(i);
                (Point3::new(x, y, z), Point3::new(r, g, b))
            })
            .collect()
    }
    pub fn next(&mut self) {
        self.p1 += 0.0004;
        self.p3 += 0.0004;
    }
    pub fn next_by(&mut self, p1: f32, p2: f32) {
        self.p1 += p1;
        self.p3 += p2;
    }
}
