// Parametric Surface
#![allow(dead_code)]

use nalgebra::{Point2, Point3, Vector3};
use std::f32::consts::PI;

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

pub type Mesh = (
    Vec<Point3<f32>>,
    Vec<Vector3<f32>>,
    Vec<Point2<f32>>,
    Vec<Point3<u16>>,
);
type Range = (f32, f32);

pub const RANGES: [[(f32, f32); 2]; 27] = [
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
    [(0., TWO_PI), (0., TWO_PI)],
    [(0., TWO_PI), (0., TWO_PI)],
    [(0., TWO_PI), (0., TWO_PI)],
];
