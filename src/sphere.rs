use cgmath::{Point3, InnerSpace};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Sphere {
        Sphere {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;
        return if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            let hit_point = ray.point_at(t);
            let n = (hit_point - self.center) / self.radius;
            Some(HitRecord::new(hit_point, n))
        } else {
            None
        }
    }
}