use image::ColorType;
use image::DynamicImage;
use image::GenericImage;
use image::Rgba;
use image::RgbaImage;

use crate::util::Bounds;
use crate::util::border_crop;

fn crop(image: &DynamicImage) -> Option<(DynamicImage, Bounds)> {
	border_crop(image).map(|(image, bounds)| (image.to_image().into(), bounds))
}

fn image_single_pixel() -> DynamicImage {
	RgbaImage::from_pixel(
		1,
		1,
		Rgba([
			0,
			0,
			0,
			255,
		]),
	)
	.into()
}

#[test]
fn border_zeroed_image() {
	let image = DynamicImage::new(0, 0, ColorType::Rgba8);
	assert_eq!(crop(&image), None);
}

#[test]
fn border_blank_image() {
	let image = DynamicImage::new(32, 32, ColorType::Rgba8);
	assert_eq!(crop(&image), None);
}

#[test]
fn border_filled_image() {
	let image: DynamicImage = RgbaImage::from_pixel(
		32,
		32,
		Rgba([
			0,
			0,
			0,
			255,
		]),
	)
	.into();
	assert_eq!(
		crop(&image),
		Some((
			image,
			Bounds {
				left: 0,
				right: 31,
				top: 0,
				bottom: 31,
			}
		))
	);
}

#[test]
fn border_single_pixel_middle_1() {
	let mut image = DynamicImage::new(32, 32, ColorType::Rgba8);
	image.put_pixel(
		16,
		16,
		Rgba([
			0,
			0,
			0,
			255,
		]),
	);
	assert_eq!(
		crop(&image),
		Some((
			image_single_pixel(),
			Bounds {
				left: 16,
				right: 16,
				top: 16,
				bottom: 16,
			}
		))
	);
}

#[test]
fn border_single_pixel_middle_left() {
	let mut image = DynamicImage::new(32, 32, ColorType::Rgba8);
	image.put_pixel(
		0,
		16,
		Rgba([
			0,
			0,
			0,
			255,
		]),
	);
	assert_eq!(
		crop(&image),
		Some((
			image_single_pixel(),
			Bounds {
				left: 0,
				right: 0,
				top: 16,
				bottom: 16,
			}
		))
	);
}

#[test]
fn border_single_pixel_middle_right() {
	let mut image = DynamicImage::new(32, 32, ColorType::Rgba8);
	image.put_pixel(
		31,
		16,
		Rgba([
			0,
			0,
			0,
			255,
		]),
	);
	assert_eq!(
		crop(&image),
		Some((
			image_single_pixel(),
			Bounds {
				left: 31,
				right: 31,
				top: 16,
				bottom: 16,
			}
		))
	);
}

#[test]
fn border_single_pixel_middle_top() {
	let mut image = DynamicImage::new(32, 32, ColorType::Rgba8);
	image.put_pixel(
		16,
		0,
		Rgba([
			0,
			0,
			0,
			255,
		]),
	);
	assert_eq!(
		crop(&image),
		Some((
			image_single_pixel(),
			Bounds {
				left: 16,
				right: 16,
				top: 0,
				bottom: 0,
			}
		))
	);
}

#[test]
fn border_single_pixel_middle_bottom() {
	let mut image = DynamicImage::new(32, 32, ColorType::Rgba8);
	image.put_pixel(
		16,
		31,
		Rgba([
			0,
			0,
			0,
			255,
		]),
	);
	assert_eq!(
		crop(&image),
		Some((
			image_single_pixel(),
			Bounds {
				left: 16,
				right: 16,
				top: 31,
				bottom: 31,
			}
		))
	);
}

#[test]
fn border_single_pixel_top_left() {
	let mut image = DynamicImage::new(32, 32, ColorType::Rgba8);
	image.put_pixel(
		0,
		0,
		Rgba([
			0,
			0,
			0,
			255,
		]),
	);
	assert_eq!(
		crop(&image),
		Some((
			image_single_pixel(),
			Bounds {
				left: 0,
				right: 0,
				top: 0,
				bottom: 0,
			}
		))
	);
}

#[test]
fn border_single_pixel_top_right() {
	let mut image = DynamicImage::new(32, 32, ColorType::Rgba8);
	image.put_pixel(
		31,
		0,
		Rgba([
			0,
			0,
			0,
			255,
		]),
	);
	assert_eq!(
		crop(&image),
		Some((
			image_single_pixel(),
			Bounds {
				left: 31,
				right: 31,
				top: 0,
				bottom: 0,
			}
		))
	);
}

#[test]
fn border_single_pixel_bottom_left() {
	let mut image = DynamicImage::new(32, 32, ColorType::Rgba8);
	image.put_pixel(
		0,
		31,
		Rgba([
			0,
			0,
			0,
			255,
		]),
	);
	assert_eq!(
		crop(&image),
		Some((
			image_single_pixel(),
			Bounds {
				left: 0,
				right: 0,
				top: 31,
				bottom: 31,
			}
		))
	);
}

#[test]
fn border_single_pixel_bottom_right() {
	let mut image = DynamicImage::new(32, 32, ColorType::Rgba8);
	image.put_pixel(
		31,
		31,
		Rgba([
			0,
			0,
			0,
			255,
		]),
	);
	assert_eq!(
		crop(&image),
		Some((
			image_single_pixel(),
			Bounds {
				left: 31,
				right: 31,
				top: 31,
				bottom: 31,
			}
		))
	);
}
