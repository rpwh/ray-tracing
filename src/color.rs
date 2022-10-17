use crate::vec3::Color;

pub fn write_color(pixel_color: Color) {
    let i = 255.999;
    println!("{} {} {}", (i*pixel_color.x) as u32, (i*pixel_color.y) as u32, (i*pixel_color.z) as u32)
}
