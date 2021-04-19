// aux_funcs -> f32 wrappers

pub fn sinf(x: f32) -> f32 {
    x.sin()
}
pub fn cosf(x: f32) -> f32 {
    x.cos()
}
pub fn sinh(x: f32) -> f32 {
    x.sinh()
}
pub fn cosh(x: f32) -> f32 {
    x.cosh()
}
pub fn sqr(x: f32) -> f32 {
    x * x
}
pub fn cube(x: f32) -> f32 {
    x * x * x
}
pub fn sqr5(x: f32) -> f32 {
    x * x * x * x * x
}
pub fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}
pub fn fabs(x: f32) -> f32 {
    x.abs()
}
pub fn exp(x: f32) -> f32 {
    x.exp()
}
pub fn ln(x: f32) -> f32 {
    x.ln()
}
pub fn max(x: f32, y: f32) -> f32 {
    x.max(y)
}
pub fn powf(x: f32, y: f32) -> f32 {
    x.powf(y)
}
