// polyhedron
#![allow(dead_code)]

use crate::color::Color;
use crate::vertex::{add, cross, divc, dot, normalize, sub};
use hashbrown::HashSet;
use rayon::prelude::*;
use std::f32::consts::PI;

// util funcs
pub fn sinf(x: f32) -> f32 {
    x.sin()
}
pub fn cosf(x: f32) -> f32 {
    x.cos()
}
pub fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}

const M_PI: f32 = PI;

type Int4 = [u32; 4];

#[derive(PartialEq)]
pub struct Int4int {
    _i4: Int4,
    i: u32,
}

// Polyhedron

#[derive(Clone, Debug)]
pub struct Polyhedron {
    pub name: String,
    pub faces: Vec<Vec<u32>>,
    pub vertexes: Vec<Vec<f32>>,
}

impl Polyhedron {
    pub fn tetrahedron() -> Self {
        Self {
            name: format!("T"),
            faces: vec![vec![0, 1, 2], vec![0, 2, 3], vec![0, 3, 1], vec![1, 3, 2]],
            vertexes: vec![
                vec![1.0, 1.0, 1.0],
                vec![1.0, -1.0, -1.0],
                vec![-1.0, 1.0, -1.0],
                vec![-1.0, -1.0, 1.0],
            ],
        }
    }
    pub fn cube() -> Self {
        Self {
            name: format!("C"),
            faces: vec![
                vec![3, 0, 1, 2],
                vec![3, 4, 5, 0],
                vec![0, 5, 6, 1],
                vec![1, 6, 7, 2],
                vec![2, 7, 4, 3],
                vec![5, 4, 7, 6],
            ],
            vertexes: vec![
                vec![0.707, 0.707, 0.707],
                vec![-0.707, 0.707, 0.707],
                vec![-0.707, -0.707, 0.707],
                vec![0.707, -0.707, 0.707],
                vec![0.707, -0.707, -0.707],
                vec![0.707, 0.707, -0.707],
                vec![-0.707, 0.707, -0.707],
                vec![-0.707, -0.707, -0.707],
            ],
        }
    }

    pub fn icosahedron() -> Self {
        Self {
            name: format!("I"),
            faces: vec![
                vec![0, 1, 2],
                vec![0, 2, 3],
                vec![0, 3, 4],
                vec![0, 4, 5],
                vec![0, 5, 1],
                vec![1, 5, 7],
                vec![1, 7, 6],
                vec![1, 6, 2],
                vec![2, 6, 8],
                vec![2, 8, 3],
                vec![3, 8, 9],
                vec![3, 9, 4],
                vec![4, 9, 10],
                vec![4, 10, 5],
                vec![5, 10, 7],
                vec![6, 7, 11],
                vec![6, 11, 8],
                vec![7, 10, 11],
                vec![8, 11, 9],
                vec![9, 11, 10],
            ],
            vertexes: vec![
                vec![0., 0., 1.176],
                vec![1.051, 0., 0.526],
                vec![0.324, 1.0, 0.525],
                vec![-0.851, 0.618, 0.526],
                vec![-0.851, -0.618, 0.526],
                vec![0.325, -1.0, 0.526],
                vec![0.851, 0.618, -0.526],
                vec![0.851, -0.618, -0.526],
                vec![-0.325, 1.0, -0.526],
                vec![-1.051, 0., -0.526],
                vec![-0.325, -1.0, -0.526],
                vec![0., 0., -1.176],
            ],
        }
    }

    pub fn octahedron() -> Self {
        Self {
            name: format!("O"),
            faces: vec![
                vec![0, 1, 2],
                vec![0, 2, 3],
                vec![0, 3, 4],
                vec![0, 4, 1],
                vec![1, 4, 5],
                vec![1, 5, 2],
                vec![2, 5, 3],
                vec![3, 5, 4],
            ],
            vertexes: vec![
                vec![0., 0., 1.414],
                vec![1.414, 0., 0.],
                vec![0., 1.414, 0.],
                vec![-1.414, 0., 0.],
                vec![0., -1.414, 0.],
                vec![0., 0., -1.414],
            ],
        }
    }

    pub fn dodecahedron() -> Self {
        Self {
            name: format!("D"),
            faces: vec![
                vec![0, 1, 4, 7, 2],
                vec![0, 2, 6, 9, 3],
                vec![0, 3, 8, 5, 1],
                vec![1, 5, 11, 10, 4],
                vec![2, 7, 13, 12, 6],
                vec![3, 9, 15, 14, 8],
                vec![4, 10, 16, 13, 7],
                vec![5, 8, 14, 17, 11],
                vec![6, 12, 18, 15, 9],
                vec![10, 11, 17, 19, 16],
                vec![12, 13, 16, 19, 18],
                vec![14, 15, 18, 19, 17],
            ],
            vertexes: vec![
                vec![0., 0., 1.07047],
                vec![0.713644, 0., 0.797878],
                vec![-0.356822, 0.618, 0.797878],
                vec![-0.356822, -0.618, 0.797878],
                vec![0.797878, 0.618034, 0.356822],
                vec![0.797878, -0.618, 0.356822],
                vec![-0.934172, 0.381966, 0.356822],
                vec![0.136294, 1.0, 0.356822],
                vec![0.136294, -1.0, 0.356822],
                vec![-0.934172, -0.381966, 0.356822],
                vec![0.934172, 0.381966, -0.356822],
                vec![0.934172, -0.381966, -0.356822],
                vec![-0.797878, 0.618, -0.356822],
                vec![-0.136294, 1.0, -0.356822],
                vec![-0.136294, -1.0, -0.356822],
                vec![-0.797878, -0.618034, -0.356822],
                vec![0.356822, 0.618, -0.797878],
                vec![0.356822, -0.618, -0.797878],
                vec![-0.713644, 0., -0.797878],
                vec![0., 0., -1.07047],
            ],
        }
    }

    pub fn pyramid(n: u32) -> Self {
        let theta = (2. * PI) / n as f32; // pie angle
        let height = 1.;

        let mut vertexes = vec![];
        let mut faces = vec![];
        for i in 0..n {
            vertexes.push(vec![
                -(i as f32 * theta).cos(),
                -(i as f32 * theta).sin(),
                -0.2,
            ]);
        }
        vertexes.push(vec![0., 0., height]); // apex

        faces.push(Self::range(n - 1, 0, true)); // base
        for i in 0..n {
            // n triangular sides
            faces.push(vec![i, (i + 1) % n, n])
        }

        Self {
            name: format!("P{}", n),
            vertexes: vertexes,
            faces: faces,
        }
    }

    pub fn prism(n: u32) -> Self {
        let theta = (2. * PI) / n as f32; // pie angle
        let h = (theta / 2.).sin(); // half-edge

        let mut vertexes = vec![];
        for i in 0..n {
            vertexes.push(vec![
                -(i as f32 * theta).cos(),
                -(i as f32 * theta).sin(),
                -h,
            ])
        }
        for i in 0..n {
            vertexes.push(vec![
                -(i as f32 * theta).cos(),
                -(i as f32 * theta).sin(),
                h,
            ])
        }
        // # vertex #'s 0 to n-1 around one face, vertex #'s n to 2n-1 around other

        let mut faces = vec![];
        faces.push(Self::range(n - 1, 0, true));
        faces.push(Self::range(n, 2 * n, false));
        for i in 0..n {
            faces.push(vec![i, (i + 1) % n, ((i + 1) % n) + n, i + n])
        }

        Self {
            name: format!("R{}", n),
            vertexes,
            faces,
        }
    }

    pub fn antiprism(n: u32) -> Self {
        let theta = (2. * PI) / n as f32; // pie angle
        let mut h = (1. - (4. / ((4. + (2. * (theta / 2.).cos())) - (2. * (theta).cos())))).sqrt();
        let mut r = (1. - (h * h)).sqrt();
        let f = ((h * h) + (r * (theta / 2.).cos()).powf(2.)).sqrt();

        // correction so edge midpoints (not vertexes) on unit sphere
        r = -r / f;
        h = -h / f;

        let mut vertexes = vec![];

        for i in 0..n {
            vertexes.push(vec![
                r * (i as f32 * theta).cos(),
                r * (i as f32 * theta).sin(),
                h,
            ])
        }
        for i in 0..n {
            vertexes.push(vec![
                r * ((i as f32 + 0.5) * theta).cos(),
                r * ((i as f32 + 0.5) * theta).sin(),
                -h,
            ])
        }

        let mut faces = vec![];
        faces.push(Self::range(n - 1, 0, true));
        faces.push(Self::range(n, (2 * n) - 1, true)); // top
        for i in 0..n {
            // 2n triangular sides
            faces.push(vec![i, (i + 1) % n, i + n]);
            faces.push(vec![i, i + n, ((((n + i) - 1) % n) + n)]);
        }
        Self {
            name: format!("A{}", n),
            vertexes,
            faces,
        }
    }

    pub fn cupola(n: u32, alpha: f32, height: f32) -> Self {
        let nf = n as f32;

        if n < 2 {
            return Self {
                name: String::default(),
                vertexes: vec![],
                faces: vec![],
            };
        }

        let s = 1.0; // alternative face/height scaling
        let rb = s / 2. / sinf(PI / 2. / n as f32);
        let rt = s / 2. / sinf(PI / n as f32);

        let mut height = height as f32;
        if height == 0. {
            height = rb - rt
        }

        // set correct height for regularity for n=3,4,5
        if n >= 3 && n <= 5 {
            height = s * sqrtf(1. - 1. / 4. / sinf(PI / nf) / sinf(PI / nf));
        }
        // init 3N vertexes
        let mut vertexes = vec![vec![]; (n * 3) as usize];

        // fill vertexes

        for i in 0..n {
            let fi = i as f32;
            vertexes[i as usize * 2] = vec![
                rb * cosf(PI * (2. * fi) / nf + PI / 2. / nf + alpha),
                rb * sinf(PI * (2. * fi) / nf + PI / 2. / nf + alpha),
                0.0,
            ];
            vertexes[2 * i as usize + 1] = vec![
                rb * cosf(PI * (2. * fi + 1.) / nf + PI / 2. / nf - alpha),
                rb * sinf(PI * (2. * fi + 1.) / nf + PI / 2. / nf - alpha),
                0.0,
            ];
            vertexes[(2 * n + i) as usize] = vec![
                rt * cosf(2. * PI * fi / nf),
                rt * sinf(2. * PI * fi / nf),
                height,
            ];
        }

        let mut faces = vec![];
        faces.push(Self::range(2 * n - 1, 0, true));
        faces.push(Self::range(2 * n, 3 * n - 1, true)); // base, top
        for i in 0..n {
            // n triangular sides and n square sides
            faces.push(vec![
                (2 * i + 1) % (2 * n),
                (2 * i + 2) % (2 * n),
                2 * n + (i + 1) % n,
            ]);
            faces.push(vec![
                2 * i,
                (2 * i + 1) % (2 * n),
                2 * n + (i + 1) % n,
                2 * n + i,
            ]);
        }

        Self {
            name: format!("U{}", n),
            vertexes,
            faces,
        }
    }

    pub fn anticupola(n: u32, alpha: f32, height: f32) -> Self {
        if n < 3 {
            return Self {
                name: String::default(),
                vertexes: vec![],
                faces: vec![],
            };
        }

        let nf = n as f32;

        let s = 1.0; // alternative face/height scaling
        let rb = s / 2. / sinf(PI / 2. / n as f32);
        let rt = s / 2. / sinf(PI / n as f32);

        let mut height = height as f32;
        if height == 0. {
            height = rb - rt
        }

        // init 3N vertexes
        let mut vertexes = vec![vec![]; (n * 3) as usize];

        // fill vertexes
        for i in 0..n {
            let fi = i as f32;
            vertexes[2 * i as usize] = vec![
                rb * cosf(M_PI * (2. * fi) / nf + alpha),
                rb * sinf(M_PI * (2. * fi) / nf + alpha),
                0.0,
            ];
            vertexes[2 * i as usize + 1] = vec![
                rb * cosf(M_PI * (2. * fi + 1.) / nf - alpha),
                rb * sinf(M_PI * (2. * fi + 1.) / nf - alpha),
                0.0,
            ];
            vertexes[(2 * n + i) as usize] = vec![
                rt * cosf(2. * M_PI * fi / nf),
                rt * sinf(2. * M_PI * fi / nf),
                height,
            ];
        }
        let mut faces = vec![];
        faces.push(Self::range(2 * n - 1, 0, true));
        faces.push(Self::range(2 * n, 3 * n - 1, true)); // base, top

        for i in 0..n {
            // n triangular sides and n square sides
            faces.push(vec![
                (2 * i) % (2 * n),
                (2 * i + 1) % (2 * n),
                2 * n + (i) % n,
            ]);
            faces.push(vec![
                2 * n + (i + 1) % n,
                (2 * i + 1) % (2 * n),
                (2 * i + 2) % (2 * n),
            ]);
            faces.push(vec![
                2 * n + (i + 1) % n,
                2 * n + (i) % n,
                (2 * i + 1) % (2 * n),
            ]);
        }

        Self {
            name: format!("V{}", n),
            vertexes,
            faces,
        }
    }

    fn v_normal(vs: &Vec<&Vec<f32>>) -> Vec<f32> {
        cross(&sub(&vs[1], &vs[0]), &sub(&vs[2], &vs[1]))
    }

    fn normal(&self, face: &Vec<u32>) -> Vec<f32> {
        let vs = face
            .iter()
            .take(3)
            .map(|vix| &self.vertexes[*vix as usize])
            .collect::<Vec<_>>();
        Self::v_normal(&vs)
    }
    fn center(&self, face: &Vec<u32>) -> Vec<f32> {
        let sum = face.iter().fold(vec![0., 0., 0.], |s, vix| {
            add(&s, &self.vertexes[*vix as usize])
        });
        divc(&sum, face.len() as f32)
    }
    fn area(&self, face: &Vec<u32>, normal: &Vec<f32>) -> f32 {
        let mut sum = vec![0., 0., 0.];
        let fl = face.len();
        let (mut v1, mut v2) = (
            &self.vertexes[face[fl - 2] as usize],
            &self.vertexes[face[fl - 1] as usize],
        );
        for i in 0..fl {
            sum = add(&sum, &cross(&v1, &v2));
            v1 = v2;
            v2 = &self.vertexes[face[i] as usize];
        }
        (dot(&normal, &sum)).abs() / 2.
    }

    pub fn avg_normals(&self) -> Vec<Vec<f32>> {
        self.faces
            .par_iter()
            .map(|face| {
                let mut normal_v = vec![0., 0., 0.];
                let (mut v1, mut v2) = (
                    &self.vertexes[face.len() - 2],
                    &self.vertexes[face.len() - 1],
                );

                for ic in face {
                    // running sum of normal vectors
                    let v3 = &self.vertexes[*ic as usize];
                    normal_v = add(&normal_v, &Self::v_normal(&vec![v1, v2, v3]));
                    v1 = v2;
                    v2 = v3; // shift over one
                }
                normal_v
            })
            .collect::<Vec<_>>()
    }
    pub fn calc_normals(&self) -> Vec<Vec<f32>> {
        self.faces.par_iter().map(|f| self.normal(f)).collect()
    }
    pub fn calc_centers(&self) -> Vec<Vec<f32>> {
        self.faces.par_iter().map(|f| self.center(f)).collect()
    }
    pub fn calc_areas(&self, normals: &Vec<Vec<f32>>) -> Vec<f32> {
        self.faces
            .par_iter()
            .zip(normals.par_iter())
            .map(|(face, normal)| self.area(face, normal))
            .collect()
    }
    pub fn calc_colors(&self, normals: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        const PALLETE_SIZE: usize = 16;
        fn sigfigs(f: &f32) -> u32 {
            // returns string w. nsigs digits ignoring magnitude
            (f.fract() * 1000000.) as u32
        }
        let areas = self.calc_areas(&normals);
        let mut color_set: HashSet<u32> = HashSet::new();

        areas.iter().for_each(|area| {
            // create color_set for each unique area
            color_set.insert(sigfigs(area));
        });

        let pallete = Color::random_pallete(PALLETE_SIZE); // create color_set
        areas
            .par_iter()
            .map(|area| pallete[*color_set.get(&sigfigs(area)).unwrap() as usize % PALLETE_SIZE].clone())
            .collect()
    }
    fn range(left: u32, right: u32, inclusive: bool) -> Vec<u32> {
        let mut range = vec![];

        let (right, left) = (right as i32, left as i32);

        let ascending = left < right;

        let end = if !inclusive {
            right
        } else {
            if ascending {
                right + 1
            } else {
                right - 1
            }
        };
        let mut i = left as i32;
        while if ascending { i < end } else { i > end } {
            range.push(i as u32);
            if ascending {
                i += 1
            } else {
                i -= 1
            }
        }
        range
    }

    pub fn normalize(&mut self) -> Self {
        let (mut min, mut max) = (f32::MAX, -f32::MAX);
        for v in &self.vertexes {
            max = max.max(
                *((*v)
                    .iter()
                    .max_by(|x, y| x.partial_cmp(&y).unwrap())
                    .unwrap()),
            );
            min = min.min(
                *((*v)
                    .iter()
                    .min_by(|x, y| x.partial_cmp(&y).unwrap())
                    .unwrap()),
            );
        }
        let dif = (max - min).abs();
        if dif != 0. {
            for v in &mut self.vertexes {
                for _v in v {
                    *_v /= dif
                }
            }
        }
        self.clone()
    }

    pub fn gen_face_map(&self) -> Vec<Int4int> {
        let mut face_map = vec![];

        for (i, face) in self.faces.iter().enumerate() {
            let mut v1 = face.last().unwrap();
            for v2 in face {
                face_map.push(Int4int {
                    _i4: [*v1, *v2, 0, 0],
                    i: i as u32,
                });
                v1 = v2
            }
        }
        face_map.sort_by(|a, b| a._i4.partial_cmp(&b._i4).unwrap());
        face_map
    }

    pub fn add_to_name(&mut self, s: &str) {
        self.name = format!("{}{}", s, self.name)
    }
}
