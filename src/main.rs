
use color::Color;
use cgmath::{Point3, Vector3};
use cgmath::prelude::*;
use image::{ImageBuffer, RgbImage, Rgb};
use ray::Ray;

mod ray;
mod color;
mod hittable;

fn hit_sphere(center: &Point3<f64>, radius: f64, ray: &Ray) -> Option<Vector3<f64>> {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant > 0.0 {
        let t = (-b - discriminant.sqrt() ) / (2.0 * a);
        let hit_point = ray.point_at(t);
        let n = (hit_point - center) / radius;
        return Some(n);
    } else {
        return None;
    }
}

fn color(ray: &Ray) -> Color {
    let hit_normal = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if hit_normal != None {
        let r = hit_normal.unwrap().x + 1.0;
        let g = hit_normal.unwrap().y + 1.0;
        let b = hit_normal.unwrap().z + 1.0;
        return 0.5*Color::new(r, g, b);
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5*(unit_direction.y + 1.0);
    return (1.0 - t) * Color::white() + t * Color::new(0.5, 0.7, 1.0);
}

fn color_to_rgb(color: &Color) -> Rgb<u8> {
    let ir = (255.99 * color.r) as u8;
    let ig = (255.99 * color.g) as u8;
    let ib = (255.99 * color.b) as u8;
    return Rgb([ir, ig, ib]);
}

fn main() {
    println!("Hello, world!");

    let height: u32 = 100;
    let width: u32 = 200;

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let origin = Point3::new(0.0, 0.0, 0.0);
    let lower_left_corner = Point3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);

    for j in 0..height {
        for i in 0..width {
            let u = (i as f64) / (width as f64);
            let v = (j as f64) / (height as f64);
            let direction = (lower_left_corner + u*horizontal + v*vertical) - origin;
            let r = Ray::new(origin, direction);
            img.put_pixel(i, height - j - 1, color_to_rgb(&color(&r)));
        }
    }

    img.save("images/output.png").expect("Talletus epäonnistui");


}
