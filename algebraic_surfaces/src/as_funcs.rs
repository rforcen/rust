// Funcs for Parametric Surface in MT mode
#![allow(non_snake_case)]
#![allow(dead_code)]
use nalgebra::{Point2, Point3, Vector3};
use rayon::prelude::*;

use crate::evals::*;
use algebraic_surfaces::*;


pub fn calc_coords_mt(func_name : FuncNames, resol: usize) -> Mesh {


    let func = match func_name {
        FuncNames::CAP => Cap_eval,
        FuncNames::BOY => Boy_eval,
        FuncNames::ROMAN => Roman_eval,
        FuncNames::SEASHELL => SeaShell_eval,
        FuncNames::TUDORROSE => TudorRose_eval,
        FuncNames::BREATHER => Breather_eval,
        FuncNames::KLEINBOTTLE => KleinBottle_eval,
        FuncNames::KLEINBOTTLE0 => KleinBottle0_eval,
        FuncNames::BOUR => Bour_eval,
        FuncNames::DINI => Dini_eval,
        FuncNames::ENNEPER => Enneper_eval,
        FuncNames::SCHERK => Scherk_eval,
        FuncNames::CONICALSPIRAL => ConicalSpiral_eval,
        FuncNames::BOHEMIANDOME => BohemianDome_eval,
        FuncNames::ASTROIDALELLIPSE => AstroidalEllipse_eval,
        FuncNames::APPLE => Apple_eval,
        FuncNames::AMMONITE => Ammonite_eval,
        FuncNames::PLUCKERCONOID => PluckerConoid_eval,
        FuncNames::CAYLEY => Cayley_eval,
        FuncNames::UPDOWNSHELL => UpDownShell_eval,
        FuncNames::BUTTERFLY => ButterFly_eval,
        FuncNames::ROSE => Rose_eval,
        FuncNames::KUEN => Kuen_eval,
        FuncNames::TANAKA0 => Tanaka0_eval,
        FuncNames::TANAKA1 => Tanaka1_eval,
        FuncNames::TANAKA2 => Tanaka2_eval,
        FuncNames::TANAKA3 => Tanaka3_eval,
        // _ => Dummy_eval,
    };
    let n_func = func_name as usize;
    let range_u = RANGES[n_func][0];
    let range_v = RANGES[n_func][1];

    let (from_u, dif_u) = (range_u.0, (range_u.1 - range_u.0).abs());
    let (from_v, dif_v) = (range_v.0, (range_v.1 - range_v.0).abs());

    let scale_u = |val: f32| val * dif_u + from_u;
    let scale_v = |val: f32| val * dif_v + from_v;

    // generate coords
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

    // normals
    let normals = coords
        .par_iter()
        .enumerate()
        .map(|(i, p)| {
            fn calc_normal(v0: &Point3<f32>, v1: Point3<f32>, v2: Point3<f32>) -> Vector3<f32> {
                (v2 - v0).cross(&(v1 - v0)) //.normalize()
            }
            let i1 = if i + 1 >= coords.len() { i - 1 } else { i + 1 };
            let i2 = if i + resol >= coords.len() {
                i - resol
            } else {
                i + resol
            };
            calc_normal(p, coords[i1], coords[i2])
        })
        .collect();

    // u,v 's
    let uvs = (0..resol * resol)
        .into_par_iter()
        .map(|i| {
            Point2::new(
                scale_u((i / resol) as f32 * delta),
                scale_v((i % resol) as f32 * delta),
            )
        })
        .collect::<Vec<Point2<f32>>>();

    // indices -> quad 2 2 x trigs
    let indices = (0..2 * resol * resol)
        .into_par_iter()
        .map(|index| {
            let even = index & 1 == 0;
            let index = index / 2;
            if index % resol == resol - 1 || index / resol == resol - 1 {
                Point3::new(0, 0, 0)
            } else {
                if even {
                    Point3::new(
                        (index + 0) as u16,
                        (index + 1) as u16,
                        (index + resol + 1) as u16,
                    )
                } else {
                    Point3::new(
                        (index + 0) as u16,
                        (index + resol + 1) as u16,
                        (index + resol) as u16,
                    )
                }
            }
        })
        .collect::<Vec<Point3<u16>>>();

    (coords, normals, uvs, indices)
}
