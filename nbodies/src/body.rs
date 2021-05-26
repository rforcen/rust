// body.rs

#![allow(dead_code)]

use nalgebra::geometry::Translation3;
use nalgebra::Vector2;
use std::convert::From;
use std::fs;

pub type Vec2 = Vector2<f64>;
const G: f64 = 6.67e-11;

// body
#[derive(Debug)]
pub struct Body {
    r: Vec2, // position
    v: Vec2, // velocity
    mass: f64,
}

impl Body {
    pub fn new(r: Vec2, v: Vec2, mass: f64) -> Self {
        Self { r, v, mass }
    }
    pub fn new_from_raw(rx: f64, ry: f64, vx: f64, vy: f64, mass: f64) -> Self {
        Self {
            r: Vec2::new(rx, ry),
            v: Vec2::new(vx, vy),
            mass,
        }
    }

    pub fn do_move(&mut self, f: Vec2, dt: f64) {
        self.v += (f * (1. / self.mass)) * dt;
        self.r += self.v * dt;
    }
    pub fn force_from(&self, b: &Body) -> Vec2 {
        // newton gravity formula
        let delta = b.r - self.r;
        let f = (G * self.mass * b.mass) / delta.magnitude_squared();
        delta / delta.magnitude() * f
    }
}

impl From<&[f64]> for Body {
    fn from(v: &[f64]) -> Self {
        Self {
            r: Vec2::new(v[0], v[1]),
            v: Vec2::new(v[2], v[3]),
            mass: v[4],
        }
    }
}
impl From<[f64; 5]> for Body {
    fn from(v: [f64; 5]) -> Self {
        Self {
            r: Vec2::new(v[0], v[1]),
            v: Vec2::new(v[2], v[3]),
            mass: v[4],
        }
    }
}

// body set
pub struct BodySet {
    pub bodies: Vec<Body>,
    radius: f64,
}

impl BodySet {
    pub fn new() -> Self {
        Self {
            bodies: vec![],
            radius: 0.,
        }
    }

    pub fn three_bodies() -> Self {
        Self {
            radius: 1.25e11,
            bodies: vec![
                Body::from([0., 0., 0.0500e04, 0., 5.974e24]),
                Body::from([0., 4.500e10, 3e4, 0., 1.989e30]),
                Body::from([0., -4.500e10, -3e4, 0., 1.989e30]),
            ],
        }
    }
    pub fn inc_time(&mut self, dt: f64) {
        let mut f = vec![Vec2::zeros(); self.bodies.len()];
        // calc forces
        for i in 0..self.bodies.len() {
            for j in 0..self.bodies.len() {
                if i != j {
                    f[i] += self.bodies[i].force_from(&self.bodies[j])
                }
            }
        }
        // move the bodies
        for i in 0..self.bodies.len() {
            self.bodies[i].do_move(f[i], dt);
        }
    }
    pub fn get_coords(&self, scale: f64) -> Vec<Vec2> {
        self.bodies
            .iter()
            .map(|b| scale * b.r / self.radius)
            .collect()
    }

    pub fn get_translation(&self, scale: f64, i: usize) -> Translation3<f32> {
        let coord = scale * self.bodies[i].r / self.radius;
        Translation3::new(coord.x as f32, coord.y as f32, 0.)
    }

    pub fn read(&mut self, path: &str) {
        let contents = fs::read_to_string(path).expect("error reading the file");
        let nums: Vec<f64> = contents
            .split_whitespace()
            .map(|s| if let Ok(x) = s.parse::<f64>() { x } else { -1. })
            .collect();

        self.radius = nums[1];
        for i in 0..nums[0] as usize {
            self.bodies
                .push(Body::from(&nums[i * 6 + 2..i * 6 + 2 + 5]))
        }
    }
}

mod test {
    use super::*;
    #[test]
    fn create_1kiters() {
        let mut univ = BodySet::three_bodies();

        println!("{:?}", univ.bodies);
        for _ in 0..10000 {
            univ.inc_time(0.0001);
        }
        println!("{:?}", univ.bodies);
    }
}
