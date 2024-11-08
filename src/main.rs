use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let img = image::open(&args[1]).unwrap().to_rgba8();

    let width = img.width();
    let height = img.height();

    for pixel in img.chunks(4) {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        println!("{:x}{:x}{:x}", r, g, b);
    }
}
