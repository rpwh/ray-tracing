mod vec3;
mod color;
mod ray;
use crate::vec3::*;
use crate::color::write_color;

fn main(){

    //Image
    const IMAGE_HEIGHT: u32 = 256;
    const IMAGE_WIDTH: u32 = 256;
    
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    //Render
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let pixel_color: Color = Color::new(i as f32/(IMAGE_WIDTH-1) as f32, j as f32/(IMAGE_HEIGHT-1) as f32, 0.25);
            write_color(pixel_color);
        }
    }
}
