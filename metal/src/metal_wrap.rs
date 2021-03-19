// metal wrapper
// metal-wrap.rs
#![allow(dead_code)]

use metal::*;
// use objc::rc::autoreleasepool;
use std::mem;
use std::slice::from_raw_parts;
use std::io::Write;                                                                                                                                                                  
use std::fs::File;   


pub struct Metal {
    device          : Device,
    command_queue   : CommandQueue,
    argument_encoder: ArgumentEncoder,
    arg_buffer      : Buffer,
    kernel          : Function,
}

impl Metal { // 
    pub fn new_src( metal_src : &str, kernel_func : &str ) -> Self { // from include_str!("voronoi.metal");
        let device = Device::system_default().expect("no device found");
        let command_queue = device.new_command_queue();
       
        let library = device
            .new_library_with_source(metal_src, &CompileOptions::new())
            .unwrap();
        let kernel = library.get_function(kernel_func, None).unwrap();

        let argument_encoder = kernel.new_argument_encoder(0); // buff 0 contains a struct w/all params
        let arg_buffer = device.new_buffer(
            argument_encoder.encoded_length(),
            MTLResourceOptions::empty(),
        );
        argument_encoder.set_argument_buffer(&arg_buffer, 0);

        Self { device, command_queue, argument_encoder, arg_buffer, kernel } 
    }

    pub fn new_lib( metal_lib : &str, kernel_func : &str ) -> Self { // from let metal = Metal::new_lib("./src/mandelbrot.metallib", "Mandelbrot");
        let device = Device::system_default().expect("no device found");
        let command_queue = device.new_command_queue();
       
        let library = device
            .new_library_with_file(metal_lib)
            .unwrap();
        let kernel = library.get_function(kernel_func, None).unwrap();

        let argument_encoder = kernel.new_argument_encoder(0); // buff 0 contains a struct w/all params
        let arg_buffer = device.new_buffer(
            argument_encoder.encoded_length(),
            MTLResourceOptions::empty(),
        );
        argument_encoder.set_argument_buffer(&arg_buffer, 0);

        Self { device, command_queue, argument_encoder, arg_buffer, kernel } 
    }

    pub fn new_buffer<T>(&self, data : &Vec<T>) -> Buffer {
        let buff = self.device.new_buffer_with_data(
            unsafe { mem::transmute(data.as_ptr()) },
            (data.len() * mem::size_of::<T>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        );
        buff
    }

    pub fn set_buffer<T>(&self, index : u64, data : &Vec<T>) -> Buffer {
        let buffer  = self.new_buffer(&data);
        self.argument_encoder.set_buffer(index, &buffer, 0);
        buffer
    }

    fn thread_sizes(&self, w : u64) -> (MTLSize, MTLSize){

        let thread_group_count = MTLSize { width : w, height: w, depth: 1, };

        let thread_group_size = MTLSize {
            width: 24, // 384 #cores  
            height: 16,
            depth: 1,
        };
        (thread_group_count, thread_group_size)
    }


    pub fn run(&self, w : usize) {
        let (th_gcnt, th_gsz) = self.thread_sizes(w as u64);

        let (command_buffer, encoder) = self.encoder();
        encoder.set_buffer(0, Some(&self.arg_buffer), 0);

        encoder.dispatch_threads(th_gcnt, th_gsz);
        encoder.end_encoding();
        command_buffer.commit();
        command_buffer.wait_until_completed();
    }

    fn encoder(&self) -> (&CommandBufferRef, &ComputeCommandEncoderRef) {
        let command_buffer : &CommandBufferRef = self.command_queue.new_command_buffer();
        let encoder = command_buffer.new_compute_command_encoder();
        self.new_pipeline(encoder);

        (command_buffer, encoder)
    }

    fn new_pipeline(&self, encoder : &ComputeCommandEncoderRef) {
        let pipeline_state_descriptor = ComputePipelineDescriptor::new();
        pipeline_state_descriptor.set_compute_function(Some(&self.kernel));

        let pl_stat = self.device.new_compute_pipeline_state_with_function(
                pipeline_state_descriptor.compute_function().unwrap(),
            ).unwrap();
        encoder.set_compute_pipeline_state(&pl_stat);
    }   
}

// utils
pub fn write_bin(file_name : &str, image : &[u8]) {
    File::create(file_name).expect("Unable to create file")
    .write_all(image).expect("Unable to write data"); // showbinimage.py w w mandelbrot.bin     
}

pub fn buffer_2_image(buff : &Buffer) -> &'static [u8] {
	unsafe { from_raw_parts(buff.contents() as *mut u8, buff.length() as usize) }
}

#[macro_export]
macro_rules! metal_buffers { // create a vec of metal buffers
    ($metal:ident; $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            let mut _i=0;
            $(
                temp_vec.push($metal.set_buffer(_i, &$x));
                _i+=1;
            )*
            temp_vec
        }
    };
}