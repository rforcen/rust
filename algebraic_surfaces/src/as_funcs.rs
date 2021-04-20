// Funcs for Parametric Surface in MT mode
#![allow(non_snake_case)]
use nalgebra::{Point2, Point3, Vector3};
use rayon::prelude::*;
use std::f32::consts::PI;

use crate::aux_funcs::*;
use crate::evals::*;

const TWO_PI: f32 = PI * 2.;

pub type Mesh = (Vec<Point3<f32>>, Vec<Vector3<f32>>, Vec<Point2<f32>>);

// Tanaka

pub struct Tanaka {
    a: f32,
    b1: f32,
    b2: f32,
    c: f32,
    d: f32,
    w: f32,
    h: f32,
}

impl Tanaka {
    pub fn new(param: usize) -> Self {
        let mut tanaka = Self {
            a: 0.,  // center hole size of a torus
            b1: 4., // number of cross
            b2: 3., // number of cross
            c: 4.,  // distance from the center of rotation
            d: 5.,  // number of torus
            w: 7.,  // gap width
            h: 4.,  // height
        };
        tanaka.set_param(param);
        tanaka
    }

    fn set_param(&mut self, param: usize) {
        const PARAM_SET: [[u8; 7]; 4] = [
            [0, 4, 3, 4, 5, 7, 4],
            [0, 4, 3, 0, 5, 7, 4],
            [0, 3, 4, 8, 5, 5, 2],
            [14, 3, 1, 8, 5, 5, 2],
        ];
        let param = param % 4;
        self.a = PARAM_SET[param][0] as f32;
        self.b1 = PARAM_SET[param][1] as f32;
        self.b2 = PARAM_SET[param][2] as f32;
        self.c = PARAM_SET[param][3] as f32;
        self.d = PARAM_SET[param][4] as f32;
        self.w = PARAM_SET[param][5] as f32;
    }

    fn get_ntorus(&self) -> f32 {
        self.d
    } // number of torus
    fn f(v: f32) -> f32 {
        sinf(2. * sinf(sinf(sinf(v))))
    }

    pub fn eval(&self, s: f32, t: f32) -> Point3<f32> {
        Point3::new(
            (self.a - cosf(t) + self.w * sinf(self.b1 * s)) * cosf(self.b2 * s),
            (self.a - cosf(t) + self.w * sinf(self.b1 * s)) * Self::f(self.b2 * s),
            self.h * (self.w * sinf(self.b1 * s) + Self::f(t)) + self.c,
        )
    }
}

pub fn calc_coords_mt(n_func: usize, resol: usize) -> Mesh {
    let ranges = [
        [(0., PI), (0., PI)],
        [(0., PI), (0., PI)],
        [(0., 1.), (0., TWO_PI)],
        [(0., TWO_PI), (0., TWO_PI)],
        [(0., PI), (0., PI)],
        [(-20., 20.), (20., 80.)],
        [(0., TWO_PI), (0., TWO_PI)],
        [(0., TWO_PI), (0., TWO_PI)],
        [(0., TWO_PI), (0., TWO_PI)],
        [(0., TWO_PI), (0., TWO_PI)],
        [(-1., 1.), (-1., 1.)],
        [(1., 30.), (1., 30.)],
        [(0., 1.), (-1., 1.)],
        [(0., TWO_PI), (0., TWO_PI)],
        [(0., TWO_PI), (0., TWO_PI)],
        [(0., TWO_PI), (-PI, PI)],
        [(0., TWO_PI), (0., TWO_PI)],
        [(-2., 2.), (-1., 1.)],
        [(0., 3.), (0., TWO_PI)],
        [(-10., 10.), (-10., 10.)],
        [(0., TWO_PI), (0., TWO_PI)],
        [(0., TWO_PI), (0., TWO_PI)],
        [(-4., 4.), (-3.75, 3.75)],
        [(0., TWO_PI), (0., TWO_PI)],
    ];

    let func = match n_func {
        0 => Cap_eval,
        1 => Boy_eval,
        2 => Roman_eval,
        3 => SeaShell_eval,
        4 => TudorRose_eval,
        5 => Breather_eval,
        6 => KleinBottle_eval,
        7 => KleinBottle0_eval,
        8 => Bour_eval,
        9 => Dini_eval,
        10 => Enneper_eval,
        11 => Scherk_eval,
        12 => ConicalSpiral_eval,
        13 => BohemianDome_eval,
        14 => AstroidalEllipse_eval,
        15 => Apple_eval,
        16 => Ammonite_eval,
        17 => PluckerConoid_eval,
        18 => Cayley_eval,
        19 => UpDownShell_eval,
        20 => ButterFly_eval,
        21 => Rose_eval,
        22 => Kuen_eval,
        //23 | 24 | 25 | 26 => Tanaka
        _ => Cap_eval,
    };
    let range_u = ranges[n_func][0];
    let range_v = ranges[n_func][1];

    let (from_u, dif_u) = (range_u.0, (range_u.1 - range_u.0).abs());
    let (from_v, dif_v) = (range_v.0, (range_v.1 - range_v.0).abs());

    let scale_u = |val: f32| val * dif_u + from_u;
    let scale_v = |val: f32| val * dif_v + from_v;

    // generate vertex, textures
    let delta = 1. / resol as f32;

    let mut coords = (0..resol * resol)
        .into_par_iter()
        .map(|i| {
            func(
                scale_u((i / resol) as f32 * delta),
                scale_v((i % resol) as f32 * delta),
            )
        })
        .collect::<Vec<Point3<f32>>>();

    // scale
    let max = coords.par_iter().cloned().reduce(
        || Point3::new(-f32::MAX, -f32::MAX, -f32::MAX),
        |a, b| a.sup(&b),
    );
    let max = max
        .iter()
        .max_by(|a, b| a.partial_cmp(&b).unwrap())
        .unwrap();
    let min = coords.par_iter().cloned().reduce(
        || Point3::new(f32::MAX, f32::MAX, f32::MAX),
        |a, b| a.inf(&b),
    );
    let min = min
        .iter()
        .min_by(|a, b| a.partial_cmp(&b).unwrap())
        .unwrap();

    let diff = (max - min).abs();
    if diff != 0. {
        coords.par_iter_mut().for_each(|p| *p /= diff)
    }
    (coords, vec![], vec![])
}
