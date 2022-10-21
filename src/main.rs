mod camera;
mod color;
mod ray;
mod sphere;
mod surface;
mod surface_list;
mod vec3;

use crate::camera::Camera;
use crate::color::write_color;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::surface_list::SurfaceList;
use crate::vec3::*;
use rand::Rng;
use std::rc::Rc;
use surface::HitRecord;

fn main() {
    //Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;

    let mut rng = rand::thread_rng();

    //World
    let mut world = SurfaceList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    //Camera
    let cam = Camera::new(16.0 / 9.0, 2.0, 1.0);

    //Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL as f32);
        }
    }
}

fn ray_color(r: &Ray, world: &SurfaceList<Sphere>) -> Color {
    //Do I change below to use Option<HitRecord>?
    let mut rec: HitRecord = HitRecord::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
        0.0,
        false,
    );
    if world.hit(r, 0.0, f32::INFINITY, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction: Vec3 = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
