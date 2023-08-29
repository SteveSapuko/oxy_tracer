use image::Rgb;
use crate::vec3::*;

pub fn in_range<T: PartialOrd>(x: T, min: T, max: T) -> bool {
    min < x && x < max
}

pub fn v3_to_rgb(v: V3) -> Rgb<u8> {
    let r:u8 = v.x.clamp(0.0, 255.0).round() as u8;
    let g:u8 = v.y.clamp(0.0, 255.0).round() as u8;
    let b:u8 = v.z.clamp(0.0, 255.0).round() as u8;

    Rgb([r, g, b])
}