// Vertex funcs
extern crate nalgebra as na;
use na::Point3;

pub fn sub(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
    vec![a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
pub fn add(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
    vec![a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
pub fn addc(a: &Vec<f32>, f: f32) -> Vec<f32> {
    vec![a[0] + f, a[1] + f, a[2] + f]
}
pub fn divc(a: &Vec<f32>, b: f32) -> Vec<f32> {
    vec![a[0] / b, a[1] / b, a[2] / b]
}
pub fn mulc(a: &Vec<f32>, b: f32) -> Vec<f32> {
    vec![a[0] * b, a[1] * b, a[2] * b]
}
pub fn cross(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
    vec![
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
pub fn dot(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
pub fn normalize(a: &Vec<f32>) -> Vec<f32> {
    let l = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
    if l != 0. {
        vec![a[0] / l, a[1] / l, a[2] / l]
    } else {
        vec![0., 0., 0.]
    }
}
pub fn midpoint(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
    divc(&add(a, b), 2.)
}
pub fn one_third(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
    tween(a, b, 1. / 3.)
}
pub fn unit(a:&Vec<f32>) -> Vec<f32> {
    normalize(a)
}

pub fn tween(a: &Vec<f32>, b: &Vec<f32>, t: f32) -> Vec<f32> {
    add(&mulc(a, 1. - t), &mulc(b, t)) // ((1.f - t) * a) + (t * b);
}

pub fn neg(a: &Vec<f32>) -> Vec<f32> {
    vec![-a[0], -a[1], -a[2]]
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
