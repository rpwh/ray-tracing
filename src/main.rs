mod camera;
mod color;
mod ray;
mod sphere;
mod surface;
mod surface_list;
mod vec3;
mod material;

use crate::camera::Camera;
use crate::color::write_color;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::surface_list::SurfaceList;
use crate::vec3::*;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::rc::Rc;
use surface::HitRecord;
use pbr::ProgressBar;

fn main() {
    //Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 200;

    let mut rng = rand::thread_rng();

    //World
    let mut world = SurfaceList::new();

    let material_ground = Material::Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) };
    let material_center = Material::Dielectric { ir: 1.5 };
    let material_left = Material::Dielectric { ir: 1.5 };
    let material_right = Material::Metal { albedo: Vec3::new(0.8, 0.6, 0.2), fuzz: 1.0 };

    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    //Camera
    let cam = Camera::new(16.0 / 9.0, 2.0, 1.0);

    //Render
    let mut image_file = BufWriter::new(File::create("image.ppm").expect("Error Creating File"));
    writeln!(image_file, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();

    let mut pb = ProgressBar::new(IMAGE_HEIGHT as u64);
    pb.format("╢▌▌░╟");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(&mut image_file, pixel_color, SAMPLES_PER_PIXEL as f32);
        }
        pb.inc();
    }
}

fn ray_color(r: &Ray, world: &SurfaceList<Sphere>, depth: i32) -> Color {
    //Do I change below to use Option<HitRecord>?
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, f32::INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth-1 )
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction: Vec3 = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
