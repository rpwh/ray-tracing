mod color;
mod ray;
mod sphere;
mod surface;
mod surface_list;
mod vec3;
use std::rc::Rc;

use surface::HitRecord;

use crate::color::write_color;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::surface_list::SurfaceList;
use crate::vec3::*;

fn main() {
    //Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    //World
    let mut world = SurfaceList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    //Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH as f32 - 1.0);
            let v = j as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color: Color = ray_color(&r, &world);
            write_color(pixel_color);
        }
    }
}

fn ray_color(r: &Ray, world: &SurfaceList<Sphere>) -> Color {
    //Do I change below to use Option<HitRecord>?
    let mut rec: HitRecord = HitRecord::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0), 0.0, false);
    if world.hit(r, 0.0, f32::INFINITY, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction: Vec3 = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
