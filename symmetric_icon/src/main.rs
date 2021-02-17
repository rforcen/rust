// Symmetric Icon w/ minimal ui

mod symm_icon;
use symm_icon::*;

use druid::widget::prelude::*;
use druid::{
	AppLauncher, Data, WindowDesc, Lens,
	piet::{ImageBuf, ImageFormat},
	widget::Image,
};

use std::time::Instant;


// UI
#[derive(Clone, Default, Data, Lens)]
struct UI {}

// symmetric icon build parameters
const PRESET : usize = 1;
const W_SIZE : usize = 800*2;
const N_ITERS : usize = 80_000_000;
const COLOR_SET : u32 = 0;


fn ui_builder() -> impl Widget<UI> {
   
    fn load_image(bytes : &[u8], (w,h) : (usize, usize)) -> ImageBuf {
        ImageBuf::from_raw(bytes, ImageFormat::RgbaSeparate, w, h)
    }

    let t = Instant::now();

    let mut symicn = SymmetricIcons::new(W_SIZE, W_SIZE, COLOR_SET);
    let (img, size) = symicn.build(PRESET, N_ITERS);

    println!("size : {} x {}, iters: {}, build lap : {:?}", W_SIZE, W_SIZE, N_ITERS, Instant::now() - t);

    Image::new(load_image(img, size))
}

fn main() {
    // _test_array2d();

    let main_window = WindowDesc::new(ui_builder)
        .title("Symmetric Icons")
        .window_size((400., 400.));

    AppLauncher::with_window(main_window)
        // .delegate(Delegate {})
        .launch(UI {})
        .expect("launch failed");
}
