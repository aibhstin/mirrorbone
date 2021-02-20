extern crate image;
use image::{RgbImage, Rgb};

use rand::Rng;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;

fn main() {
    let mut rng = rand::thread_rng();

    let mut img = RgbImage::new(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let n1: u8 = rng.gen();
            let n2: u8 = rng.gen();
            let n3: u8 = rng.gen();
            img.put_pixel(x, y, Rgb([n1, n2, n3]));
        }
    }

    img.save("output.png").unwrap();
}





