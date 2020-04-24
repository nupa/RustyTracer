use cgmath::{Point3, Vector3, InnerSpace};
use crate::ray::Ray;
use std::f64::consts::PI;
use rand::random;
use cgmath::num_traits::real::Real;

pub struct Camera {
    origin: Point3<f64>,
    lower_left_corner:  Point3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
    lens_radius: f64
}

impl Camera {
    pub fn new(look_from: Point3<f64>, look_at: Point3<f64>, vup:Vector3<f64>, vfov: f64,
               aspect: f64, aperture: f64, focal_distance: f64) -> Camera {
        let theta = vfov * PI/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = half_height * aspect;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            origin: look_from, //Point3::new(0.0, 0.0, 0.0),
            lower_left_corner: look_from - half_width * focal_distance * u - half_height * focal_distance * v - focal_distance * w,
            horizontal: 2.0 * half_width * focal_distance * u,
            vertical: 2.0 * half_height * focal_distance * v,
            u,
            v,
            w,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let direction = (self.lower_left_corner + s *self.horizontal + t *self.vertical) - self.origin - offset;
        return Ray::new(self.origin + offset, direction);
    }
}

fn random_in_unit_disk() -> Point3<f64> {
    let a = random::<f64>() * 2.0 * PI;
    let r = random::<f64>().sqrt();

    return Point3::new(r * a.cos(), r * a.sin(), 0.0);
}