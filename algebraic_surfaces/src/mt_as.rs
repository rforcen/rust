// Algebraic Surfaces MT
#![allow(dead_code)]
use rayon::prelude::*;

use crate::evals::*;
use algebraic_surfaces::*;
use nalgebra::{Point2, Point3, Vector3};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

struct Mesh {
    vertex: Point3<f32>,
    normal: Vector3<f32>,
    uv: Point2<f32>,
}

pub struct ASMesh {
    resol: usize,
    mesh: Vec<Mesh>,
}

impl ASMesh {
    pub fn new(func_name: FuncNames, resol: usize) -> ASMesh {
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

        let range_u = RANGES[func_name as usize][0];
        let range_v = RANGES[func_name as usize][1];
        let (from_u, dif_u) = (range_u.0, (range_u.1 - range_u.0).abs());
        let (from_v, dif_v) = (range_v.0, (range_v.1 - range_v.0).abs());

        let scale_u = |val: f32| val * dif_u + from_u;
        let scale_v = |val: f32| val * dif_v + from_v;

        let delta = 1. / resol as f32;

        let size = resol * resol;
        // vertices & uv's
        let mut mesh = (0..size)
            .into_par_iter()
            .map(|i| {
                let (u, v) = (
                    scale_u((i / resol) as f32 * delta),
                    scale_v((i % resol) as f32 * delta),
                );
                let vertex = func(u, v);
                let normal = Vector3::zeros();
                let uv = Point2::new(u, v);

                Mesh { vertex, normal, uv }
            })
            .collect::<Vec<_>>();

        // normals
        let normals = (0..size)
            .into_par_iter()
            .map(|i| {
                fn calc_normal(
                    v0: &Point3<f32>,
                    v1: &Point3<f32>,
                    v2: &Point3<f32>,
                ) -> Vector3<f32> {
                    (v2 - v0).cross(&(v1 - v0)).normalize()
                }
                let i1 = if i + 1 >= size { i - 1 } else { i + 1 };
                let i2 = if i + resol >= size {
                    i - resol
                } else {
                    i + resol
                };
                calc_normal(&mesh[i].vertex, &mesh[i1].vertex, &mesh[i2].vertex)
            })
            .collect::<Vec<_>>();

        // copy normals to mesh
        mesh.par_iter_mut()
            .enumerate()
            .for_each(|(i, m)| m.normal = normals[i]);
        // for (i, m) in mesh.iter_mut().enumerate() { // ST version works better in small set of data
        //     m.normal = normals[i]
        // }

        ASMesh { resol, mesh }
    }

    pub fn write_obj(&self, path: &str) -> std::io::Result<()> {
        let mut buff_write = BufWriter::new(File::create(path).unwrap());

        for m in &self.mesh {
            // vertex
            buff_write.write(
                &format!("v {:.3} {:.3} {:.3}\n", m.vertex.x, m.vertex.y, m.vertex.z).as_bytes(),
            )?;
        }
        for m in &self.mesh {
            // normal
            buff_write.write(
                &format!("vn {:.2} {:.2} {:.2}\n", m.normal.x, m.normal.y, m.normal.z).as_bytes(),
            )?;
        }
        for m in &self.mesh {
            // uv's
            buff_write.write(&format!("vt {:.3} {:.3}\n", m.uv.x, m.uv.y).as_bytes())?;
        }
        for i in 0..self.resol - 1 {
            for j in 0..self.resol - 1 {
                // faces -> quads
                let f = i * self.resol + j + 1; // faces start @1
                buff_write.write(
                    &format!(
                        "f {} {} {} {}\n",
                        f,
                        f + 1,
                        f + self.resol + 1,
                        f + self.resol
                    )
                    .as_bytes(),
                )?;
            }
        }
        buff_write.flush()?;
        Ok(())
    }
}
