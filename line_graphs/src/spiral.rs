// log spiral
#![allow(dead_code)]

use crate::music_freq::freq2color;
use line_graphs::LinesVec;
use nalgebra::{Point2, Point3};
use rayon::prelude::*;

const SPEED: usize = 3000;
const MAX_ITER: usize = 250;

pub struct Spiral {
    delta: f32,
    index: usize,
}

impl Spiral {
    pub fn new() -> Self {
        Self {
            delta: 1.3, // delta=1.3
            index: 0,
        }
    }
    pub fn with_delta(mut self, delta: f32) -> Self {
        self.delta = delta;
        self
    }

    // amp, iters, delta
    // vec[coord, color]
    pub fn generate_lines(&mut self) -> LinesVec {
        self._generate_lines(
            1.02,     // amp
            MAX_ITER, // iters
            self.delta + ((self.index % SPEED) as f32 / SPEED as f32),
        )
    }
    fn _generate_lines(&mut self, amp: f32, iters: usize, delta: f32) -> LinesVec {
        self.index += 1;

        (0..iters) // parallel generator
            .into_par_iter()
            .map(|i| {
                let th = delta * i as f32;
                let r = amp.powf(th) + 3.;
                let (x, y) = (r * th.sin(), r * th.cos());

                (Point2::new(x, y), Point3::from(freq2color(th as f64)))
            })
            .collect()
    }
}
