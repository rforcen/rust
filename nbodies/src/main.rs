mod body;
use body::*;

extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

fn main() {
    // view parameters
    let scale_dist = 0.35;
    let sphere_size = 0.007;
    let time_inc = 3e4;

    let mut univ = BodySet::new();
    univ.read("bodies/awesome");

    let mut window = Window::new("n bodies");
    let mut nodes: Vec<SceneNode> = vec![];

    while window.render() {
        // delete nodes
        nodes.iter_mut().for_each(|node| window.remove_node(node));
        nodes.clear();

        // create nodes
        nodes = univ
            .bodies
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut node = window.add_sphere(sphere_size);
                node.append_translation(&univ.get_translation(scale_dist, i));
                node
            })
            .collect();

        // next epoch
        univ.inc_time(time_inc);
    }
}
