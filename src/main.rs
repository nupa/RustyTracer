
use color::Color;
use cgmath::{Point3};
use image::{ImageBuffer, RgbImage, Rgb};

mod ray;
mod color;

fn main() {
    println!("Hello, world!");

    let height: u32 = 200;
    let width: u32 = 300;

    let mut img: RgbImage = ImageBuffer::new(width, height);

    for j in 0..height {
        println!("{}",j);
        for i in 0..width {
            img.put_pixel(i, j, Rgb([j as u8, j as u8, (i % 255) as u8]))
        }
    }

    img.save("images/output.png");


}
