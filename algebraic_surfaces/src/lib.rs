// Parametric Surface
use nalgebra::{Point2, Point3, Vector3};
use std::f32::consts::PI;

mod aux_funcs;
use crate::aux_funcs::*;

const TWO_PI: f32 = PI * 2.;

pub const N_SURFACES: usize = 27;
pub const SURF_NAMES: [&str; N_SURFACES] = [
    "cap",
    "boy",
    "roman",
    "sea shell",
    "tudor rose",
    "breather",
    "klein bottle",
    "klein bottle 0",
    "bour",
    "dini",
    "enneper",
    "scherk",
    "conical spiral",
    "bohemian dome",
    "astrodial ellipse",
    "apple",
    "ammonite",
    "plucker comoid",
    "cayley",
    "up down shell",
    "butterfly",
    "rose",
    "kuen",
    "tanaka-0",
    "tanaka-1",
    "tanaka-2",
    "tanaka-3",
];

pub type Mesh = (Vec<Point3<f32>>, Vec<Vector3<f32>>, Vec<Point2<f32>>);
type Range = (f32, f32);

// ParametricSurface
pub trait ParametricSurface {
    fn eval(&self, u: f32, v: f32) -> Point3<f32>;

    fn calc_coords(&self, resol: usize, range_u: Range, range_v: Range) -> Mesh {
        let mut coords = vec![];
        let mut textures = vec![];

        let mut max_p = Point3::new(-f32::MAX, -f32::MAX, -f32::MAX);
        let mut min_p = Point3::new(f32::MAX, f32::MAX, f32::MAX);

        let (from_u, dif_u) = (range_u.0, (range_u.1 - range_u.0).abs());
        let (from_v, dif_v) = (range_v.0, (range_v.1 - range_v.0).abs());

        let mut add_text_vertex = |u: f32, v: f32| {
            // eval & add vertex, texture
            let scale_u = |val: f32| val * dif_u + from_u;
            let scale_v = |val: f32| val * dif_v + from_v;

            let p = self.eval(scale_u(u), scale_v(v));

            min_p = Point3::new(min_p.x.min(p.x), min_p.y.min(p.y), min_p.z.min(p.z));
            max_p = Point3::new(max_p.x.max(p.x), max_p.y.max(p.y), max_p.z.max(p.z));

            coords.push(p);
            textures.push(Point2::new(u, v))
        };

        // generate vertex, textures
        let delta = 1. / resol as f32;

        for i in 0..resol {
            for j in 0..resol {
                add_text_vertex(i as f32 * delta, j as f32 * delta);
            }
        }

        // scale coords
        let (min_val, max_val) = (
            min_p.x.min(min_p.y.min(min_p.z)),
            max_p.x.max(max_p.y.max(max_p.z)),
        );
        let diff = (max_val - min_val).abs();
        if diff != 0. {
            coords.iter_mut().for_each(|p| *p = *p / diff);
        }

        // normals
        let mut normals = vec![];
        for (i0, coord) in coords.iter().enumerate() {
            fn calc_normal(v0: &Point3<f32>, v1: Point3<f32>, v2: Point3<f32>) -> Vector3<f32> {
                (v2 - v0).cross(&(v1 - v0)) //.normalize()
            }
            let i1 = if i0 % resol == resol - 1 {
                i0 - 1
            } else {
                i0 + 1
            };
            let i2 = if i0 / resol < resol - 1 || i0 < resol {
                i0 + resol
            } else {
                i0 - resol
            };
            normals.push(calc_normal(coord, coords[i1], coords[i2]))
        }

        (coords, normals, textures)
    }
}

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

impl ParametricSurface for Tanaka {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        self.eval(u, v)
    }
}

// Cap
struct Cap {}

impl ParametricSurface for Cap {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        Point3::new(
            0.5 * cosf(u) * sinf(2. * v),
            0.5 * sinf(u) * sinf(2. * v),
            0.5 * (sqr(cosf(v)) - sqr(cosf(u)) * sqr(sinf(v))),
        )
    }
}

// Boy
struct Boy {}
impl ParametricSurface for Boy {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        let dv = 2. - sqrtf(2.) * sinf(3. * u) * sinf(2. * v);
        let d1 = cosf(u) * sinf(2. * v);
        let d2 = sqrtf(2.) * sqr(cosf(v));

        Point3::new(
            (d2 * cosf(2. * u) + d1) / dv,
            (d2 * sinf(2. * u) + d1) / dv,
            (3. * sqr(cosf(v))) / (2. - sqrtf(2.) * sinf(3. * u) * sinf(2. * v)),
        )
    }
}

// Roman
struct Roman {}
impl ParametricSurface for Roman {
    fn eval(&self, r: f32, t: f32) -> Point3<f32> {
        let r2 = r * r;
        let rq = sqrtf(1. - r2);
        let st = sinf(t);
        let ct = cosf(t);
        Point3::new(r2 * st * ct, r * st * rq, r * ct * rq)
    }
}

// SeaShell
struct SeaShell {}

impl ParametricSurface for SeaShell {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        let n = 5.6; // number of turns
        let h = 3.5; // height
        let p = 2.; // power
        let l = 4.; // Controls spike length
        let k = 9.;

        let W = |u: f32| powf(u / (2. * PI), p);

        Point3::new(
            W(u) * cosf(n * u) * (1. + cosf(v)),
            W(u) * sinf(n * u) * (1. + cosf(v)),
            W(u) * (sinf(v) + powf(sinf(v / 2.), k) * l) + h * powf(u / (2. * PI), p + 1.),
        )
    }
}

// TudorRose
struct TudorRose {}
impl ParametricSurface for TudorRose {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        fn r(u: f32, v: f32) -> f32 {
            cosf(v) * cosf(v) * max(fabs(sinf(4. * u)), 0.9 - 0.2 * fabs(cosf(8. * u)))
        }
        Point3::new(
            r(u, v) * cosf(u) * cosf(v),
            r(u, v) * sinf(u) * cosf(v),
            r(u, v) * sinf(v) * 0.5,
        )
    }
}

// BreatherSurface
struct BreatherSurface {}

impl ParametricSurface for BreatherSurface {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        let aa = 0.45; // Values from 0.4 to 0.6 produce sensible results
        let w1 = 1. - aa * aa;
        let w = sqrtf(w1);

        let d = |u, v| aa * (powf(w * cosh(aa * u), 2.) + powf(aa * sinf(w * v), 2.));

        Point3::new(
            -u + (2. * w1 * cosh(aa * u) * sinh(aa * u) / d(u, v)),
            2. * w * cosh(aa * u) * (-(w * cosf(v) * cosf(w * v)) - (sinf(v) * sinf(w * v)))
                / d(u, v),
            2. * w * cosh(aa * u) * (-(w * sinf(v) * cosf(w * v)) + (cosf(v) * sinf(w * v)))
                / d(u, v),
        )
    }
}

// KleinBottle
struct KleinBottle {}

impl ParametricSurface for KleinBottle {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        let t = 4.5;
        let tmp = 4. + 2. * cosf(u) * cosf(t * v) - sinf(2. * u) * sinf(t * v);

        Point3::new(
            sinf(v) * tmp,
            cosf(v) * tmp,
            2. * cosf(u) * sinf(t * v) + sinf(2. * u) * cosf(t * v),
        )
    }
}

// KleinBottle0

struct KleinBottle0 {}

impl ParametricSurface for KleinBottle0 {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        let _t = 4.5;

        Point3::new(
            if 0. <= u && u < PI {
                6. * cosf(u) * (1. + sinf(u)) + 4. * (1. - 0.5 * cosf(u)) * cosf(u) * cosf(v)
            } else {
                6. * cosf(u) * (1. + sinf(u)) + 4. * (1. - 0.5 * cosf(u)) * cosf(v + PI)
            },
            if 0. <= u && u < PI {
                16. * sinf(u) + 4. * (1. - 0.5 * cosf(u)) * sinf(u) * cosf(v)
            } else {
                16. * sinf(u)
            },
            4. * (1. - 0.5 * cosf(u)) * sinf(v),
        )
    }
}

// Bour

struct Bour {}

impl ParametricSurface for Bour {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        Point3::new(
            u * cosf(v) - 0.5 * u * u * cosf(2. * v),
            -u * sinf(v) - 0.5 * u * u * sinf(2. * v),
            4. / 3. * powf(u, 1.5) * cosf(1.5 * v),
        )
    }
}

struct Dini {}

impl ParametricSurface for Dini {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        let mut psi = 0.3; // aa;
        if psi < 0.001 {
            psi = 0.001
        };
        if psi > 0.999 {
            psi = 0.999
        };
        let psi = psi * PI;
        let sinpsi = sinf(psi);
        let cospsi = cosf(psi);
        let g = (u - cospsi * v) / sinpsi;
        let s = exp(g);
        let r = (2. * sinpsi) / (s + 1. / s);
        let t = r * (s - 1. / s) * 0.5;

        Point3::new(u - t, r * cosf(v), r * sinf(v))
    }
}

// Scherk
struct Scherk {}
impl ParametricSurface for Scherk {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        let aa = 0.1;
        let v = v + 0.1;

        Point3::new(u, v, (ln(fabs(cosf(aa * v) / cosf(aa * u)))) / aa)
    }
}

// Enneper
struct Enneper {}
impl ParametricSurface for Enneper {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        Point3::new(
            u - u * u * u / 3. + u * v * v,
            v - v * v * v / 3. + v * u * u,
            u * u - v * v,
        )
    }
}

// ConicalSpiral
struct ConicalSpiral {}
impl ParametricSurface for ConicalSpiral {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        Point3::new(u * v * sinf(15. * v), v, u * v * cosf(15. * v))
    }
}

// BohemianDome
struct BohemianDome {}
impl ParametricSurface for BohemianDome {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        let a = 0.5;
        let b = 1.5;
        let c = 1.;
        Point3::new(a * cosf(u), b * cosf(v) + a * sinf(u), c * sinf(v))
    }
}

// AstroidalEllipse
struct AstroidalEllipse {}
impl ParametricSurface for AstroidalEllipse {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        let a = 1.;
        let b = 1.;
        let c = 1.;
        Point3::new(
            powf(a * cosf(u) * cosf(v), 3.),
            powf(b * sinf(u) * cosf(v), 3.),
            powf(c * sinf(v), 3.),
        )
    }
}

// Apple
struct Apple {}
impl ParametricSurface for Apple {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        let r1 = 4.;
        let r2 = 3.8;
        Point3::new(
            cosf(u) * (r1 + r2 * cosf(v)) + powf(v / PI, 100.),
            sinf(u) * (r1 + r2 * cosf(v)) + 0.25 * cosf(5. * u),
            -2.3 * ln(1. - v * 0.3157) + 6. * sinf(v) + 2. * cosf(v),
        )
    }
}

// Ammonite
struct Ammonite {}
impl ParametricSurface for Ammonite {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        let w = |u| powf(u / (2. * PI), 2.2);
        let n = 5.6; // number of turns
        let f = 120.0; // wave frequency
        let a = 0.2; // wave amplitude
        Point3::new(
            w(u) * cosf(n * u) * (2. + sinf(v + cosf(f * u) * a)),
            w(u) * sinf(n * u) * (2. + sinf(v + cosf(f * u) * a)),
            w(u) * cosf(v),
        )
    }
}

// PluckerConoid
struct PluckerConoid {}
impl ParametricSurface for PluckerConoid {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        Point3::new(u * v, u * sqrtf(1. - sqr(v)), 1. - sqr(v))
    }
}

// Cayley
struct Cayley {}
impl ParametricSurface for Cayley {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        Point3::new(
            u * sinf(v) - u * cosf(v),
            sqr(u) * sinf(v) * cosf(v),
            cube(u) * sqr(sinf(v)) * cosf(v),
        )
    }
}

// UpDownShell
struct UpDownShell {}
impl ParametricSurface for UpDownShell {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        Point3::new(
            u * sinf(u) * cosf(v),
            u * cosf(u) * cosf(v),
            u * sinf(v), // -10,10, -10,10
        )
    }
}

// ButterFly
struct ButterFly {}
impl ParametricSurface for ButterFly {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        let t1 = (exp(cosf(u)) - 2. * cosf(4. * u) + sqr5(sinf(u / 12.))) * sinf(v);

        Point3::new(sinf(u) * t1, cosf(u) * t1, sinf(v))
    }
}

// Rose
struct Rose {}
impl ParametricSurface for Rose {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        let a = 1.;
        let n = 7.;
        Point3::new(
            a * sinf(n * u) * cosf(u) * sinf(v),
            a * sinf(n * u) * sinf(u) * sinf(v),
            cosf(v) / (n * 3.),
        )
    }
}

// Kuen
struct Kuen {}
impl ParametricSurface for Kuen {
    fn eval(&self, u: f32, v: f32) -> Point3<f32> {
        Point3::new(
            2. * cosh(v) * (cosf(u) + u * sinf(u)) / (cosh(v) * cosh(v) + u * u),
            2. * cosh(v) * (-u * cosf(u) + sinf(u)) / (cosh(v) * cosh(v) + u * u),
            v - (2. * sinh(v) * cosh(v)) / (cosh(v) * cosh(v) + u * u),
        )
    }
}

//
pub fn generate_alg_suf(ns: usize, resol: usize) -> Mesh {
    match ns {
        0 => Cap {}.calc_coords(resol, (0., PI), (0., PI)),
        1 => Boy {}.calc_coords(resol, (0., PI), (0., PI)),
        2 => Roman {}.calc_coords(resol, (0., 1.), (0., TWO_PI)),
        3 => SeaShell {}.calc_coords(resol, (0., TWO_PI), (0., TWO_PI)),
        4 => TudorRose {}.calc_coords(resol, (0., PI), (0., PI)),
        5 => BreatherSurface {}.calc_coords(resol, (-20., 20.), (20., 80.)),
        6 => KleinBottle {}.calc_coords(resol, (0., TWO_PI), (0., TWO_PI)),
        7 => KleinBottle0 {}.calc_coords(resol, (0., TWO_PI), (0., TWO_PI)),
        8 => Bour {}.calc_coords(resol, (0., TWO_PI), (0., TWO_PI)),
        9 => Dini {}.calc_coords(resol, (0., TWO_PI), (0., TWO_PI)),
        10 => Enneper {}.calc_coords(resol, (-1., 1.), (-1., 1.)),
        11 => Scherk {}.calc_coords(resol, (1., 30.), (1., 30.)),
        12 => ConicalSpiral {}.calc_coords(resol, (0., 1.), (-1., 1.)),
        13 => BohemianDome {}.calc_coords(resol, (0., TWO_PI), (0., TWO_PI)),
        14 => AstroidalEllipse {}.calc_coords(resol, (0., TWO_PI), (0., TWO_PI)),
        15 => Apple {}.calc_coords(resol, (0., TWO_PI), (-PI, PI)),
        16 => Ammonite {}.calc_coords(resol, (0., TWO_PI), (0., TWO_PI)),
        17 => PluckerConoid {}.calc_coords(resol, (-2., 2.), (-1., 1.)),
        18 => Cayley {}.calc_coords(resol, (0., 3.), (0., TWO_PI)),
        19 => UpDownShell {}.calc_coords(resol, (-10., 10.), (-10., 10.)),
        20 => ButterFly {}.calc_coords(resol, (0., TWO_PI), (0., TWO_PI)),
        21 => Rose {}.calc_coords(resol, (0., TWO_PI), (0., TWO_PI)),
        22 => Kuen {}.calc_coords(resol, (-4., 4.), (-3.75, 3.75)),
        23 | 24 | 25 | 26 => Tanaka::new(ns - 23).calc_coords(resol, (0., TWO_PI), (0., TWO_PI)),
        _ => (vec![], vec![], vec![]),
    }
}
