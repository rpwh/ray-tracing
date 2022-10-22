use crate::vec3::Color;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufWriter;

pub fn write_color(file: &mut BufWriter<File>, pixel_color: Color, samples: f32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();


    writeln!(
        file,
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as u32,
        (256.0 * g.clamp(0.0, 0.999)) as u32,
        (256.0 * b.clamp(0.0, 0.999)) as u32,
    )
    .unwrap();
}
