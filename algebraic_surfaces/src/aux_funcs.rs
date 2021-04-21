// aux_funcs -> f32 wrappers
#![allow(dead_code)]
use nalgebra::*;
use rayon::prelude::*;

pub fn trig_strip3(n: usize) -> Vec<u32> {
    // quad -> 2 x trigs
    let size = n * n;
    let ix_vect = [0, 1, n + 1, 0, n + 1, n]; // trig order

    (0..6 * size)
        .into_par_iter()
        .map(|index| {
            if (index / 6) % n == n - 1 || (index / 6) / n == n - 1 {
                0 // edges are already solved
            } else {
                (ix_vect[index % 6] + (index / 6)) as u32
            }
        })
        .collect::<Vec<u32>>()
}

pub fn quad2trigs(n: usize) -> Vec<Point3<u16>> {
    // quad -> 2 x trigs

    (0..2 * n * n)
        .into_par_iter()
        .map(|index| {
            let even = index & 1 == 0;
            let index = index / 2;
            if index % n == n - 1 || index / n == n - 1 {
                Point3::new(0, 0, 0)
            } else {
                if even {
                    Point3::new(
                        (index + 0) as u16,
                        (index + 1) as u16,
                        (index + n + 1) as u16,
                    )
                } else {
                    Point3::new(
                        (index + 0) as u16,
                        (index + n + 1) as u16,
                        (index + n) as u16,
                    )
                }
            }
        })
        .collect::<Vec<Point3<u16>>>()
}

pub fn triangularize(n_sides: usize) -> Vec<Point3<u16>> {
    match n_sides {
        3 => vec![Point3::new(0, 1, 2)],
        4 => vec![Point3::new(0, 1, 2), Point3::new(0, 2, 3)],
        5 => vec![
            Point3::new(0, 1, 2),
            Point3::new(0, 2, 3),
            Point3::new(0, 3, 4),
        ],
        6 => vec![
            Point3::new(0, 1, 2),
            Point3::new(0, 2, 3),
            Point3::new(0, 3, 4),
            Point3::new(0, 4, 5),
        ],
        _ => {
            // generate n_sides polygon set of trig index coords
            (0..(n_sides - 2))
                .map(|i| Point3::new(0, i as u16 + 1, i as u16 + 2))
                .collect::<Vec<Point3<u16>>>()
        }
    }
}

pub fn sinf(x: f32) -> f32 {
    x.sin()
}
pub fn cosf(x: f32) -> f32 {
    x.cos()
}
pub fn sinh(x: f32) -> f32 {
    x.sinh()
}
pub fn cosh(x: f32) -> f32 {
    x.cosh()
}
pub fn sqr(x: f32) -> f32 {
    x * x
}
pub fn cube(x: f32) -> f32 {
    x * x * x
}
pub fn sqr5(x: f32) -> f32 {
    x * x * x * x * x
}
pub fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}
pub fn fabs(x: f32) -> f32 {
    x.abs()
}
pub fn exp(x: f32) -> f32 {
    x.exp()
}
pub fn ln(x: f32) -> f32 {
    x.ln()
}
pub fn max(x: f32, y: f32) -> f32 {
    x.max(y)
}
pub fn powf(x: f32, y: f32) -> f32 {
    x.powf(y)
}
