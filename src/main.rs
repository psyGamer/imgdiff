use std::env;
use num::abs;

use image::{ImageBuffer, RgbImage};

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 4 {
		println!("Usage: imgdiff [image1] [image2] [outputImage]");
		std::process::exit(exitcode::USAGE);
	}

    let first_img = read_img(&args[1]);
	let second_img = read_img(&args[2]);

	if first_img.width() != second_img.width() || first_img.height() != second_img.width() {
		println!("Images must have same width and height!");
		std::process::exit(exitcode::DATAERR);
	}

	let diff_img: RgbImage = ImageBuffer::from_fn(first_img.width(), first_img.height(), |x, y| {
		let r: u8 = abs(first_img[(x, y)].0[0] as i16 - second_img[(x, y)].0[0] as i16) as u8;
		let g: u8 = abs(first_img[(x, y)].0[1] as i16 - second_img[(x, y)].0[1] as i16) as u8;
		let b: u8 = abs(first_img[(x, y)].0[2] as i16 - second_img[(x, y)].0[2] as i16) as u8;

		image::Rgb([r, g, b])
	});
	diff_img.save(&args[3])
		.expect("Failed writing diff.png");
	println!("Successfully saved image diff as {}", args[3]);
}

fn read_img(path: &str) -> RgbImage {
	image::io::Reader::open(path)
		.expect(&format!("Failed opening {}", path))
		.decode()
		.expect(&format!("Failed reading {}", path))
		.to_rgb8()
}
