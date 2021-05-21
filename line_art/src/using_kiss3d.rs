use kiss3d::window::Window;
use line_art::*;
use nalgebra::geometry::Translation2;

fn show_in_window() {
    let mut parser = Parser::new(BUTTERFLY1);
    assert!(parser.compile());

    // parser.print_code();
    let circs = parser.generate_circles();

    let scale = 200.;
    let mut window = Window::new("line art");

    for circ in &circs {
        let mut c = window.add_circle(circ.2 * scale);
        c.append_translation(&Translation2::new(circ.0 * scale, circ.1 * scale))
    }
    while window.render() {}
}