use std::{thread, time};
use crate::consts;

#[allow(dead_code)]
pub fn test() {
    let mut image = ProxyImage::new("rust.ascii");
    image.display();
    image.display();
}

// Image

trait Image {
    fn new(file_name: &'static str) -> Self;
    fn display(&mut self) -> ();
}

// RealImage

#[derive(Debug)]
struct RealImage {
    file_name: &'static str,
}

impl RealImage {
    fn load_from_disk(&self) -> () {
        println!("Load image {}", self.file_name);
        thread::sleep(time::Duration::from_millis(consts::WAIT));
        println!("Finish loading");
    }
}

impl Image for RealImage {
    fn new(file_name: &'static str) -> Self {
        let r = RealImage { file_name };
        r.load_from_disk();
        r
    }

    fn display(&mut self) -> () {
        println!("Display image {}", self.file_name);
        println!("{}", consts::RUST_IMG);
    }
}

// ProxyImage

#[derive(Debug)]
struct ProxyImage {
    file_name: &'static str,
    real_image: Option<RealImage>,
}

impl Image for ProxyImage {
    fn new(file_name: &'static str) -> Self {
        ProxyImage { file_name, real_image: None }
    }

    fn display(&mut self) -> () {
        let n = self.file_name;
        let i = self.real_image.get_or_insert_with(|| RealImage::new(n));
        i.display();
    }
}
