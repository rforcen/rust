use kiss3d::*;
use nalgebra::*;

fn ui() {
    let lorentz = Lorentz::new();
    lorentz.write_wrl("lorentz.wrl");

    let mut window = window::Window::new("Lorentz attractor");

    window.set_light(light::Light::StickToCamera);
    while window.render() {
        for p in &lorentz.pnts {
            window.draw_point(p, &Point3::new(1., 1., 1.))
        }
    }
}