use image::RgbImage;

pub fn generate() -> RgbImage {
	let mut image = RgbImage::new(128, 128);
	image.fill(255);
	image
}
