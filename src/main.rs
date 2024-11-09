use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let img = image::open(&args[1]).unwrap().to_rgba8();

    let mut file = File::create("output.qoi").unwrap();

    file.write(b"qoif").unwrap();
    file.write(&img.width().to_be_bytes()).unwrap();
    file.write(&img.height().to_be_bytes()).unwrap();
    file.write(&(4 as u8).to_be_bytes()).unwrap();
    file.write(&(0 as u8).to_be_bytes()).unwrap();

    let mut seen = [(0, 0, 0, 255); 64];
    let mut previous = seen[0];

    for pixel in img.chunks(4) {
        let current = (pixel[0], pixel[1], pixel[2], pixel[3]);
        let hash = (pixel[0] * 3 + pixel[1] * 5 + pixel[2] * 7 + pixel[3] * 11) % 64;

        seen[hash as usize] = current;

        previous = current;
    }
}
