// lib.rs
use nalgebra::{Point2, Point3};
use std::f32::consts::PI;

pub type LinesVec = Vec<(Point2<f32>, Point3<f32>)>; // from line, to line, color
pub type LinesVec3D = Vec<(Point3<f32>, Point3<f32>)>; // from, color

pub const PI2: f32 = PI * 2.;
