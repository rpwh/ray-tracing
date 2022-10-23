use rand::Rng;

use crate::{
    ray::Ray,
    surface::HitRecord,
    vec3::{Color, Vec3, reflect, unit_vector, dot},
};

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { ir: f32 }
}

impl Default for Material {
    fn default() -> Material {
        Material::Lambertian {
            albedo: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

impl Material {
    pub fn scatter(self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                Self::scatter_lambertian(&albedo, r_in, rec, attenuation, scattered)
            },
            Material::Metal { albedo, fuzz } => {
                Self::scatter_metal(&albedo, fuzz, r_in, rec, attenuation, scattered)
            },
            Material::Dielectric { ir } => {
                Self::scatter_dielectric(ir, r_in, rec, attenuation, scattered)
            }
        }
    }

    fn scatter_lambertian(albedo: &Vec3, _ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = *albedo;
        true
    }

    fn scatter_metal(albedo: &Vec3, fuzz: f32, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(&unit_vector(r_in.direction), &rec.normal);
        *scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere()*fuzz);
        *attenuation = *albedo;
        dot(&scattered.direction, &rec.normal) > 0.0
    }

    fn scatter_dielectric(ir: f32, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0/ir
        } else {
            ir
        };
        let unit_direction = unit_vector(r_in.direction);
        let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut rng = rand::thread_rng();
        let direction = if cannot_refract || Material::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0) {
            reflect(&unit_direction, &rec.normal)
        } else {
            unit_direction.refract(&rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction);
        true
    }

    fn reflectance(cosine: f32, ir: f32) -> f32 {
        let mut r0 = (1.0 - ir) / (1.0 + ir);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
