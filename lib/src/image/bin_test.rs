use std::num::NonZero;

use image::DynamicImage;
use image::Rgba;
use image::RgbaImage;

use crate::BinAdd;
use crate::Options2;
use crate::Pos2;
use crate::Rotate2;

#[test]
fn image_buffer_bin_add() {
	let options = Options2::with_max_size(NonZero::new(64).unwrap(), NonZero::new(64).unwrap());
	let mut bin = RgbaImage::new(options.max_width(), options.max_height());
	let item = RgbaImage::from_pixel(
		10,
		10,
		Rgba([
			255,
			0,
			0,
			255,
		]),
	);

	bin.item_add(
		&item,
		&Pos2 {
			x: 5,
			y: 5,
		},
	)
	.unwrap();
	assert_eq!(
		*bin.get_pixel(5, 5),
		Rgba([
			255,
			0,
			0,
			255
		])
	);

	bin.item_add(
		&item,
		&Rotate2 {
			pos: Pos2 {
				x: 20,
				y: 20,
			},
			rotate: true,
		},
	)
	.unwrap();
	assert_eq!(
		*bin.get_pixel(20, 20),
		Rgba([
			255,
			0,
			0,
			255
		])
	);
}

#[test]
fn dynamic_image_bin_add() {
	let options = Options2::with_max_size(NonZero::new(64).unwrap(), NonZero::new(64).unwrap());
	let mut bin = RgbaImage::new(options.max_width(), options.max_height());
	let item = DynamicImage::from(RgbaImage::from_pixel(
		10,
		10,
		Rgba([
			255,
			0,
			0,
			255,
		]),
	));

	bin.item_add(
		&item,
		&Pos2 {
			x: 5,
			y: 5,
		},
	)
	.unwrap();
	assert_eq!(
		*bin.get_pixel(5, 5),
		Rgba([
			255,
			0,
			0,
			255
		])
	);

	bin.item_add(
		&item,
		&Rotate2 {
			pos: Pos2 {
				x: 20,
				y: 20,
			},
			rotate: true,
		},
	)
	.unwrap();
	assert_eq!(
		*bin.get_pixel(20, 20),
		Rgba([
			255,
			0,
			0,
			255
		])
	);
}
