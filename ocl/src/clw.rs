// opencl wrapper
#![allow(unused_imports, dead_code)]
extern crate cl3;

use cl3::command_queue::{
    create_command_queue, enqueue_nd_range_kernel, enqueue_read_buffer, enqueue_write_buffer,
    finish, flush, release_command_queue, CL_QUEUE_PROFILING_ENABLE,
};
use cl3::context::{create_context, release_context};
use cl3::device::{
    get_device_ids, get_device_info, DeviceInfo, CL_DEVICE_TYPE_CPU, CL_DEVICE_TYPE_GPU,
};
use cl3::event::{get_event_profiling_info, release_event, wait_for_events, ProfilingInfo};
use cl3::info_type::InfoType;
use cl3::kernel::{create_kernel, release_kernel, set_kernel_arg};
use cl3::memory::{create_buffer, release_mem_object, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
use cl3::platform::{get_platform_ids, get_platform_info, PlatformInfo};
use cl3::program::{
    build_program, create_program_with_source, get_program_build_info, release_program,
    ProgramBuildInfo,
};
use cl3::types::{
    cl_command_queue, cl_context, cl_device_id, cl_event, cl_float, cl_int, cl_kernel, cl_mem,
    cl_platform_id, cl_program, cl_uint, CL_FALSE, CL_TRUE,
};
use libc::{c_void, size_t};
use std::ffi::CString;
use std::mem;
use std::ptr;

// Clw

pub struct Clw {
    platforms: Vec<cl_platform_id>,
    platform_id: cl_platform_id,
    device_id: cl_device_id,
    context: cl_context,
    program: cl_program,
    kernel: cl_kernel,
    queue: cl_command_queue,
    events: Vec<cl_event>,
    kernel_event: cl_event,
}

impl Clw {
    pub fn new() -> Self {
        const NULLPRT: *mut c_void = 0 as *mut c_void;
        Self {
            platforms: get_platform_ids().unwrap(),
            platform_id: NULLPRT,
            device_id: NULLPRT,
            context: NULLPRT,
            program: NULLPRT,
            kernel: NULLPRT,
            queue: NULLPRT,
            kernel_event: NULLPRT,
            events: vec![],
        }
    }
    pub fn with_platform(mut self, index: usize) -> Self {
        self.platform_id = self.platforms[index];
        self
    }
    pub fn with_device(mut self, index: usize) -> Self {
        let device_ids = get_device_ids(self.platform_id, CL_DEVICE_TYPE_GPU).unwrap();
        self.device_id = device_ids[index];
        self
    }
    pub fn get_platform_names(&self) -> Vec<String> {
        self.platforms
            .iter()
            .map(|pid| {
                get_platform_info(*pid, PlatformInfo::CL_PLATFORM_NAME)
                    .unwrap()
                    .to_string()
            })
            .collect()
    }
    // Set up OpenCL compute environment
    pub fn compile(&mut self, source: &str, kernel_name: &str) {
        // Create OpenCL context from the  device
        let device_ids = [self.device_id];
        self.context = create_context(&device_ids, ptr::null(), None, ptr::null_mut()).unwrap();

        // Create the OpenCL program source
        let sources = [source];
        self.program = create_program_with_source(self.context, &sources).unwrap();

        // Build the OpenCL program for the device
        let build_options = CString::default();
        if let Err(rc) = build_program(
            self.program,
            &device_ids,
            &build_options,
            None,
            ptr::null_mut(),
        ) {
            if let InfoType::VecUchar(msg) = get_program_build_info(
                self.program,
                self.device_id,
                ProgramBuildInfo::CL_PROGRAM_BUILD_LOG,
            )
            .unwrap()
            {
                println!("error code:{}\n{:?}", rc, String::from_utf8(msg));
            }
            panic!("compilation error");
        }

        // Create the OpenCL kernel from the program
        let kernel_name = CString::new(kernel_name).unwrap();
        self.kernel = create_kernel(self.program, &kernel_name).unwrap();

        // Create a command_queue for the device
        self.queue =
            create_command_queue(self.context, self.device_id, CL_QUEUE_PROFILING_ENABLE).unwrap();
    }

    pub fn out_buffer<T>(&self, buff: &Vec<T>, index: u32) -> cl_mem {
        let ob = create_buffer(
            self.context,
            CL_MEM_READ_ONLY,
            buff.len() * mem::size_of::<T>(),
            ptr::null_mut(),
        )
        .unwrap();
        // set argument index
        set_kernel_arg(
            self.kernel,
            index,
            mem::size_of::<cl_mem>(),
            &ob as *const _ as *const c_void,
        )
        .unwrap();
        ob
    }
    pub fn in_buffer<T>(&self, buff: &Vec<T>, index: u32) -> (cl_mem, cl_event) {
        let ob = create_buffer(
            self.context,
            CL_MEM_WRITE_ONLY,
            buff.len() * mem::size_of::<T>(),
            ptr::null_mut(),
        )
        .unwrap();
        // Blocking write to OpenCL device buffer
        let buff_write_event = enqueue_write_buffer(
            self.queue,
            ob,
            CL_TRUE,
            0,
            buff.len() * mem::size_of::<T>(),
            buff.as_ptr() as cl_mem,
            0,
            ptr::null(),
        )
        .unwrap();
        // set argument index
        set_kernel_arg(
            self.kernel,
            index,
            mem::size_of::<cl_mem>(),
            &ob as *const _ as *const c_void,
        )
        .unwrap();
        (ob, buff_write_event)
    }

    pub fn set_arg<T: std::any::Any>(&self, value: &T, index: u32) {
        fn is_of_type<T: 'static>(x: &dyn std::any::Any) -> bool {
            x.is::<T>()
        }
        let value = value as &dyn std::any::Any;
        if value.is::<f32>() {
            let value = value.downcast_ref::<f32>().unwrap();
            self.set_farg(*value, index);
        } else {
            if value.is::<i32>() {
                let value = value.downcast_ref::<i32>().unwrap();
                self.set_iarg(*value, index);
            } else {
                if value.is::<[f32; 2]>() {
                    let value = value.downcast_ref::<[f32; 2]>().unwrap();
                    self.set_f2arg(*value, index);
                }
            }
        }
    }

    pub fn set_farg(&self, value: f32, index: u32) {
        set_kernel_arg(
            self.kernel,
            index,
            mem::size_of::<f32>(),
            &value as *const _ as *const c_void,
        )
        .unwrap();
    }
    pub fn set_iarg(&self, value: i32, index: u32) {
        set_kernel_arg(
            self.kernel,
            index,
            mem::size_of::<i32>(),
            &value as *const _ as *const c_void,
        )
        .unwrap();
    }
    pub fn set_larg(&self, value: usize, index: u32) {
        set_kernel_arg(
            self.kernel,
            index,
            mem::size_of::<usize>(),
            &value as *const _ as *const c_void,
        )
        .unwrap();
    }
    pub fn set_f2arg(&self, value: [f32; 2], index: u32) {
        set_kernel_arg(
            self.kernel,
            index,
            mem::size_of::<cl_float>() * 2,
            &value as *const _ as *const c_void,
        )
        .unwrap();
    }
    pub fn set_i2arg(&self, value: [i32; 2], index: u32) {
        set_kernel_arg(
            self.kernel,
            index,
            mem::size_of::<cl_int>() * 2,
            &value as *const _ as *const c_void,
        )
        .unwrap();
    }

    pub fn run(&mut self, n: usize) {
        self.events = Vec::default();
        // Enqueue the OpenCL kernel for execution
        let global_work_sizes: [size_t; 1] = [n];
        let _local_work_sizes: [size_t; 1] = [16]; // required on some intel gpu
        self.kernel_event = enqueue_nd_range_kernel(
            self.queue,
            self.kernel,
            1,
            ptr::null(),
            global_work_sizes.as_ptr(),
            ptr::null(), //local_work_sizes.as_ptr(), //
            0,
            ptr::null(),
        )
        .unwrap();

        // Push the kernel_event to the events wait list so that enqueue_read_buffer
        // can wait on it
        self.events.clear();
        self.events.push(self.kernel_event);
    }
    
    pub fn read<T>(&mut self, buffer: cl_mem, vect: &Vec<T>) {
        // let results = vec![0; n];
        let _read_event = enqueue_read_buffer(
            self.queue,
            buffer,
            CL_FALSE,
            0,
            vect.len() * mem::size_of::<T>(),
            vect.as_ptr() as cl_mem,
            1,
            self.events.as_ptr(),
        )
        .unwrap();
        self.events.clear();
        // Block until all commands on the queue (i.e. the read_event) have completed
        finish(self.queue).unwrap();
        flush(self.queue).unwrap();
        // results
    }

    pub fn free_buffer(&self, buffer: cl_mem) {
        release_mem_object(buffer).unwrap();
    }
    pub fn free_event(&self, ev: cl_event) {
        release_event(ev).unwrap();
    }

    pub fn lap_ns(&self) -> u64 {
        // in ns
        let start_time =
            get_event_profiling_info(self.kernel_event, ProfilingInfo::CL_PROFILING_COMMAND_START)
                .unwrap();
        let end_time =
            get_event_profiling_info(self.kernel_event, ProfilingInfo::CL_PROFILING_COMMAND_END)
                .unwrap();
        end_time.to_ulong() - start_time.to_ulong()
    }
}

impl Drop for Clw {
    fn drop(&mut self) {
        // Release OpenCL objects
        release_event(self.kernel_event).unwrap();

        // Release the OpenCL compute environment
        release_kernel(self.kernel).unwrap();
        release_program(self.program).unwrap();
        release_command_queue(self.queue).unwrap();
        release_context(self.context).unwrap();
    }
}
//

// write buffer to binary file
pub fn write2file<T>(name: &str, image: &Vec<T>) {
    use std::io::prelude::*;
    // create a slice[u8]
    let slice_u8: &[u8] = unsafe {
        std::slice::from_raw_parts(
            image.as_ptr() as *const u8,
            image.len() * std::mem::size_of::<T>(),
        )
    };
    std::fs::File::create(name)
        .unwrap()
        .write_all(slice_u8)
        .unwrap();
}
