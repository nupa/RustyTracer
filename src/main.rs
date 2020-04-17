
use color::Color;
use cgmath::{Point3, Vector3};
use cgmath::prelude::*;
use image::{ImageBuffer, RgbImage, Rgb};
use ray::Ray;

mod ray;
mod color;

fn color(ray: &Ray) -> Color {
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
            let point_to = lower_left_corner + u*horizontal + v*vertical;
            let r = Ray::new(origin, Vector3::new(point_to.x, point_to.y, point_to.z));
            img.put_pixel(i, height - j - 1, color_to_rgb(&color(&r)));
        }
    }

    img.save("images/output.png").expect("Talletus epäonnistui");


}
