#[macro_use]
extern crate glium;

#[allow(unused_imports)]

use glium::{glutin, Surface};
use std::time::Instant;
use glsl::*;
use crate::glutin::dpi::PhysicalSize;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Image { out_color: u32  }

fn main() {
    let size : i32 = 8000;
    let n_pnts = (size * size) as usize;

    // building the display, ie. the main object
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    // headless
    /*
    let dsize = PhysicalSize {
        width: 800,
        height: 600,
    };
    let context = cb.build_headless(&event_loop, dsize).unwrap();
    let context = unsafe {
        context.treat_as_current()
    };
    let display = glium::backend::glutin::headless::Headless::new(context).unwrap(); // this doesn't work
    */
    let display = glium::Display::new(wb, cb, &event_loop).unwrap(); // this does
     
    // vertex buffer of n_pnts items
    #[derive(Copy, Clone)]
    struct Vertex { position: [u8; 2], } // then smaller the better as the shader doesn't use it at all
    implement_vertex!(Vertex, position);
    let vertex_buffer : glium::VertexBuffer<Vertex> = glium::VertexBuffer::empty(&display, n_pnts).unwrap();
    
    // image vertex w/ out_color
    implement_vertex!(Image, out_color);

    // source w/ out out_color 
    let source =  glium::program::ProgramCreationInput::SourceCode {
        vertex_shader: include_str!("dc.shader"), 
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
  
    let t = Instant::now();

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

    println!("dc {}x{}, lap:{:?} -> generated dc.bin", size, size, Instant::now()-t);
   
    if let Ok(result) = out_buffer.read() { // read result & write to flat bin file (ARGB) u32 size x size
        write_bin("dc.bin", rgba_to_u8(&result))
    }
}

fn rgba_to_u8(image : &Vec<Image>) -> &'static [u8] { // rgba -> u8
    unsafe {
        std::slice::from_raw_parts(
            image.as_ptr() as *const u8,
            image.len() * std::mem::size_of::<u32>(),
        )
    }
}