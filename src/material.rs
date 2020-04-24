use cgmath::{Vector3, dot};
use::cgmath::prelude::*;
use rand::random;
use std::f64::consts::PI;
use cgmath::num_traits::real::Real;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let target = hit.normal + random_unit_vector();
        return Some((Ray::new(hit.p, target), self.albedo));
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0)
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&r_in.direction.normalize(), &hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());

        if dot(scattered.direction, hit.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ref_idx: f64
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric {
            ref_idx
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::white();
        let etai_over_etat = match hit.frontface {
            true => 1.0/self.ref_idx,
            false => self.ref_idx
        };
        let unit_direction = r_in.direction.normalize();
        let cos_theta = hit.normal.dot(-1.0 * unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        return if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(&unit_direction, &hit.normal);
            Some((Ray::new(hit.p, reflected), attenuation))
        } else {
            let reflect_prob = schlick(cos_theta, etai_over_etat);
            if random::<f64>() < reflect_prob {
                let reflected = reflect(&unit_direction, &hit.normal);
                Some((Ray::new(hit.p, reflected), attenuation))
            } else {
                let refracted = refract(&unit_direction, &hit.normal, etai_over_etat);
                Some((Ray::new(hit.p, refracted), attenuation))
            }
        }
    }
}

fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    return v - 2.0 * v.dot(*n) * n;
}

fn random_in_unit_sphere() -> Vector3<f64> {
    loop {
        let v = 2.0 * Vector3::new(random::<f64>(), random::<f64>(), random::<f64>()) - Vector3::new(1.0, 1.0, 1.0);
        if v.magnitude2() < 1.0 {
            return v;
        }
    }
}

fn random_unit_vector() -> Vector3<f64> {
    let a = random::<f64>()*2.0*PI;
    let z = 1.0 - 2.0 * random::<f64>();
    let r = (1.0 - z*z).sqrt();
    return Vector3::new(r*a.cos(), r*a.sin(), z);
}

fn refract(uv: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = n.dot(-1.0 * uv);
    let r_out_paraller = etai_over_etat * (uv + cos_theta * n);
    let r_out_perp = -1.0 * (1.0 - r_out_paraller.magnitude2()).sqrt() * n;
    return r_out_paraller + r_out_perp;
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx)/(1.0 + ref_idx)).powi(2);
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}