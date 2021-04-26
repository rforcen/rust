// log spiral
#![allow(dead_code)]

use crate::music_freq::freq2color;
use nalgebra::{Point2, Point3};
use std::f32::consts::PI;

const SPEED: usize = 3000;

pub struct Spiral {
    x: f32,
    y: f32,
    w: u32,
    delta: f32,
    lh_index: usize,
}

impl Spiral {
    pub fn new(delta: f32, w: u32) -> Self {
        // delta=1.3
        Self {
            x: 0.,
            y: 0.,
            w,
            delta,
            lh_index: 0,
        }
    }

    // draw(painter, 1.02, 100, deltah+(((lh_index++) % 3000) / 3000.));
    // a, turns, delta
    // vec[from line, to line, color]
    pub fn draw(&mut self) -> Vec<(Point2<f32>, Point2<f32>, Point3<f32>)> {
        self._draw(
            1.02,
            100,
            self.delta + ((self.lh_index % SPEED) as f32 / SPEED as f32),
        )
    }
    fn _draw(
        &mut self,
        a: f32,
        turns: usize,
        delta: f32,
    ) -> Vec<(Point2<f32>, Point2<f32>, Point3<f32>)> {
        let mut lines = vec![];

        let x0 = 0.5 + self.x;
        let y0 = 0.5 + self.y;
        let (mut xa, mut ya, mut _xt, mut _yt) = (0., 0., 0., 0.);

        let mut ft = true;
        if turns == 0 || self.delta == 0. || a == 0. {
            return vec![];
        }

        let mut iters = 0;
        let max_iter = 250;

        // for (double th = 0; th < M_PI * 2. * turns && iters<max_iter; th += deltaTH, iters++)
        let mut th = 0.;
        let turns = PI * 2. * turns as f32;

        while th < turns && iters < max_iter {
            let r = a.powf(th);
            let (x, y) = (r * th.sin(), r * th.cos());

            if r > self.w as f32 {
                // outside w
                break;
            }

            if r > 5. {
                _xt = x + x0;
                _yt = y + y0;

                if !ft {
                    let color = freq2color(th as f64);
                    lines.push((
                        Point2::new(xa, ya),
                        Point2::new(_xt, _yt),
                        Point3::from(color),
                    ))
                }
                xa = x + x0;
                ya = y + y0;

                ft = false;
            }
            th += delta;
            iters += 1;
        }
        self.lh_index += 1;
        lines
    }
}
