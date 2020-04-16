
use cgmath::{Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    fn point_at(&self, time: f64) -> Point3<f64> {
        self.origin + self.direction * time
    }
}
