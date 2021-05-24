// attr3d.rs

use crate::vector3d::*;

use crate::color_interp;
use std::fs::File;
use std::io::BufWriter;
use std::io::*;

const ITERS: usize = 40000;
const INITIAL: usize = 100;
static PRESETS: [[f32; 6]; 17] = [
    [
        2.4132195, -2.1976194, 1.9788465, 2.2592974, 1.9788465, 2.2592974,
    ],
    [
        1.5240102, -1.3507154, -1.0039501, 4.373563, -1.0039501, 4.373563,
    ],
    [
        -0.7921877, 2.679759, -1.3762264, 4.2112646, -1.3762264, 4.2112646,
    ],
    [
        -2.9854157, 4.9019957, 2.7782016, -1.5313616, 2.7782016, -1.5313616,
    ],
    [
        -1.4460807, 2.2048817, 0.31676674, 2.6867342, 0.31676674, 2.6867342,
    ],
    [
        2.9146361, -3.4562402, 2.7556057, 2.003243, 2.7556057, 2.003243,
    ],
    [
        1.1250505, 4.6858244, 4.930643, -0.7496538, 4.930643, -0.7496538,
    ],
    [
        4.93571, 1.7190294, 4.1666403, 0.05218315, 4.1666403, 0.05218315,
    ],
    [
        -1.3544173, 1.1977987, -4.4361877, 0.72273064, -4.4361877, 0.72273064,
    ],
    [
        -1.6095421, 1.1782713, -2.0525944, -2.485671, -2.0525944, -2.485671,
    ],
    [
        -0.83017254,
        1.3246107,
        -3.6927686,
        1.7803202,
        -3.6927686,
        1.7803202,
    ],
    [
        1.440599, -1.837692, 4.4717484, -2.753281, 4.4717484, -2.753281,
    ],
    [
        -4.491502, 1.5501595, 2.0431209, -2.4081306, 2.0431209, -2.4081306,
    ],
    [
        -0.5917244, -1.5971804, -0.7590456, -3.8690374, -0.7590456, -3.8690374,
    ],
    [
        1.9087152, -4.3082824, -4.5445232, 0.45649147, -4.5445232, 0.45649147,
    ],
    [-4.0, 4.0, 4.0, -4.0, 4.0, -4.0],
    [-0.966918, 2.879879, 0.765145, 0.744728, 0.765145, 0.744728],
];
pub struct Attr3d {
    points: Vec<Vector3d<f32>>,
}

impl Attr3d {
    pub fn new(preset: usize, n_eval: usize) -> Self {
        Self {
            points: Self::eval(preset, n_eval),
        }
    }
    pub fn eval(preset: usize, n_eval: usize) -> Vec<Vector3d<f32>> {
        fn tuple6<T>(a: &[T]) -> (&T, &T, &T, &T, &T, &T) {
            (&a[0], &a[1], &a[2], &a[3], &a[4], &a[5])
        }
        let (a, b, c, d, e, f) = tuple6(&PRESETS[preset % PRESETS.len()]);
        let mut p = Vector3d::<f32>::new(0.1, 0.1, 0.1);

        let eval01 = |p: &mut Vector3d<f32>| {
            p.set(
                p.z * (a * p.x).sin() + (b * p.y).cos(),
                p.x * (c * p.y).sin() + (d * p.z).cos(),
                p.y * (e * p.z).sin() + (f * p.x).cos(),
            )
        };
        let eval02 = |p: &mut Vector3d<f32>| {
            p.set(
                p.z * (a * p.x).sin() - (b * p.y).cos(),
                p.x * (c * p.y).sin() - (d * p.z).cos(),
                p.y * (e * p.z).sin() - (f * p.x).cos(),
            )
        };

        match n_eval {
            0 => {
                for _ in 0..INITIAL {
                    eval01(&mut p);
                }
                (0..ITERS).map(|_| eval01(&mut p)).collect()
            }
            _ => {
                for _ in 0..INITIAL {
                    eval02(&mut p);
                }
                (0..ITERS).map(|_| eval02(&mut p)).collect()
            }
        }
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
