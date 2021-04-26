mod spiral;
use crate::spiral::*;
mod music_freq;

extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::window::Window;

fn w3d() {
    let mut window = Window::new("log spiral");
    let size = window.size();

    let mut sp = Spiral::new(1.2, size.x);

    while window.render() {
        for line in &sp.draw() {
            window.draw_planar_line(&line.0, &line.1, &line.2);
        }
    }
}

fn main() {
    w3d()
}
