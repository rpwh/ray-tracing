use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;

pub struct HitRecord{
    pub p: Point3,
    pub normal: Vec3,
    pub t:f32,
}

impl HitRecord {
    pub fn new(p: Point3 , normal: Vec3, t: f32) -> HitRecord {
        HitRecord{p, normal, t}
    }
}

pub trait Surface {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
