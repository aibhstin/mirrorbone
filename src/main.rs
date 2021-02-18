extern crate image;
use image::{RgbImage, Rgb};

use rand::Rng;

use std::process::Command;
use std::thread;
use std::time;
use std::fs;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

fn main() {
    let mut rng = rand::thread_rng();

    loop {
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

        let mut child = Command::new("termimage")
            .arg("output.png")
            .spawn()
            .expect("Failed to execute child");

        thread::sleep(time::Duration::from_secs(5));
        
        //child
        //    .kill()
        //    .expect("Command not running");

        //let ecode = child
        //    .wait()
        //    .expect("Failed to wait on child");

        fs::remove_file("output.png");
    }
}





