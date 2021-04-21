// parametric surfaces evals
#![allow(non_snake_case)]

use crate::aux_funcs::*;
use nalgebra::Point3;
use std::f32::consts::PI;

// Cap
pub fn Cap_eval(u: f32, v: f32) -> Point3<f32> {
    Point3::new(
        0.5 * cosf(u) * sinf(2. * v),
        0.5 * sinf(u) * sinf(2. * v),
        0.5 * (sqr(cosf(v)) - sqr(cosf(u)) * sqr(sinf(v))),
    )
}

// Boy
pub fn Boy_eval(u: f32, v: f32) -> Point3<f32> {
    let dv = 2. - sqrtf(2.) * sinf(3. * u) * sinf(2. * v);
    let d1 = cosf(u) * sinf(2. * v);
    let d2 = sqrtf(2.) * sqr(cosf(v));

    Point3::new(
        (d2 * cosf(2. * u) + d1) / dv,
        (d2 * sinf(2. * u) + d1) / dv,
        (3. * sqr(cosf(v))) / (2. - sqrtf(2.) * sinf(3. * u) * sinf(2. * v)),
    )
}

// Roman
pub fn Roman_eval(r: f32, t: f32) -> Point3<f32> {
    let r2 = r * r;
    let rq = sqrtf(1. - r2);
    let st = sinf(t);
    let ct = cosf(t);
    Point3::new(r2 * st * ct, r * st * rq, r * ct * rq)
}

// SeaShell
pub fn SeaShell_eval(u: f32, v: f32) -> Point3<f32> {
    let n = 5.6; // number of turns
    let h = 3.5; // height
    let p = 2.; // power
    let l = 4.; // Controls spike length
    let k = 9.;

    let W = |u: f32| powf(u / (2. * PI), p);

    Point3::new(
        W(u) * cosf(n * u) * (1. + cosf(v)),
        W(u) * sinf(n * u) * (1. + cosf(v)),
        W(u) * (sinf(v) + powf(sinf(v / 2.), k) * l) + h * powf(u / (2. * PI), p + 1.),
    )
}

// TudorRose
pub fn TudorRose_eval(u: f32, v: f32) -> Point3<f32> {
    pub fn r(u: f32, v: f32) -> f32 {
        cosf(v) * cosf(v) * max(fabs(sinf(4. * u)), 0.9 - 0.2 * fabs(cosf(8. * u)))
    }
    Point3::new(
        r(u, v) * cosf(u) * cosf(v),
        r(u, v) * sinf(u) * cosf(v),
        r(u, v) * sinf(v) * 0.5,
    )
}

// Breather
pub fn Breather_eval(u: f32, v: f32) -> Point3<f32> {
    let aa = 0.45; // Values from 0.4 to 0.6 produce sensible results
    let w1 = 1. - aa * aa;
    let w = sqrtf(w1);

    let d = |u, v| aa * (powf(w * cosh(aa * u), 2.) + powf(aa * sinf(w * v), 2.));

    Point3::new(
        -u + (2. * w1 * cosh(aa * u) * sinh(aa * u) / d(u, v)),
        2. * w * cosh(aa * u) * (-(w * cosf(v) * cosf(w * v)) - (sinf(v) * sinf(w * v))) / d(u, v),
        2. * w * cosh(aa * u) * (-(w * sinf(v) * cosf(w * v)) + (cosf(v) * sinf(w * v))) / d(u, v),
    )
}

// KleinBottle
pub fn KleinBottle_eval(u: f32, v: f32) -> Point3<f32> {
    let t = 4.5;
    let tmp = 4. + 2. * cosf(u) * cosf(t * v) - sinf(2. * u) * sinf(t * v);

    Point3::new(
        sinf(v) * tmp,
        cosf(v) * tmp,
        2. * cosf(u) * sinf(t * v) + sinf(2. * u) * cosf(t * v),
    )
}

// KleinBottle0

pub fn KleinBottle0_eval(u: f32, v: f32) -> Point3<f32> {
    let _t = 4.5;

    Point3::new(
        if 0. <= u && u < PI {
            6. * cosf(u) * (1. + sinf(u)) + 4. * (1. - 0.5 * cosf(u)) * cosf(u) * cosf(v)
        } else {
            6. * cosf(u) * (1. + sinf(u)) + 4. * (1. - 0.5 * cosf(u)) * cosf(v + PI)
        },
        if 0. <= u && u < PI {
            16. * sinf(u) + 4. * (1. - 0.5 * cosf(u)) * sinf(u) * cosf(v)
        } else {
            16. * sinf(u)
        },
        4. * (1. - 0.5 * cosf(u)) * sinf(v),
    )
}

// Bour

pub fn Bour_eval(u: f32, v: f32) -> Point3<f32> {
    Point3::new(
        u * cosf(v) - 0.5 * u * u * cosf(2. * v),
        -u * sinf(v) - 0.5 * u * u * sinf(2. * v),
        4. / 3. * powf(u, 1.5) * cosf(1.5 * v),
    )
}

// Dini
pub fn Dini_eval(u: f32, v: f32) -> Point3<f32> {
    let mut psi = 0.3; // aa;
    if psi < 0.001 {
        psi = 0.001
    };
    if psi > 0.999 {
        psi = 0.999
    };
    let psi = psi * PI;
    let sinpsi = sinf(psi);
    let cospsi = cosf(psi);
    let g = (u - cospsi * v) / sinpsi;
    let s = exp(g);
    let r = (2. * sinpsi) / (s + 1. / s);
    let t = r * (s - 1. / s) * 0.5;

    Point3::new(u - t, r * cosf(v), r * sinf(v))
}

// Scherk
pub fn Scherk_eval(u: f32, v: f32) -> Point3<f32> {
    let aa = 0.1;
    let v = v + 0.1;

    Point3::new(u, v, (ln(fabs(cosf(aa * v) / cosf(aa * u)))) / aa)
}

// Enneper
pub fn Enneper_eval(u: f32, v: f32) -> Point3<f32> {
    Point3::new(
        u - u * u * u / 3. + u * v * v,
        v - v * v * v / 3. + v * u * u,
        u * u - v * v,
    )
}

// ConicalSpiral
pub fn ConicalSpiral_eval(u: f32, v: f32) -> Point3<f32> {
    Point3::new(u * v * sinf(15. * v), v, u * v * cosf(15. * v))
}

// BohemianDome
pub fn BohemianDome_eval(u: f32, v: f32) -> Point3<f32> {
    let a = 0.5;
    let b = 1.5;
    let c = 1.;
    Point3::new(a * cosf(u), b * cosf(v) + a * sinf(u), c * sinf(v))
}

// AstroidalEllipse
pub fn AstroidalEllipse_eval(u: f32, v: f32) -> Point3<f32> {
    let a = 1.;
    let b = 1.;
    let c = 1.;
    Point3::new(
        powf(a * cosf(u) * cosf(v), 3.),
        powf(b * sinf(u) * cosf(v), 3.),
        powf(c * sinf(v), 3.),
    )
}

// Apple
pub fn Apple_eval(u: f32, v: f32) -> Point3<f32> {
    let r1 = 4.;
    let r2 = 3.8;
    Point3::new(
        cosf(u) * (r1 + r2 * cosf(v)) + powf(v / PI, 100.),
        sinf(u) * (r1 + r2 * cosf(v)) + 0.25 * cosf(5. * u),
        -2.3 * ln(1. - v * 0.3157) + 6. * sinf(v) + 2. * cosf(v),
    )
}

// Ammonite
pub fn Ammonite_eval(u: f32, v: f32) -> Point3<f32> {
    let w = |u| powf(u / (2. * PI), 2.2);
    let n = 5.6; // number of turns
    let f = 120.0; // wave frequency
    let a = 0.2; // wave amplitude
    Point3::new(
        w(u) * cosf(n * u) * (2. + sinf(v + cosf(f * u) * a)),
        w(u) * sinf(n * u) * (2. + sinf(v + cosf(f * u) * a)),
        w(u) * cosf(v),
    )
}

// PluckerConoid
pub fn PluckerConoid_eval(u: f32, v: f32) -> Point3<f32> {
    Point3::new(u * v, u * sqrtf(1. - sqr(v)), 1. - sqr(v))
}

// Cayley
pub fn Cayley_eval(u: f32, v: f32) -> Point3<f32> {
    Point3::new(
        u * sinf(v) - u * cosf(v),
        sqr(u) * sinf(v) * cosf(v),
        cube(u) * sqr(sinf(v)) * cosf(v),
    )
}

// UpDownShell
pub fn UpDownShell_eval(u: f32, v: f32) -> Point3<f32> {
    Point3::new(
        u * sinf(u) * cosf(v),
        u * cosf(u) * cosf(v),
        u * sinf(v), // -10,10, -10,10
    )
}

// ButterFly
pub fn ButterFly_eval(u: f32, v: f32) -> Point3<f32> {
    let t1 = (exp(cosf(u)) - 2. * cosf(4. * u) + sqr5(sinf(u / 12.))) * sinf(v);

    Point3::new(sinf(u) * t1, cosf(u) * t1, sinf(v))
}

// Rose
pub fn Rose_eval(u: f32, v: f32) -> Point3<f32> {
    let a = 1.;
    let n = 7.;
    Point3::new(
        a * sinf(n * u) * cosf(u) * sinf(v),
        a * sinf(n * u) * sinf(u) * sinf(v),
        cosf(v) / (n * 3.),
    )
}

// Kuen
pub fn Kuen_eval(u: f32, v: f32) -> Point3<f32> {
    Point3::new(
        2. * cosh(v) * (cosf(u) + u * sinf(u)) / (cosh(v) * cosh(v) + u * u),
        2. * cosh(v) * (-u * cosf(u) + sinf(u)) / (cosh(v) * cosh(v) + u * u),
        v - (2. * sinh(v) * cosh(v)) / (cosh(v) * cosh(v) + u * u),
    )
}

// Tanakas 0..3
pub fn Tanaka0_eval(s: f32, t: f32) -> Point3<f32> {
    fn f(v: f32) -> f32 {
        sinf(2. * sinf(sinf(sinf(v))))
    }
    let (a, b1, b2, c, _d, w, h) = (0., 4., 3., 4., 5., 7., 4.);
    Point3::new(
        (a - cosf(t) + w * sinf(b1 * s)) * cosf(b2 * s),
        (a - cosf(t) + w * sinf(b1 * s)) * f(b2 * s),
        h * (w * sinf(b1 * s) + f(t)) + c,
    )
}
pub fn Tanaka1_eval(s: f32, t: f32) -> Point3<f32> {
    fn f(v: f32) -> f32 {
        sinf(2. * sinf(sinf(sinf(v))))
    }
    let (a, b1, b2, c, _d, w, h) = (0., 4., 3., 0., 5., 7., 4.);
    Point3::new(
        (a - cosf(t) + w * sinf(b1 * s)) * cosf(b2 * s),
        (a - cosf(t) + w * sinf(b1 * s)) * f(b2 * s),
        h * (w * sinf(b1 * s) + f(t)) + c,
    )
}
pub fn Tanaka2_eval(s: f32, t: f32) -> Point3<f32> {
    fn f(v: f32) -> f32 {
        sinf(2. * sinf(sinf(sinf(v))))
    }
    let (a, b1, b2, c, _d, w, h) = (0., 3., 4., 8., 5., 5., 2.);
    Point3::new(
        (a - cosf(t) + w * sinf(b1 * s)) * cosf(b2 * s),
        (a - cosf(t) + w * sinf(b1 * s)) * f(b2 * s),
        h * (w * sinf(b1 * s) + f(t)) + c,
    )
}
pub fn Tanaka3_eval(s: f32, t: f32) -> Point3<f32> {
    fn f(v: f32) -> f32 {
        sinf(2. * sinf(sinf(sinf(v))))
    }
    let (a, b1, b2, c, _d, w, h) = (14., 3., 1., 8., 5., 5., 2.);
    Point3::new(
        (a - cosf(t) + w * sinf(b1 * s)) * cosf(b2 * s),
        (a - cosf(t) + w * sinf(b1 * s)) * f(b2 * s),
        h * (w * sinf(b1 * s) + f(t)) + c,
    )
}

pub fn Dummy_eval(_s: f32, _t: f32) -> Point3<f32> {
    Point3::new(0., 0., 0.)
}
