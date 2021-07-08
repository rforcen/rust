
const PROGRAM_SOURCE: &str = "
kernel void saxpy_float (
    global float* z,
    global float const* x,
    global float const* y,
    float a) 
{
    double cd;
    float ff;

    size_t i = get_global_id(0);
    z[i] = a * x[i] + y[i];
}";

const KERNEL_NAME: &str = "saxpy_float";

fn test_opencl_1_2_example() {
    // get platforms
    let platform_ids = get_platform_ids().unwrap();
    assert!(0 < platform_ids.len());
    println!("# platforms: {}", platform_ids.len());

    for (i, platform_id) in platform_ids.iter().enumerate() {
        let platform_name =
            get_platform_info(*platform_id, PlatformInfo::CL_PLATFORM_NAME).unwrap();
        println!("Platform {} : {}", i, platform_name);
    }
    println!("-----------------------");

    // Choose the first platform
    let platform_id = platform_ids[1];
    let platform_name = get_platform_info(platform_id, PlatformInfo::CL_PLATFORM_NAME).unwrap();
    println!("Platform Name: {}", platform_name);

    // get devices of selected platform
    let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_GPU).unwrap();
    assert!(0 < device_ids.len());

    // Choose the first GPU device
    let device_id = device_ids[0];
    let vendor_name = get_device_info(device_id, DeviceInfo::CL_DEVICE_VENDOR).unwrap();
    println!("OpenCL device vendor name: {}", vendor_name);
    let vendor_id = get_device_info(device_id, DeviceInfo::CL_DEVICE_VENDOR_ID).unwrap();
    println!("OpenCL device vendor id: {:X}", vendor_id.to_uint());

    /////////////////////////////////////////////////////////////////////
    // Set up OpenCL compute environment

    // Create OpenCL context from the first device
    let device_ids = [device_id];
    let context = create_context(&device_ids, ptr::null(), None, ptr::null_mut()).unwrap();

    // Create the OpenCL program source
    let sources = [PROGRAM_SOURCE];
    let program = create_program_with_source(context, &sources).unwrap();

    // Build the OpenCL program for the device
    let build_options = CString::default();
    if let Err(rc) = build_program(program, &device_ids, &build_options, None, ptr::null_mut()) {
        if let InfoType::VecUchar(msg) =
            get_program_build_info(program, device_id, ProgramBuildInfo::CL_PROGRAM_BUILD_LOG)
                .unwrap()
        {
            println!("error code:{}\n{:?}", rc, String::from_utf8(msg));
        }
        panic!("compilation error");
    }

    // Create the OpenCL kernel from the program
    let kernel_name = CString::new(KERNEL_NAME).unwrap();
    let kernel = create_kernel(program, &kernel_name).unwrap();

    // Create a command_queue for the device
    let queue = create_command_queue(context, device_id, CL_QUEUE_PROFILING_ENABLE).unwrap();

    /////////////////////////////////////////////////////////////////////
    // Process some data

    // The input data
    const ARRAY_SIZE: usize = 10000;
    let ones: [cl_float; ARRAY_SIZE] = [1.0; ARRAY_SIZE];
    let mut sums: [cl_float; ARRAY_SIZE] = [0.0; ARRAY_SIZE];
    for i in 0..ARRAY_SIZE {
        sums[i] = 1.0 + 1.0 * i as cl_float;
    }

    // Create OpenCL device buffers for input and output data
    let x = create_buffer(
        context,
        CL_MEM_WRITE_ONLY,
        ARRAY_SIZE * mem::size_of::<cl_float>(),
        ptr::null_mut(),
    )
    .unwrap();
    let y = create_buffer(
        context,
        CL_MEM_WRITE_ONLY,
        ARRAY_SIZE * mem::size_of::<cl_float>(),
        ptr::null_mut(),
    )
    .unwrap();
    let z = create_buffer(
        context,
        CL_MEM_READ_ONLY,
        ARRAY_SIZE * mem::size_of::<cl_float>(),
        ptr::null_mut(),
    )
    .unwrap();

    // Blocking write to OpenCL device buffer
    let x_write_event = enqueue_write_buffer(
        queue,
        x,
        CL_TRUE,
        0,
        ones.len() * mem::size_of::<cl_float>(),
        ones.as_ptr() as cl_mem,
        0,
        ptr::null(),
    )
    .unwrap();
    // Non-blocking write to OpenCL device buffer
    let y_write_event = enqueue_write_buffer(
        queue,
        y,
        CL_FALSE,
        0,
        sums.len() * mem::size_of::<cl_float>(),
        sums.as_ptr() as cl_mem,
        0,
        ptr::null(),
    )
    .unwrap();

    // wait for y_write_event
    let mut events: Vec<cl_event> = Vec::default();
    events.push(y_write_event);
    wait_for_events(&events).unwrap();

    // a value for the kernel function
    let a: cl_float = 300.0;

    // Set up the arguments to call the OpenCL kernel function
    // i.e. the x, y & z buffers and the constant value, a
    set_kernel_arg(
        kernel,
        0,
        mem::size_of::<cl_mem>(),
        &z as *const _ as *const c_void,
    )
    .unwrap();
    set_kernel_arg(
        kernel,
        1,
        mem::size_of::<cl_mem>(),
        &x as *const _ as *const c_void,
    )
    .unwrap();
    set_kernel_arg(
        kernel,
        2,
        mem::size_of::<cl_mem>(),
        &y as *const _ as *const c_void,
    )
    .unwrap();
    set_kernel_arg(
        kernel,
        3,
        mem::size_of::<cl_float>(),
        &a as *const _ as *const c_void,
    )
    .unwrap();

    // Enqueue the OpenCL kernel for execution
    let global_work_sizes: [size_t; 1] = [ARRAY_SIZE];
    let local_work_sizes: [size_t; 1] = [16];
    let kernel_event = enqueue_nd_range_kernel(
        queue,
        kernel,
        1,
        ptr::null(),
        global_work_sizes.as_ptr(),
        local_work_sizes.as_ptr(), //ptr::null(),
        0,
        ptr::null(),
    )
    .unwrap();

    // Push the kernel_event to the events wait list so that enqueue_read_buffer
    // can wait on it
    events.clear();
    events.push(kernel_event);

    // Create a results array to hold the results from the OpenCL device z buffer
    // and enqueue a read command to read the device buffer into the array
    // after the kernel event completes.
    let results: [cl_float; ARRAY_SIZE] = [0.0; ARRAY_SIZE];
    let read_event = enqueue_read_buffer(
        queue,
        z,
        CL_FALSE,
        0,
        results.len() * mem::size_of::<cl_float>(),
        results.as_ptr() as cl_mem,
        1,
        events.as_ptr(),
    )
    .unwrap();
    events.clear();

    // Block until all commands on the queue (i.e. the read_event) have completed
    finish(queue).unwrap();

    // Test and print the results from OpenCL
    assert_eq!(10300.0, results[ARRAY_SIZE - 1]);
    println!("results back: {}", results[ARRAY_SIZE - 1]);

    let start_time =
        get_event_profiling_info(kernel_event, ProfilingInfo::CL_PROFILING_COMMAND_START).unwrap();
    let end_time =
        get_event_profiling_info(kernel_event, ProfilingInfo::CL_PROFILING_COMMAND_END).unwrap();
    let duration = end_time.to_ulong() - start_time.to_ulong();
    println!("kernel execution duration (ns): {}", duration);

    /////////////////////////////////////////////////////////////////////
    // Release OpenCL objects

    release_event(x_write_event).unwrap();
    release_event(y_write_event).unwrap();
    release_event(kernel_event).unwrap();
    release_event(read_event).unwrap();
    release_mem_object(z).unwrap();
    release_mem_object(y).unwrap();
    release_mem_object(x).unwrap();

    // Release the OpenCL compute environment
    release_kernel(kernel).unwrap();
    release_program(program).unwrap();
    release_command_queue(queue).unwrap();
    release_context(context).unwrap();
}
