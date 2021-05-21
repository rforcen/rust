// color_interp.rs
#![allow(dead_code)]

pub fn default_interpolate(ratio: f32) -> [f32; 3] {
  color2rgbf3(interp_hsl(0xff_00_00, 0x00_00_ff, ratio))
}

pub fn interpolate(c1: u32, c2: u32, ratio: f32) -> (f32, f32, f32) {
  color2rgbf(interp_hsl(c1, c2, ratio))
}

pub fn interp_rgb(c1: u32, c2: u32, ratio: f32) -> u32 {
  let (r1, g1, b1) = color2rgb(c1);
  let (r2, g2, b2) = color2rgb(c2);

  // ratio parameter must be between 0 and 1
  match ratio {
    _r if ratio <= 0. => c1,
    _r if ratio >= 1. => c2,
    _ => make_rgb(
      (r1 + (r2 - r1)) as f32 * ratio + 0.5,
      (g1 + (g2 - g1)) as f32 * ratio + 0.5,
      (b1 + (b2 - b1)) as f32 * ratio + 0.5,
    ),
  }
}

pub fn interp_hsl(c1: u32, c2: u32, ratio: f32) -> u32 {
  // ratio parameter must be between 0 and 1
  match ratio {
    _r if ratio <= 0. => c1,
    _r if ratio >= 1. => c2,
    _ => {
      let (h1, l1, s1) = rgb2hls(c1);
      let (h2, l2, s2) = rgb2hls(c2);
      hls2rgb(
        h1 + (h2 - h1) * ratio,
        l1 + (l2 - l1) * ratio,
        s1 + (s2 - s1) * ratio,
      )
    }
  }
}

fn make_rgb(r: f32, g: f32, b: f32) -> u32 {
  (((r * 255.) as u32) << 16) | (((g * 255.) as u32) << 8) | ((b * 255.) as u32)
}
fn color2rgb(color: u32) -> (u32, u32, u32) {
  ((color >> 16) & 0xff, (color >> 8) & 0xff, color & 0xff)
}

fn hls2rgb(h: f32, l: f32, s: f32) -> u32 {
  if s == 0. {
    make_rgb(l, l, l)
  } else {
    let m2 = if l <= 0.5 {
      l * (1. + s)
    } else {
      l + s - l * s
    };
    let m1 = 2.0 * l - m2;
    let r = hue2rgb(m1, m2, h + 1.0 / 3.0);
    let g = hue2rgb(m1, m2, h);
    let b = hue2rgb(m1, m2, h - 1.0 / 3.0);
    make_rgb(r, g, b)
  }
}

fn hue2rgb(m1: f32, m2: f32, h: f32) -> f32 {
  let h = h
    + match h {
      _ if h < 0. => 1.,
      _ if h > 1. => -1.,
      _ => 0.,
    };
  match h {
    _ if 6. * h < 1. => m1 + (m2 - m1) * h * 6.,
    _ if 2. * h < 1. => m2,
    _ if 3. * h < 2. => m1 + (m2 - m1) * ((2. / 3.) - h) * 6.,
    _ => m1,
  }
}

fn color2rgbf(color: u32) -> (f32, f32, f32) {
  (
    ((color >> 16) & 0xff) as f32 / 255.,
    ((color >> 8) & 0xff) as f32 / 255.,
    (color & 0xff) as f32 / 255.,
  )
}
fn color2rgbf3(color: u32) -> [f32; 3] {
  [
    ((color >> 16) & 0xff) as f32 / 255.,
    ((color >> 8) & 0xff) as f32 / 255.,
    (color & 0xff) as f32 / 255.,
  ]
}

fn rgb2hls(rgb: u32) -> (f32, f32, f32) {
  let (r, g, b) = color2rgbf(rgb);

  let (cmax, cmin) = (r.max(g.max(b)), r.min(g.min(b)));

  let (l, mut s, mut h) = ((cmax + cmin) / 2., 0., 0.);

  if cmax != cmin {
    s = if l < 0.5 {
      (cmax - cmin) / (cmax + cmin)
    } else {
      (cmax - cmin) / (2. - cmax - cmin)
    };
    let delta = cmax - cmin;
    h = match r {
      _ if r == cmax => (g - b) / delta,
      _ if g == cmax => 2. + (b - r) / delta,
      _ => 4. + (r - g) / delta,
    } / 6.;
    if h < 0. {
      h += 1.;
    }
  }
  (h, l, s)
}
