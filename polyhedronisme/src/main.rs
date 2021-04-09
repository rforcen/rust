// polyhedronisme
// https://levskaya.github.io/polyhedronisme/
#![allow(dead_code)]
// extern crate convex_hull;
extern crate kiss3d;
extern crate nalgebra as na;

// use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

use na::{Point3, Vector3};
use std::cell::RefCell;
use std::rc::Rc;

mod polyhedron;
use crate::polyhedron::Polyhedron;
mod transformations;
use crate::transformations::{
    ambo, chamfer, dual, extruden, gyro, hollow, insetn, kis_n, loft, perspectiva1, propellor,
    quinto, reflect, whirl,
};
mod color;
mod vertex;
use crate::vertex::triangularize;
mod flag;

use std::time::Instant;

fn main() {
    test();
}

fn test() {
    fn test_trans() {
        let mut poly = Polyhedron::dodecahedron();
        let mut i = 1;
        loop {
            let t = Instant::now();
            poly = ambo(&poly);
            println!("{}: {} - {:?}", i, poly.faces.len(), Instant::now() - t);
            if poly.faces.len() > 400_000 {
                break;
            }
            i += 1;
        }
    }
    fn test_kisn() {
        let t = Instant::now();

        let mut poly = Polyhedron::dodecahedron();
        poly = kis_n(&perspectiva1(&poly), 0, 0.1);
     
        println!("faces:{} lap:{:?}", poly.faces.len(), Instant::now() - t);
        show_poly(&poly);
    }
    test_kisn()
}

impl Polyhedron {
    pub fn to_nodes(&self, window: &mut Window, scale: f32) -> Vec<SceneNode> {
        let mut poly = vec![];

        for face in &self.faces {
            let verts = face
                .iter()
                .map(|f| {
                    let pp = &self.vertexes[*f as usize];
                    Point3::new(pp[0], pp[1], pp[2])
                })
                .collect();
            let indices = triangularize(face.len());
            let mesh = Mesh::new(verts, indices, None, None, false);
            poly.push(window.add_mesh(
                Rc::new(RefCell::new(mesh)),
                Vector3::new(scale, scale, scale),
            ));
        }
        poly
    }
}

fn show_poly(poly: &Polyhedron) {
    let colors = poly.calc_colors(&poly.calc_normals());

    let mut window = Window::new(&poly.name[..]);
    let scale = 0.4;

    // set colors
    let poly = poly.clone().normalize();
    for (i, n) in poly.to_nodes(&mut window, scale).iter_mut().enumerate() {
        n.set_color(colors[i][0], colors[i][1], colors[i][2]);
    }

    window.set_light(Light::StickToCamera);
    while window.render() {}
}
