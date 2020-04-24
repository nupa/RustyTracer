use cgmath::{Point3, InnerSpace};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub material: Box<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material
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
        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            fn hit_record<'a>(t: f64, ray: &Ray, center: &Point3<f64>, radius: f64, material: &'a dyn Material) -> HitRecord<'a> {
                let hit_point = ray.point_at(t);
                let outward_normal = (hit_point - center) / radius;
                let ff = ray.direction.dot(outward_normal) < 0.0;
                let normal = if ff {outward_normal} else {-1.0*outward_normal};
                return HitRecord::new(t, hit_point, normal, material, ff);
            }

            // check if either of hits match timelimits
            let t = (-b - root) / (2.0 * a);
            if t > t_min && t < t_max {
                return Some(hit_record(t, ray, &self.center, self.radius, &*self.material));
            }
            let t = (-b + root) / (2.0 * a);
            if t > t_min && t < t_max {
                return Some(hit_record(t, ray, &self.center, self.radius, &*self.material))
            }
        }
        return None

    }
}
