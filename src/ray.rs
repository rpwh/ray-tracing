use crate::vec3::*;

pub struct Ray{
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction*t
    }
}
