use crate::ray::Ray;
use crate::surface::{HitRecord, Surface};
use std::rc::Rc;

pub struct SurfaceList<T: Surface> {
    pub surface_list: Vec<Rc<T>>,
}

impl<T> SurfaceList<T>
where
    T: Surface,
{
    pub fn new() -> Self {
        SurfaceList {
            surface_list: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<T>) {
        self.surface_list.push(object);
    }

    pub fn clear(&mut self) {
        self.surface_list.clear();
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = rec.clone();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.surface_list {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
