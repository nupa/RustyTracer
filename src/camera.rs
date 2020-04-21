use cgmath::{Point3, Vector3};
use crate::ray::Ray;

pub struct Camera {
    origin: Point3<f64>,
    lower_left_corner:  Point3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: Point3::new(0.0, 0.0, 0.0),
            lower_left_corner: Point3::new(-2.0, -1.0, -1.0),
            horizontal: Vector3::new(4.0, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0, 0.0)
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = (self.lower_left_corner + u*self.horizontal + v*self.vertical) - self.origin;
        return Ray::new(self.origin, direction);
    }
}