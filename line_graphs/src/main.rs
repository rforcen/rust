#![allow(dead_code)]

mod spiral;
use crate::spiral::*;
mod harmonigraph3d;
use crate::harmonigraph3d::*;
mod harmonigraph;
use crate::harmonigraph::*;
mod music_freq;

use kiss3d::window::Window;

fn do_spiral() {
    let mut window = Window::new("log spiral");

    let mut spiral = Spiral::new().with_delta(1.3);
    while window.render() {
        for line in &mut spiral.generate_lines().windows(2) {
            window.draw_planar_line(&line[0].0, &line[1].0, &line[0].1);
        }
    }
}

fn do_hg() {
    let mut window = Window::new("harmonigraph");
    let mut hg = HarmoniGraph::new().with_preset(4).with_scale(0.25);
    let size = window.size();

    while window.render() {
        for line in &mut hg.generate_lines(size.x).windows(2) {
            window.draw_planar_line(&line[0].0, &line[1].0, &line[0].1);
        }
        hg.next();
    }
}

fn do_hg3d() {
    let mut window = Window::new("harmonigraph 3d");
    let mut hg3d = HarmoniGraph3D::new().with_preset(8).with_scale(0.25);

    while window.render() {
        for line in &mut hg3d.generate_lines().windows(2) {
            window.draw_line(&line[0].0, &line[1].0, &line[0].1);
        }
        hg3d.next();
    }
}

fn main() {
    // do_spiral()
    // do_hg3d() 
    do_hg()
}
