use algebraic_surfaces::*;

extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::event::{Action, Key, MouseButton, WindowEvent};
use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

use na::{Point3, Vector3};
use std::cell::RefCell;
use std::rc::Rc;

pub fn gen_nodes(
    resol: usize,
    mesh: &algebraic_surfaces::Mesh,
    window: &mut Window,
    scale: f32,
) -> Vec<SceneNode> {
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

    fn trig_strip3(n: usize) -> Vec<u16> {
        // quad -> trig
        let size = n * (n - 2);
        let ix_vect = [0, 1, n + 1, 0, n + 1, n]; // trig order

        (0..6 * size)
            .map(|index| (ix_vect[index % 6] + (index / 6)) as u16)
            .collect::<Vec<u16>>()
    }

    let mut nodes_by_surface = || -> Vec<SceneNode> {
        let indices = trig_strip3(resol)
            .chunks(3)
            .map(|ix| Point3::new(ix[0], ix[1], ix[2]))
            .collect();
        let mesh = Mesh::new(mesh.0.clone(), indices, Some(mesh.1.clone()), None, false);
        let mut node = window.add_mesh(
            Rc::new(RefCell::new(mesh)),
            Vector3::new(scale, scale, scale),
        );
        node.enable_backface_culling(false);
        vec![node]
    };
    nodes_by_surface()
}

fn del_nodes(nodes: &mut Vec<SceneNode>, window: &mut Window) {
    nodes.iter_mut().for_each(|node| window.remove_node(node));
    nodes.clear();
}

fn main() {
    let resol = 100;
    let mut scale = 0.7;

    let mut ns = 4; // initial surface
    let mut update = true;

    let mut window = Window::new(&*format!("{}", SURF_NAMES[ns]));

    let mut nodes = vec![];
    // window.set_light(Light::StickToCamera);

    while window.render() {
        if update {
            del_nodes(&mut nodes, &mut window);
            let mesh = generate_alg_suf(ns, resol);
            nodes = gen_nodes(resol, &mesh, &mut window, scale);
            window.set_title(&*format!("{}", SURF_NAMES[ns]));
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
                        ns = if ns == N_SURFACES - 1 { 0 } else { ns + 1 }
                    }
                }
                WindowEvent::Key(button, Action::Press, _) => {
                    ns = match button {
                        Key::Down => {
                            update = true;
                            if ns == 0 {
                                N_SURFACES - 1
                            } else {
                                ns - 1
                            }
                        }
                        Key::Up | Key::Space => {
                            update = true;
                            if ns == N_SURFACES - 1 {
                                0
                            } else {
                                ns + 1
                            }
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
