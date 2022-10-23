use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Material,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction, outward_normal) < 0.0;
        match self.front_face {
            true => self.normal = *outward_normal,
            false => self.normal = -*outward_normal,
        };
    }
}

pub trait Surface {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
