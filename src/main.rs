
use color::Color;
use cgmath::{Point3, Vector3};
use cgmath::prelude::*;
use image::{ImageBuffer, RgbImage, Rgb};
use ray::Ray;
use crate::sphere::Sphere;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use std::f64;
use crate::camera::Camera;
use rand::random;
use std::time::Instant;
use crate::material::{Lambertian, Metal, Dielectric};
use std::f64::consts::PI;

mod ray;
mod color;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;
mod material;

fn color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    // let hit = sphere.hit(ray, 0.0, 5000.0);
    if depth >= 50 {
        return Color::black();
    }

    if let Some(h) = world.hit(ray, 0.001, f64::MAX) {
        let s = h.material.scatter(ray, &h);
        return match s {
            Some((r, a)) => a * color(&r, world, depth+1),
            None => Color::black()
        };
        // let target = h.p + h.normal + random_unit_vector();
        /* let r = h.normal.x + 1.0;
        let g = h.normal.y + 1.0;
        let b = h.normal.z + 1.0; */
        // return 0.5*Color::new(r, g, b);
        // return 0.5 * color(&Ray::new(h.p, target - h.p), world, depth +1);
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5*(unit_direction.y + 1.0);
    return (1.0 - t) * Color::white() + t * Color::new(0.5, 0.7, 1.0);
}

fn color_to_rgb(color: &Color) -> Rgb<u8> {
    let gamma_cor = color.gamma_correct();
    let ir = (255.99 * gamma_cor.r) as u8;
    let ig = (255.99 * gamma_cor.g) as u8;
    let ib = (255.99 * gamma_cor.b) as u8;
    return Rgb([ir, ig, ib]);
}

fn main() {
    let now = Instant::now();

    let height: u32 = 200;
    let width: u32 = 300;
    let num_of_samples = 50;

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let world = create_world();

    /*let look_from = Point3::new(3.0, 3.0, 2.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (look_from - look_at).magnitude();*/

    let look_from = Point3::new(3.0, 3.0, 2.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (look_from - look_at).magnitude();
    let cam = Camera::new(look_from, look_at, Vector3::new(0.0, 1.0, 0.0), 20.0, width as f64/height as f64, 0.5, dist_to_focus);

    for j in 0..height {
        print!("starting row {} ...", j);
        for i in 0..width {
            let mut col = Color::black();
            for _s in 0..num_of_samples {
                let u = (i as f64 + random::<f64>()) / (width as f64);
                let v = (j as f64 + random::<f64>()) / (height as f64);
                let r = cam.get_ray(u, v);
                col += color(&r, &*world, 0);
            }
            col /= num_of_samples as f64;
            img.put_pixel(i, height - j - 1, color_to_rgb(&col));
        }
        println!("done.");
    }

    img.save("images/output.png").expect("Talletus epäonnistui");

    let elapsed_time = now.elapsed().as_secs();

    println!("Took {}s to render the image.", elapsed_time);
}

fn create_world() -> Box<dyn Hittable> {
    let mut vec: Vec<Box<dyn Hittable>> = vec![];
    vec.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))))));
    vec.push(Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0, Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))))));

    vec.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.3)))));
    // vec.push(Box::new(Sphere::new(Point3::new(-1.0,0.0, -1.0), 0.5, Box::new(Metal::new(Color::new(0.8, 0.8, 0.8), 1.0)))));
    vec.push(Box::new(Sphere::new(Point3::new(-1.0,0.0, -1.0), 0.5, Box::new(Dielectric::new(1.5)))));
    vec.push(Box::new(Sphere::new(Point3::new(-1.0,0.0, -1.0), -0.45, Box::new(Dielectric::new(1.5)))));

    return Box::new(HittableList::new(vec));
}

fn create_random_scene() -> Box<dyn Hittable> {
    let mut vec: Vec<Box<dyn Hittable>> = vec![];

    vec.push(Box::new(Sphere::new(Point3::new(0.0,-100.0, 0.0), 1000.0, Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))))));

    for a in -11..11 {
        for b in -11..11 {

        }
    }
    vec.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dielectric::new(1.5)))));
    vec.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))))));
    vec.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)))));

    return Box::new(HittableList::new(vec));
}
