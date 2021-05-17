// lorentz attractor

use nalgebra::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::*;

const N_ITERATIONS: usize = 300_000;

pub struct Lorentz {
    p0: Point3<f32>,
    a: f32,
    b: f32,
    c: f32,
    h: f32,
    iterations: usize,
    ni: usize,
    pub pnts: Vec<Point3<f32>>,
}

impl Lorentz {
    pub fn new() -> Self {
        let mut s = Self {
            p0: Point3::new(0., -2., -1.),
            a: 10.,
            b: 28.,
            c: 8. / 3.,
            h: 0.015,
            iterations: N_ITERATIONS,
            ni: 0,
            pnts: vec![],
        };
        s.pnts = s.calc();
        s
    }
    pub fn calc(&mut self) -> Vec<Point3<f32>> {
        (0..self.iterations)
            .map(|_| {
                // lorentz linear function set
                self.p0 = Point3::new(
                    self.p0.x + self.h * self.a * (self.p0.y - self.p0.x),
                    self.p0.y + self.h * (self.p0.x * (self.b - self.p0.z) - self.p0.y),
                    self.p0.z + self.h * (self.p0.x * self.p0.y - self.c * self.p0.z),
                );
                // solution becomes next seed
                self.ni += 1;

                self.p0 * 0.1
            })
            .collect()
    }
    pub fn write_wrl(&self, path: &str) {
        // scale distances
        let distances: Vec<f32> = self
            .pnts
            .iter()
            .map(|p| distance(p, &Point3::origin()))
            .collect();
        let max = distances
            .iter()
            .max_by(|x, y| x.partial_cmp(&y).unwrap())
            .unwrap();
        let distances = distances.iter().map(|d| *d / *max).collect::<Vec<_>>();

        let mut bw = BufWriter::new(File::create(path).unwrap());
        bw.write(
            "#VRML V2.0 utf8 
Shape {
    geometry PointSet {
        coord Coordinate {
            point [\n"
                .as_bytes(),
        )
        .unwrap();

        for p in &self.pnts {
            bw.write(format!("{:.3} {:.3} {:.3},\n", p.x, p.y, p.z).as_bytes())
                .unwrap();
        }
        bw.write(
            "]
    }
    color Color {
        color ["
                .as_bytes(),
        )
        .unwrap();

        // colors -> distance
        for d in &distances {
            bw.write(format!("0.6 0.8 {:.3},\n", *d).as_bytes())
                .unwrap();
        }

        bw.write(
            "]
    }
}
}"
            .as_bytes(),
        )
        .unwrap();
        bw.flush().unwrap();
    }
}
