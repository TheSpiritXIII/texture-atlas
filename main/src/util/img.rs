use std::borrow::Borrow;

use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use image::Pixel;
use image::Rgb;
use image::SubImage;

use crate::AtlasBin;
use crate::AtlasRect;
use crate::AtlasRectExt;

// TODO: Put all these methods into an Ext trait so we could reuse it for ImageBuffer.

/// Returns the amount of empty space at the left of the given image.
pub fn border_left(image: &DynamicImage) -> u32 {
	for x in 0..image.width() {
		for y in 0..image.height() {
			if image.get_pixel(x, y).alpha() != 0 {
				return x;
			}
		}
	}
	image.width()
}

/// Returns the amount of empty space at the right of the given image.
pub fn border_right(image: &DynamicImage) -> u32 {
	for x in (0..image.width()).rev() {
		for y in 0..image.height() {
			if image.get_pixel(x, y).alpha() != 0 {
				return x;
			}
		}
	}
	0
}

/// Returns the amount of empty space at the top of the given image.
pub fn border_top(image: &DynamicImage) -> u32 {
	for y in 0..image.height() {
		for x in 0..image.width() {
			if image.get_pixel(x, y).alpha() != 0 {
				return y;
			}
		}
	}
	image.height()
}

/// Returns the amount of empty space at the bottom of the given image.
pub fn border_bottom(image: &DynamicImage) -> u32 {
	for y in (0..image.height()).rev() {
		for x in 0..image.width() {
			if image.get_pixel(x, y).alpha() != 0 {
				return y;
			}
		}
	}
	0
}

/// Represents a boundary of an image.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bounds {
	pub left: u32,
	pub right: u32,
	pub top: u32,
	pub bottom: u32,
}

/// Returns the empty image boundary. If no boundary is found, returns a boundary where `right` and
/// `bottom` are 0, `left`` is the image width and `top` is the image height.
pub fn border(image: &DynamicImage) -> Bounds {
	Bounds {
		left: border_left(image),
		right: border_right(image),
		top: border_top(image),
		bottom: border_bottom(image),
	}
}

/// Returns a tuple of the cropped image and the empty borders. If no empty borders are found,
/// returns `None`.
pub fn border_crop(image: &DynamicImage) -> Option<(SubImage<&DynamicImage>, Bounds)> {
	let bounds = border(image);
	if bounds.right == 0
		&& bounds.bottom == 0
		&& bounds.left == image.width()
		&& bounds.top == image.height()
	{
		return None;
	}
	let width = bounds.right - bounds.left + 1;
	let height = bounds.bottom - bounds.top + 1;
	Some((image.view(bounds.left, bounds.top, width, height), bounds))
}

pub(crate) fn image_from_bin<T>(rect_list: &[T], bin: &AtlasBin) -> DynamicImage
where
	T: AtlasRect + Borrow<DynamicImage>,
{
	let dimensions = bin.dimensions();
	let mut image = DynamicImage::new_rgba8(dimensions.width, dimensions.height);

	for reference in &bin.part_list {
		let texture = &rect_list[reference.rect_index];
		if !reference.rotate {
			for x in 0..AtlasRect::width(texture) {
				for y in 0..AtlasRect::height(texture) {
					let pixel = texture.borrow().get_pixel(x, y);
					image.put_pixel(reference.x + x, reference.y + y, pixel);
				}
			}
		} else {
			for x in 0..AtlasRect::width(texture) {
				for y in 0..AtlasRect::height(texture) {
					let pixel = texture.borrow().get_pixel(x, y);
					image.put_pixel(
						reference.x + (AtlasRect::height(texture) - 1 - y),
						reference.y + x,
						pixel,
					);
				}
			}
		}
	}
	image
}

#[derive(Debug)]
struct Hsv {
	data: [u8; 3],
}

impl Hsv {
	fn to_rgb(&self) -> Rgb<u8> {
		let sat = self.data[1] as f32 / u8::MAX as f32;
		let val = self.data[2] as f32 / u8::MAX as f32;

		let chroma = val * sat;
		let h_prime = self.data[0] as f32 / u8::MAX as f32 * (359.0 / 60.0);
		let x = chroma * (1.0 - (h_prime % 2.0 - 1.0).abs());

		let result: [f32; 3] = match h_prime as isize {
			0 => {
				[
					chroma,
					x,
					0.0,
				]
			}
			1 => {
				[
					x,
					chroma,
					0.0,
				]
			}
			2 => {
				[
					0.0,
					chroma,
					x,
				]
			}
			3 => {
				[
					0.0,
					x,
					chroma,
				]
			}
			4 => {
				[
					x,
					0.0,
					chroma,
				]
			}
			5 => {
				[
					chroma,
					0.0,
					x,
				]
			}
			_ => {
				[
					0.0,
					0.0,
					0.0,
				]
			}
		};

		let m = val - chroma;
		Rgb::<u8>([
			((result[0] + m) * u8::MAX as f32) as u8,
			((result[1] + m) * u8::MAX as f32) as u8,
			((result[2] + m) * u8::MAX as f32) as u8,
		])
	}
}

pub(crate) fn colors_weight(len: usize) -> f32 {
	(1f32 / len as f32) * 255f32
}

pub(crate) fn colors_from_bin<T>(color_weight: f32, rect_list: &[T], bin: &AtlasBin) -> DynamicImage
where
	T: AtlasRect,
{
	let mut color_current = Hsv {
		data: [
			0,
			255,
			255,
		],
	};

	let mut image = DynamicImage::new_rgba8(bin.dimensions.width, bin.dimensions.height);

	for reference in &bin.part_list {
		color_current.data[0] = (reference.rect_index as f32 * color_weight) as u8;

		let rotate = reference.rotate;
		let dimensions = if rotate {
			rect_list[reference.rect_index].dimensions_rotated()
		} else {
			rect_list[reference.rect_index].dimensions()
		};

		for x in reference.x..(reference.x + dimensions.width) {
			for y in reference.y..(reference.y + dimensions.height) {
				image.put_pixel(x, y, color_current.to_rgb().to_rgba());
			}
		}
	}
	image
}
