use cgmath::{Point3, Vector3};
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: &'a dyn Material,
    pub frontface: bool
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, p: Point3<f64>, normal: Vector3<f64>, material: &'a dyn Material, frontface: bool) -> Self {
        Self {
            t,
            p,
            normal,
            material,
            frontface
        }
    }
}
