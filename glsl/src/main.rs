#[macro_use]
extern crate glium;

#[allow(unused_imports)]

use glium::{glutin, Surface, Rect};
use std::time::Instant;
use rand::prelude::*;
use std::cell::RefCell;
use image::{ImageBuffer, Rgba};

// main image struct 
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Image { out_color: u32  }

fn main() {
    // image vertex w/ out_color
    implement_vertex!(Image, out_color);

    voronoi()
}

#[allow(dead_code)]
fn voronoi() {
    let side = 800;
    let n_points=side / 2;
  
    
    let points = {
        let mut rng = rand::thread_rng();
        (0..n_points * 3).map(|_| rng.gen::<f32>()).collect() }; // x, y, color [0..1]

    let t = Instant::now();
    if let Ok(image) = render_voronoi(side, n_points, &points, include_str!("voronoi.shader")) { // read result & write to flat bin file (ARGB) u32 size x size
        println!("voronoi: {}x{}={}, lap:{:.0} ms -> voronoi.png", side, side, side*side, (Instant::now()-t).as_millis());
  
        write_png("voronoi.png", &image)
    }
}


fn render_voronoi(size : i32, n_points : i32, points : &Vec<f32>, shader : &str) -> Result<Vec<Image>, glium::buffer::ReadError>{
   
    let n_pnts = (size * size) as usize;

    let display = build_display();
     
    // vertex buffer of n_pnts items
    #[derive(Copy, Clone)]
    struct Vertex { position: [u8; 2], } // then smaller the better as the shader doesn't use it at all
    implement_vertex!(Vertex, position);
    let vertex_buffer : glium::VertexBuffer<Vertex> = glium::VertexBuffer::empty(&display, n_pnts).unwrap();

    // source w/ out out_color 
    let source =  glium::program::ProgramCreationInput::SourceCode {
        vertex_shader: shader, 
        tessellation_control_shader: None, 
        tessellation_evaluation_shader: None,
        geometry_shader: None,
        fragment_shader: include_str!("dummy.shader"),
        transform_feedback_varyings: Some((vec!["out_color".to_string()], glium::program::TransformFeedbackMode::Interleaved)), 
        outputs_srgb: false,
        uses_point_size: false,
    };
    let program = glium::Program::new(&display, source).unwrap();

    // alloc w/ n_pnts items
    let mut out_buffer: glium::VertexBuffer<Image> = glium::VertexBuffer::empty(&display, n_pnts).unwrap();
  
    // convert Vec<f32> to texture
    fn vf32_2_texture(display : &glium::Display, vf32 : &Vec <f32>, dims : (u32, u32)) -> glium::texture::Texture2d {
        fn vecf32_to_u8(vf32 : &Vec<f32>) -> Vec<u8> { // vec<f32> -> vec<u8>
            unsafe {
                std::slice::from_raw_parts(
                    vf32.as_ptr() as *const u8,
                    vf32.len() * std::mem::size_of::<f32>(),
                ).to_vec()
            }
        }
        
        let img_points = glium::texture::RawImage2d::from_raw_rgba(vecf32_to_u8(vf32), dims);
        glium::texture::Texture2d::new(display, img_points).unwrap()
    }

    // points are passed as textures -> uniforms use Ref
    let target_points: RefCell<Option<glium::texture::Texture2d>> = RefCell::new(None);
    let mut target_points = target_points.borrow_mut();
    *target_points = Some(vf32_2_texture(&display, points, (n_points as u32, 3))); // *3 is x,y,color
    let target_points = target_points.as_ref().unwrap();


    // generate image in a feedback session
    {    
        let session = glium::vertex::TransformFeedbackSession::new(&display, &program, &mut out_buffer).unwrap();

        let params = glium::DrawParameters {
            transform_feedback: Some(&session),
            .. Default::default() };

        let mut target = display.draw();
        target.draw(
            &vertex_buffer, 
            &glium::index::NoIndices(glium::index::PrimitiveType::Points), 
            &program, &uniform!{
                n_points: n_points, 
                size:[size, size],
                points: target_points
                }, &params).unwrap();
        target.finish().unwrap();
    }

    out_buffer.read()
}

#[allow(dead_code)]
fn domain_coloring() {
    let size = 1000;
    let t = Instant::now();
    if let Ok(image) = render_dc(size, include_str!("dc.shader")) { // read result & write to flat bin file (ARGB) u32 size x size
        println!("dc {}x{}={}, lap:{:.0} ms -> generated dc.png", size, size, size*size, (Instant::now()-t).as_millis());
  
        write_png("dc.png", &image)
    }
}

fn render_dc(size : i32, shader : &str) -> Result<Vec<Image>, glium::buffer::ReadError>{

    let n_pnts = (size * size) as usize;

    let display = build_display();
     
    // vertex buffer of n_pnts items
    #[derive(Copy, Clone)]
    struct Vertex { position: [u8; 2], } // then smaller the better as the shader doesn't use it at all
    implement_vertex!(Vertex, position);
    let vertex_buffer : glium::VertexBuffer<Vertex> = glium::VertexBuffer::empty(&display, n_pnts).unwrap();
   
    // source w/ out out_color 
    let source =  glium::program::ProgramCreationInput::SourceCode {
        vertex_shader: shader, 
        tessellation_control_shader: None, 
        tessellation_evaluation_shader: None,
        geometry_shader: None,
        fragment_shader: include_str!("dummy.shader"),
        transform_feedback_varyings: Some((vec!["out_color".to_string()], glium::program::TransformFeedbackMode::Interleaved)), 
        outputs_srgb: false,
        uses_point_size: false,
    };
    let program = glium::Program::new(&display, source).unwrap();

    // alloc w/ n_pnts items
    let mut out_buffer: glium::VertexBuffer<Image> = glium::VertexBuffer::empty(&display, n_pnts).unwrap();
    

    // generate image ina feedback session
    {    
        let session = glium::vertex::TransformFeedbackSession::new(&display, &program, &mut out_buffer).unwrap();

        let params = glium::DrawParameters {
            transform_feedback: Some(&session),
            .. Default::default() };

        let mut target = display.draw();
        target.draw(
            &vertex_buffer, 
            &glium::index::NoIndices(glium::index::PrimitiveType::Points), 
            &program, &uniform!{size:[size, size]}, &params).unwrap();
        target.finish().unwrap();
    }

    out_buffer.read()
}

// utils

fn build_display() -> glium::Display {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_visible(false);
    let cb = glutin::ContextBuilder::new();
    glium::Display::new(wb, cb, &event_loop).unwrap()
}

fn write_png(name : &str, image: &Vec<Image>) {
    let side = (image.len() as f32).sqrt() as u32;
    let mut imgbuf = ImageBuffer::< Rgba<u8>, Vec<u8> >::new(side, side);
    
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Rgba(image[(x + y * side) as usize].out_color.to_le_bytes());
    }
    imgbuf.save(name).unwrap();
}