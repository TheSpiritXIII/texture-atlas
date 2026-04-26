use image::RgbImage;
use rand::RngExt;

pub fn generate(min: (u32, u32), max: (u32, u32)) -> RgbImage {
	let mut rng = rand::rng();
	let width = rng.random_range(min.0..=max.0);
	let height = rng.random_range(min.1..=max.1);

	let mut image = RgbImage::new(width, height);
	image.fill(255);
	image
}
