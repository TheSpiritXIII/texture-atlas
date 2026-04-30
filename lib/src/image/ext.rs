use image::DynamicImage;
use image::GenericImageView;
use image::Pixel;
use image::SubImage;

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
