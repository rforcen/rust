// Mandelbrot Voronoi - metal
mod metal_wrap;
use metal_wrap::*;
use objc::rc::autoreleasepool;
use std::time::Instant;


fn domain_coloring(){
    static DC_METAL: &str = include_str!("dc.metal");

    static PREDEF_FUNCS : [&str; 19] = ["acos(c(1,2)*log(sin(z*z*z-1)/z))", "c(1,1)*log(sin(z*z*z-1)/z)", "c(1,1)*sin(z)",
                "z + z*z/sin(z*z*z*z-1)", "log(sin(z))", "cos(z)/(sin(z*z*z*z-1))", "z*z*z*z*z*z-1",
                "(z*z-1) * pow((z-c(2,1)),2) / (z*z+c(2,1))", "sin(z)*c(1,2)", "sin(c(1)/z)", "sin(z)*sin(c(1)/z)",
                "c(1)/sin(c(1)/sin(z))", "z", "(z*z+1)/(z*z-1)", "(z*z+c(1))/z", "(z+3)*pow((z+1),2)",
                "pow((z/c(2)),2)*(z+c(1,2))*(z+c(2,2))/(z*z*z)", "(z*z)-0.75-c(0,0.2)",
                "z*sin(z/cos(sin(c(2.2)/z)*tan(z)))"];

    let side = 2000;
    let size   = (side * side) as usize;

    autoreleasepool( || {
        
        let metal = Metal::new_src(&str::replace(DC_METAL, "%%FUNC%%", PREDEF_FUNCS[3])[..], "DomainColoring");
        
        let buffs = metal_buffers![metal; 
            vec![0_u32; size], // image, side
            vec![side] ];

        let t = Instant::now();
        metal.run(side as usize);
        println!("DomainColoring {} x {} = {}, lap: {:.1?}", side, side, size, Instant::now() - t);

        write_bin("dc.bin", buffer_2_image(&buffs[0])); // showbinimage.py 2000 2000 dc.bin 
    });
}

fn mandelbrot() {
    let side = 2000;
    let size   = (side * side) as usize;
    let n_iters : u32 = 200;

    autoreleasepool( || {
        
        let metal = Metal::new_lib("./src/mandelbrot.metallib", "Mandelbrot");
        
        let buffs = metal_buffers![metal; 
            vec![0_u32; size], // image, range, side, n_iters
            vec![-2.0, -2.0, 2.0, 2.0_f32], 
            vec![side], 
            vec![n_iters]];

        let t = Instant::now();
        metal.run(side as usize);
        println!("Mandelbrot {} x {} = {}, lap: {:.1?}", side, side, size,Instant::now() - t);


        write_bin("mandelbrot.bin", buffer_2_image(&buffs[0])); // showbinimage.py 2000 2000 mandelbrot.bin 
    });
}

fn voronoi() {
    static VORONOI: &str = include_str!("voronoi.metal");

    let side = 2000_u32;
    let size   = (side * side) as usize;
    let n_points : u32 = side * 4;

    autoreleasepool( || {
        let metal = Metal::new_src(VORONOI, "Voronoi");
    
        let rand_set = |side : u32| { (0..n_points).map(|_i| rand::random::<u32>() % side).collect::<Vec<u32>>() };
        
        let buffs = metal_buffers![ metal; 
            vec![0_u32; size], // x, y, colors, w, n_points
            rand_set(side), 
            rand_set(side), 
            rand_set(u32::MAX), 
            vec![side], 
            vec![n_points]];
        
        let t = Instant::now();
        metal.run(side as usize);
        println!("Voronoi {} x {} = {}, lap: {:?}", side, side, size, Instant::now() - t);
        
        write_bin("voronoi.bin", buffer_2_image(&buffs[0]));
    });

}

fn main() {
    domain_coloring()
}
