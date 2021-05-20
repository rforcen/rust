// lorentz attractor

use std::fs::File;
use std::io::BufWriter;
use std::io::*;

use crate::color_interp;
use crate::vector3d::*;

const N_ITERATIONS: usize = 300_000;

pub struct Lorentz {
    pub points: Vec<Vector3d<f32>>,
}

impl Lorentz {
    pub fn new() -> Self {
        Self {
            points: Self::calc_points(),
        }
    }
    fn calc_points() -> Vec<Vector3d<f32>> {
        let mut p = Vector3d::new(0., -2., -1.);
        let (a, b, c, h) = (10., 28., 8. / 3., 0.015);

        (0..N_ITERATIONS)
            .map(|_| {
                // lorentz linear function set l(i+1)=L(l(i))
                p.set(
                    p.x + h * a * (p.y - p.x),
                    p.y + h * (p.x * (b - p.z) - p.y),
                    p.z + h * (p.x * p.y - c * p.z),
                )
            })
            .collect()
    }
    pub fn write_wrl(&self, path: &str) {
        fn wrt(bw: &mut BufWriter<File>, s: &str) {
            bw.write(s.as_bytes()).unwrap();
        }
        let distances = Vector3d::scaled_distances(&self.points);

        let mut bw = BufWriter::new(File::create(path).unwrap());
        wrt(
            &mut bw,
            "#VRML V2.0 utf8 
Shape {
    geometry PointSet {
        coord Coordinate {
            point [\n",
        );

        for p in &self.points {
            wrt(&mut bw, &*format!("{},\n", p));
        }
        wrt(
            &mut bw,
            "]
    }
    color Color {
        color [",
        );

        // colors -> distance
        for d in &distances {
            let (r, g, b) = color_interp::interpolate(0xff0000, 0xff, *d);
            wrt(&mut bw, &*format!("{:.2} {:.2} {:.2},\n", r, g, b));
        }

        wrt(
            &mut bw,
            "]
    }
}
}",
        );
        bw.flush().unwrap();
    }
}
