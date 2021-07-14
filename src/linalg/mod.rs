pub mod vec2;
pub mod vec2i;
pub mod vec3;
pub mod mat4;

pub use vec2::Vec2;
pub use vec2i::Vec2i;
pub use vec3::Vec3;
pub use mat4::Mat4;

pub fn norm_u64(v: u64, min: u64, max: u64) -> f32 {
    if v <= min { return 0.0; }
    if v >= max { return 1.0; }
    (v - min) as f32 / (max - min) as f32
}

/*
fn norm_f32(v: f32, min: f32, max: f32) -> f32 {
    if v <= min { return 0.0; }
    if v >= max { return 1.0; }
    (v - min) / (max - min)
}
*/

pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    Vec2 {
        x: lerp_f32(a.x, b.x, t),
        y: lerp_f32(a.y, b.y, t)
    }
}
