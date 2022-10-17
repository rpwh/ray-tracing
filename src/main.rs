mod vec3;
use crate::vec3::Vec3;

fn main(){

    //Image
    const IMAGE_HEIGHT: u32 = 256;
    const IMAGE_WIDTH: u32 = 256;
    
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    //Render
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / (IMAGE_HEIGHT-1) as f64;
            let g = (j as f64) / (IMAGE_WIDTH-1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
