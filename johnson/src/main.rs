extern crate kiss3d;
extern crate nalgebra as na;

// use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

use na::{Point3, Vector3};
use std::cell::RefCell;
use std::rc::Rc;

mod johnson_data;
use crate::johnson_data::johnsons_vec;
mod strobe;
use crate::strobe::random_colors;

pub fn gen_nodes(jp: &Vec<Vec<Vec<f32>>>, window: &mut Window, scale: f32) -> Vec<SceneNode> {
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
    let colors = random_colors();

    jp.iter().map(|face| {
        let verts = face.iter().map(|f| Point3::new(f[0], f[1], f[2])).collect();
        let indices = triangularize(face.len());
        let mesh = Mesh::new(verts, indices, None, None, false);
        let mut node = window.add_mesh(
            Rc::new(RefCell::new(mesh)),
            Vector3::new(scale, scale, scale),
        );
        let color = &colors[face.len()];
        node.set_color(color[0], color[1], color[2]);
        node
    }).collect::<Vec<SceneNode>>()
}

fn del_nodes(nodes: &mut Vec<SceneNode>, window: &mut Window) {
    nodes.iter_mut().for_each(|node| window.remove_node(node));
    nodes.clear();
}

fn main() {
    let jm = johnsons_vec();

    let mut nj = 0;
    let mut window = Window::new(&*format!("j{}", nj));
    let scale = 0.3;

    let mut nodes = gen_nodes(&jm[nj], &mut window, scale);
    window.set_light(Light::StickToCamera);

    while window.render() {
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(button, Action::Press, _) => {
                    let mut update = true;
                    nj = match button {
                        Key::Down => {
                            if nj == 0 {
                                jm.len() - 1
                            } else {
                                nj - 1
                            }
                        }
                        Key::Up | Key::Space => {
                            if nj == jm.len() - 1 {
                                0
                            } else {
                                nj + 1
                            }
                        }
                        _ => {
                            update = false;
                            nj
                        }
                    };
                    if update {
                        del_nodes(&mut nodes, &mut window);
                        nodes = gen_nodes(&jm[nj], &mut window, scale);
                        window.set_title(&*format!("j{}", nj));
                    }
                    event.inhibited = true // override the default keyboard handler
                }
                _ => {}
            }
        }
    }
}
