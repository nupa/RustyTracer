use cgmath::{Vector3};
use::cgmath::prelude::*;
use rand::random;
use std::f64::consts::PI;
use cgmath::num_traits::real::Real;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;

pub trait Material {
    fn scatter(r_in: &Ray, hit: &HitRecord) -> Option<(Vector3<f64>, Color)>;
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    loop {
        let v = 2.0 * Vector3::new(random::<f64>(), random::<f64>(), random::<f64>()) - Vector3::new(1.0, 1.0, 1.0);
        if v.magnitude2() < 1.0 {
            return v;
        }
    }
}

pub fn random_unit_vector() -> Vector3<f64> {
    let a = random::<f64>()*2.0*PI;
    let z = 1.0 - 2.0 * random::<f64>();
    let r = (1.0 - z*z).sqrt();
    return Vector3::new(r*a.cos(), r*a.sin(), z);
}
