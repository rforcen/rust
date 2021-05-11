// fast hsv rust interface
/*
    void fast_hsv2rgb_8bit(uint16_t h, uint8_t s, uint8_t v, uint8_t *r, uint8_t *g , uint8_t *b) HSVFUNC_ATTRUSED;
    void fast_hsv2rgb_32bit(uint16_t h, uint8_t s, uint8_t v, uint8_t *r, uint8_t *g , uint8_t *b) HSVFUNC_ATTRUSED;
*/
#![allow(dead_code)]

#[link(name = "hsv")]
extern "C" {
    pub fn fast_hsv2rgb_8bit(h: u16, s: u8, v: u8, r: *mut u8, g: *mut u8, b: *mut u8);
    pub fn fast_hsv2rgb_32bit(h: u16, s: u8, v: u8, r: *mut u8, g: *mut u8, b: *mut u8);
}

// wrappers

pub fn hvs2_rgbu8(h: u16, s: u8, v: u8) -> u32 {
    let (mut r, mut g, mut b) = (0_u8, 0_u8, 0_u8);
    unsafe { fast_hsv2rgb_8bit(h, s, v, &mut r, &mut g, &mut b) };
    (r as u32) << 16 | (g as u32) << 8 | b as u32
}
pub fn hvs2_rgbu32(h: u16, s: u8, v: u8) -> u32 {
    let (mut r, mut g, mut b) = (0_u8, 0_u8, 0_u8);
    unsafe { fast_hsv2rgb_32bit(h, s, v, &mut r, &mut g, &mut b) };
    (r as u32) << 16 | (g as u32) << 8 | b as u32
}
pub fn hvs2_rgb(h: f32, s: f32, v: f32) -> u32 {
    let (mut r, mut g, mut b) = (0_u8, 0_u8, 0_u8);
    unsafe {
        fast_hsv2rgb_32bit(
            (h * u16::MAX as f32) as u16,
            (s * u8::MAX as f32) as u8,
            (v * u8::MAX as f32) as u8,
            &mut r,
            &mut g,
            &mut b,
        )
    };
    (r as u32) << 16 | (g as u32) << 8 | b as u32
}

mod test {
    use super::*;
    #[test]
    fn test_all() {
        let (mut r, mut g, mut b) = (0_u8, 0_u8, 0_u8);
        unsafe { fast_hsv2rgb_8bit(1200, 100, 100, &mut r, &mut g, &mut b) };
        println!("{} {} {}", r, g, b);
        unsafe { fast_hsv2rgb_32bit(1200, 100, 100, &mut r, &mut g, &mut b) };
        println!("{} {} {}", r, g, b)
    }
    #[test]
    fn test_wrapper() {
        println!("{:x}", hvs2_rgbu8(1200, 100, 100));
        println!("{:x}", hvs2_rgbu32(1200, 100, 100));
        println!("{:x}", hvs2_rgb(0.4, 0.3, 0.2));
    }
}
