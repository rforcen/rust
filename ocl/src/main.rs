// install intel hasswel opencl driver: sudo apt install beignet-opencl-icd
#![allow(dead_code)]

mod clw;
use clw::*;
mod vertex;
use vertex::*;
mod wrl;
use wrl::*;
mod zvm;
use zvm::*;

fn mandelbrot() {
    // use nvidia platform=1, gpu device=0, check w/clinfo
    let mut cl = Clw::new().with_platform(1).with_device(0);
    cl.compile(include_str!("cl/mandelbrot.cl"), "mandelbrot");

    // image size
    let w = 2000;
    let n = w * w; // square image w x w

    // kernel parameters
    let image = vec![0_u32; n];
    let center = [2., 1.];
    let scale = 2.;
    let iter = 150;

    let image_buffer = cl.out_buffer(&image, 0);
    cl.set_f2arg(center, 1);
    cl.set_farg(scale, 2);
    cl.set_iarg(iter, 3);

    // run & read image vec
    cl.run(n);
    cl.read(image_buffer, &image);
    cl.free_buffer(image_buffer);

    println!(
        "lap mandelbrot for {}x{}={}: {:.0} ms",
        w,
        w,
        n,
        cl.lap_ns() as f32 / 1e6
    );
    write2file("mandelbrot.bin", &image);
}
fn voronoi() {
    // use nvidia platform=1, gpu device=0, check w/clinfo
    let mut cl = Clw::new().with_platform(1).with_device(0);
    cl.compile(include_str!("cl/voronoi.cl"), "voronoi");

    // image size
    let w = 1024 * 2;
    let n = w * w; // square image w x w

    let image = vec![0_u32; n];
    // create points buffer (x, y, color, pad) all [0..1]
    #[repr(C)]
    struct Point {
        x: f32,
        y: f32,
        color: u32,
        pad: u32,
    }
    let n_points = w;
    let points: Vec<Point> = (0..n_points)
        .map(|_| Point {
            x: rand::random::<f32>(),
            y: rand::random::<f32>(),
            color: 0xff00_0000 | rand::random::<u32>(),
            pad: 0,
        })
        .collect();

    // kernel parameters (image, points, n_points)
    let image_buffer = cl.out_buffer(&image, 0);
    let (points_buffer, points_event) = cl.in_buffer(&points, 1);
    cl.set_iarg(n_points as i32, 2);

    // run & read image vec
    cl.run(n);
    cl.read(image_buffer, &image);
    // release buffers
    cl.free_buffer(image_buffer);
    cl.free_buffer(points_buffer);
    cl.free_event(points_event);

    println!(
        "lap voronoi for {}x{}x{}={} iters: {:.0} ms",
        w,
        w,
        n_points,
        n * n_points,
        cl.lap_ns() as f32 / 1e6
    );
    write2file("voronoi.bin", &image);
}

fn spherical_harmonics() {
    // use nvidia platform=1, gpu device=0, check w/clinfo
    let mut cl = Clw::new().with_platform(1).with_device(0);
    cl.compile(include_str!("cl/sh.cl"), "spherical_harmonics");

    let resol = 64 * 2; // sh parameters (resolution, color_map, preset code)
    let color_map = 1;
    let n_code = 209;

    let size = (resol * resol) as usize;

    let mesh = vec![Vertex::new(); size];

    // kernel parameters (mesh,  resolution, color_map, preset code)
    let mesh_buffer = cl.out_buffer(&mesh, 0);
    cl.set_iarg(resol, 1);
    cl.set_iarg(color_map, 2);
    cl.set_iarg(n_code, 3);

    // run & read image vec
    cl.run(size);
    cl.read(mesh_buffer, &mesh);
    // release buffers
    cl.free_buffer(mesh_buffer);

    println!(
        "lap spherical harmonics for {}x{}={} iters: {:.0} ms",
        resol,
        resol,
        size,
        cl.lap_ns() as f32 / 1e6
    );
    Wrl::write_indexed_faceset("sh.wrl", &mesh, &generate_faces(resol as u32));
}

fn domain_coloring() {
    // use nvidia platform=1, gpu device=0, check w/clinfo
    let mut cl = Clw::new().with_platform(1).with_device(0);
    cl.compile(include_str!("cl/dc.cl"), "domain_coloring");

    let w = 800;
    let size = (w * w) as usize; // dc params(image)

    let image = vec![0_u32; size];

    // kernel parameters
    let image_buffer = cl.out_buffer(&image, 0);

    // run & read image vec
    cl.run(size);
    cl.read(image_buffer, &image);
    cl.free_buffer(image_buffer);

    println!(
        "lap dc for {}x{}={}: {:.0} ms",
        w,
        w,
        size,
        cl.lap_ns() as f32 / 1e6
    );
    write2file("dc.bin", &image);
}

// zvm compiled code
fn domain_coloring_zvm(expr: &str, w: u32) {
    let zvm = ZVm::new(expr);
    if zvm.ok() {
        let code = zvm.get_code();
        // zvm.walk();

        // use nvidia platform=1, gpu device=0, check w/clinfo
        let mut cl = Clw::new().with_platform(1).with_device(0);
        cl.compile(include_str!("cl/dc_zvm.cl"), "domain_coloring");

        let size = (w * w) as usize; // dc params(image, code)

        let image = vec![0_u32; size];

        // kernel parameters
        let image_buffer = cl.out_buffer(&image, 0);
        let (code_buffer, code_event) = cl.in_buffer(&code, 1); // END zvm code

        // run & read image vec
        cl.run(size);
        cl.read(image_buffer, &image);
        // release buffers
        cl.free_buffer(image_buffer);
        cl.free_buffer(code_buffer);
        cl.free_event(code_event);

        println!(
            "lap dc - zvm for {}x{}={}: {:.0} ms",
            w,
            w,
            size,
            cl.lap_ns() as f32 / 1e6
        );
        write2file("dc_zvm.bin", &image);
    } else {
        panic!("z expression syntax error");
    }
}
fn main() {
    // mandelbrot();
    // voronoi();
    // spherical_harmonics();
    // domain_coloring();
    domain_coloring_zvm(PREDEF_FUNCS[18], 1024 * 2);
}
