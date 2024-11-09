use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let img = image::open(&args[1]).unwrap().to_rgba8();

    let width = img.width();
    let height = img.height();

    let mut output = Vec::new();

    output.extend_from_slice(b"qoif");
    output.extend(width.to_be_bytes());
    output.extend(height.to_be_bytes());
    output.push(4);
    output.push(0);

    let mut seen = [(0, 0, 0, 255); 64];
    let mut previous = seen[0];

    for pixel in img.chunks(4) {
        if let &[r, g, b, a] = pixel {
            let current = (r, g, b, a);
            let hash = (r * 3 + g * 5 + b * 7 + a * 11) % 64;

            seen[hash as usize] = current;

            previous = current;
        }
    }

    output.extend([0, 0, 0, 0, 0, 0, 0, 1]);

    let mut file = File::create("output.qoi").unwrap();
    file.write_all(&output).unwrap();
}
