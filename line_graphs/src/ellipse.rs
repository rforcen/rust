// Ellipse support
// https://es.wikipedia.org/wiki/Elipse
// foci (p0,p1), ecc -> cx,cy,  a,b

type Complex = num::complex::Complex<f32>;
use std::f32::consts::{E, PI};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

fn foci_ecc_2_center_radii(f0: Complex, f1: Complex, ecc: f32) -> (Complex, f32, f32) {
    fn sqr(x: f32) -> f32 {
        x * x
    }
    let c = (f1 - f0).norm() / 2.;
    let a = c / ecc;
    let b = a * (1. - sqr(ecc)).sqrt();
    let fmid = (f0 + f1) / 2.;

    (fmid, a, b)
}
fn sin(x: f32) -> f32 {
    x.sin()
}
fn cos(x: f32) -> f32 {
    x.cos()
}

pub trait Ellipse {
    fn new(n: u32) -> Self;
    fn get_n(&self) -> u32;
    fn a(k: f32) -> f32;
    fn b(k: f32) -> f32;
    fn c(k: f32) -> f32;
    fn d(k: f32) -> f32;
    fn f0(k: f32) -> Complex;
    fn f1(k: f32) -> Complex;

    fn generate_svg(&self, path: &str) {
        self._generate_svg(path, 4000., 4000., 3.5, 3.5 / 2., 0.0)
    }
    fn _generate_svg(
        &self,
        path: &str,
        width: f32,
        height: f32,
        scale_factor: f32,
        x_offset: f32,
        y_offset: f32,
    ) {
        let mut buff_write = BufWriter::new(File::create(path).unwrap());
        let scale = width / scale_factor;
        buff_write
            .write(
                &format!(
                    "
            <svg width='{w}' height='{h}' fill='none' stroke='blue' stroke-width='{sw}' >
    \t<rect width='{w}' height='{h}' style='fill:white' />\n\n",
                    w = width,
                    h = height,
                    sw = 0.3
                )
                .as_bytes(),
            )
            .unwrap();
        for k in 1..self.get_n() {
            let k = k as f32;
            let (fmid, a, b) = foci_ecc_2_center_radii(Self::f0(k), Self::f1(k), Self::d(k));
            // println!("<ellipse cx='{}' cy='{}' rx='{}' ry='{}' />", cx, cy, a, b);
            buff_write
                .write(
                    &format!(
                        "<ellipse cx='{:.0}' cy='{:.0}' rx='{:.0}' ry='{:.0}'/>\n",
                        (x_offset + fmid.re) * scale,
                        height / 2. - (y_offset + fmid.im) * scale,
                        a * scale,
                        b * scale
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
        buff_write.write(&format!("</svg>").as_bytes()).unwrap();
        buff_write.flush().unwrap();
    }
}

// ellipse 1
pub struct Ellipse1 {
    n: u32,
}
impl Ellipse for Ellipse1 {
    fn new(n: u32) -> Self {
        Ellipse1 { n }
    }
    fn get_n(&self) -> u32 {
        self.n
    }
    fn a(k: f32) -> f32 {
        (-3. / 2.) * sin(2. * PI * k / 2500.).powi(3)
            + (3. / 10.) * sin(2. * PI * k / 2500.).powi(7)
    }
    fn b(k: f32) -> f32 {
        sin((2. * PI * k / 1875.) + (PI / 6.))
            + (1. / 4.) * (sin((2. * PI * k / 1875.) + (PI / 6.))).powi(3)
    }
    fn c(k: f32) -> f32 {
        (2. / 15.) - (1. / 8.) * cos(PI * k / 625.)
    }
    fn d(k: f32) -> f32 {
        (49. / 50.) - (1. / 7.) * (sin(4. * PI * k / 2500.)).powi(4)
    }

    fn f0(k: f32) -> Complex {
        Self::a(k)
            + Complex::i() * Self::b(k)
            + Self::c(k) * (68. * PI * Complex::i() * k / 2500.).expf(E)
    }
    fn f1(k: f32) -> Complex {
        Self::a(k) + Complex::i() * Self::b(k)
            - Self::c(k) * (68. * PI * Complex::i() * k / 2500.).expf(E)
    }
}

pub fn ellipse1() {
    let e: Ellipse1 = Ellipse::new(2500);
    e.generate_svg("svg/ellipse_1.svg");
}

pub struct Ellipse2 {
    n: u32,
}
impl Ellipse for Ellipse2 {
    fn new(n: u32) -> Self {
        Ellipse2 { n }
    }
    fn get_n(&self) -> u32 {
        self.n
    }

    fn a(k: f32) -> f32 {
        (3. / 4.) * sin(2. * PI * k / 8000.) * cos(6. * PI * k / 8000.)
            + (1. / 4.) * sin(28. * PI * k / 8000.)
    }
    fn b(k: f32) -> f32 {
        (3. / 4.) * cos(2. * PI * k / 8000.) * cos(8. * PI * k / 8000.)
            + (1. / 4.) * cos(28. * PI * k / 8000.)
    }
    fn c(k: f32) -> f32 {
        (1. / 18.) + (1. / 20.) * cos(24. * PI * k / 8000.)
    }
    fn d(k: f32) -> f32 {
        (49. / 50.) - (1. / 7.) * (sin(10. * PI * k / 8000.)).powi(4)
    }

    fn f0(k: f32) -> Complex {
        Self::a(k)
            + Complex::i() * Self::b(k)
            + Self::c(k) * (300. * PI * Complex::i() * k / 8000.).expf(E)
    }
    fn f1(k: f32) -> Complex {
        Self::a(k) + Complex::i() * Self::b(k)
            - Self::c(k) * (300. * PI * Complex::i() * k / 8000.).expf(E)
    }
}

pub fn ellipse2() {
    let e: Ellipse2 = Ellipse::new(8000);
    e.generate_svg("svg/ellipse_2.svg");
}

pub struct EllipseRing {
    n: u32,
}
impl Ellipse for EllipseRing {
    fn new(n: u32) -> Self {
        EllipseRing { n }
    }
    fn get_n(&self) -> u32 {
        self.n
    }

    fn a(k: f32) -> f32 {
        cos(28. * PI * k / 5600.).powi(3)
    }
    fn b(k: f32) -> f32 {
        sin(28. * PI * k / 5600.)
            + (1. / 4.) * (cos((14. * PI * k / 5600.) - (7. * PI / 4.))).powi(18)
    }
    fn c(k: f32) -> f32 {
        (1. / 70.) + (1. / 6.) + (1. / 6.) * sin(28. * PI * k / 5600.)
    }
    fn d(k: f32) -> f32 {
        (399. / 400.) - (1. / 6.) * (sin(28. * PI * k / 5600.)).powi(8)
    }

    fn f0(k: f32) -> Complex {
        Self::a(k)
            + Complex::i() * Self::b(k)
            + Self::c(k) * (44. * PI * Complex::i() * k / 5600.).expf(E)
    }
    fn f1(k: f32) -> Complex {
        Self::a(k) + Complex::i() * Self::b(k)
            - Self::c(k) * (44. * PI * Complex::i() * k / 5600.).expf(E)
    }
}

pub fn ellipse_ring() {
    let e: EllipseRing = Ellipse::new(5600);
    e.generate_svg("svg/ellipse_ring.svg");
}


pub fn generate_all() {
    ellipse1();
    ellipse2();
    ellipse_ring();
}