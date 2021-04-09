// color.rs

use rand::random;

pub struct Color {}

impl Color {
    pub fn random_pallete(n: usize) -> Vec<Vec<f32>> {
        (0..n).map(|_| Self::rnd()).collect()
    }

    fn hsl2rgb(h: f32, s: f32, l: f32) -> Vec<f32> {
        if s == 0. {
            vec![l, l, l] // acromatic
        } else {
            let q = if l < 0.5 { l * (1. + s) } else { l + s - l * s };
            let p = 2. * l - q;
            vec![
                Self::hue2rgb(p, q, h + 1. / 3.),
                Self::hue2rgb(p, q, h),
                Self::hue2rgb(p, q, h - 1. / 3.),
            ]
        }
    }
    fn hue2rgb(p: f32, q: f32, t: f32) -> f32 {
        let mut t = t;
        if t < 0. {
            t += 1.
        }
        if t > 1. {
            t -= 1.
        }
        if t < 1. / 6. {
            return p + (q - p) * 6. * t;
        }
        if t < 1. / 2. {
            return q;
        }
        if t < 2. / 3. {
            return p + (q - p) * (2. / 3. - t) * 6.;
        }
        p
    }
    fn rnd() -> Vec<f32> {
        fn rndf32() -> f32 {
            random::<f32>()
        }
        Self::hsl2rgb(rndf32(), 0.5 * rndf32() + 0.3, 0.5 * rndf32() + 0.45)
    }
}
