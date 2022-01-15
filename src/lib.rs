pub mod vec3;

use crate::vec3::Vec3;

// type Point3D = Vec3;
pub type ColorRGB = Vec3;

pub fn write_color(pixel: &ColorRGB) {
    let r = (pixel.x() * 255.999) as u32;
    let g = (pixel.y() * 255.999) as u32;
    let b = (pixel.z() * 255.999) as u32;

    println!("{} {} {}", r, g, b);
}
