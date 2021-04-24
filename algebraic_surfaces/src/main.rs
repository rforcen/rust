// algebraic surfaces

#![allow(dead_code)]
use algebraic_surfaces::*;
mod as_funcs;
use as_funcs::calc_coords_mt;
mod aux_funcs;
mod evals;
mod mt_as;
use mt_as::*;

extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::event::{Action, Key, MouseButton, WindowEvent};
// use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

use na::Vector3;
use std::cell::RefCell;
use std::rc::Rc;

use std::time::Instant;

pub fn gen_nodes(
    resol: usize,
    mesh: &algebraic_surfaces::Mesh,
    window: &mut Window,
    scale: f32,
) -> Vec<SceneNode> {
    let chunk_size = if 1 << 16 < resol * resol {
        1 << 16
    } else {
        resol * resol
    }; // 2^16, u16::MAX+1

    (0..resol * resol)
        .step_by(chunk_size) // size/chunk:size + ramiander
        .map(|i| {
            let (start, end) = (i, (i + chunk_size)); // selected range
            let mesh = Mesh::new(
                mesh.0[start..end].to_vec(),
                mesh.3[start * 2..end * 2].to_vec(), // 2 trigs per quad
                Some(mesh.1[start..end].to_vec()),
                None,
                true,
            );
            let mut node = window.add_mesh(
                Rc::new(RefCell::new(mesh)),
                Vector3::new(scale, scale, scale),
            );
            node.enable_backface_culling(false);
            node
        })
        .collect()
}

fn del_nodes(nodes: &mut Vec<SceneNode>, window: &mut Window) {
    nodes.iter_mut().for_each(|node| window.remove_node(node));
    nodes.clear();
}

fn ui() {
    let resol = 1 << 8; // MUST be a power of 2
    let mut scale = 0.7;

    let mut ns = FuncNames::first(); // initial surface
    let (mut update, mut refresh) = (true, true);

    let mut window = Window::new(&*format!("{}", ns.to_string()));

    let mut nodes = vec![];
    let mut mesh: algebraic_surfaces::Mesh = (vec![], vec![], vec![], vec![]);

    while window.render() {
        if update {
            let t = Instant::now();
            if refresh {
                mesh = calc_coords_mt(ns, resol); // MT rayon mode
                refresh = false
            }
            //
            println!(
                "lap for {} {}x{}: {:?}",
                ns.to_string(),
                resol,
                resol,
                Instant::now() - t
            );

            del_nodes(&mut nodes, &mut window);
            nodes = gen_nodes(resol, &mesh, &mut window, scale);
            window.set_title(&*format!("{}", ns.to_string()));

            update = false;
        }

        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Scroll(_f0, f1, _) => {
                    scale += f1 as f32 / 150.;
                    update = true;
                    event.inhibited = true; // override the default scroll
                }
                WindowEvent::MouseButton(button, Action::Press, _) => {
                    if button == MouseButton::Button3 {
                        update = true;
                        refresh = true;
                        ns = ns.next()
                    }
                }
                WindowEvent::Key(button, Action::Press, _) => {
                    ns = match button {
                        Key::Down => {
                            update = true;
                            refresh = true;
                            ns.prev()
                        }
                        Key::Up | Key::Space => {
                            update = true;
                            refresh = true;
                            ns.next()
                        }
                        _ => {
                            update = false;
                            ns
                        }
                    };
                    event.inhibited = true // override the default keyboard handler
                }
                _ => {}
            }
        }
    }
}


fn main() {
    // ui()
    gen_obj(FuncNames::TUDORROSE, 512)
}
