// Parametric Surface
#![allow(dead_code)]

use nalgebra::{Point2, Point3, Vector3};

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

pub type Mesh = (Vec<Point3<f32>>, Vec<Vector3<f32>>, Vec<Point2<f32>>, Vec<Point3<u16>>);
type Range = (f32, f32);
