use cgmath::{Point3, Vector3};
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Point3<f64>,
    pub normal: Vector3<f64>
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(p: Point3<f64>, normal: Vector3<f64>) -> HitRecord {
        HitRecord {
            p,
            normal
        }
    }
}
