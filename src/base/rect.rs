use crate::util::Rect;
use crate::util::RotatableRect;

/// Represents an axis aligned rectangle to be packed in a bin.
pub trait AtlasRect {
	/// The width size dimension of the rectangle.
	fn width(&self) -> u32;

	/// The height size dimension of the rectangle.
	fn height(&self) -> u32;
}

pub trait AtlasRectExt {
	/// Returns the total number of pixels this rectangle takes up.
	fn area(&self) -> u64;

	/// Returns true if this rectangle has an area of 0.
	fn empty(&self) -> bool;

	/// Returns the dimensions of this rectangle.
	fn dimensions(&self) -> Rect;

	/// Returns the dimensions of this rect with width and height inverted.
	fn dimensions_rotated(&self) -> Rect;

	/// Returns a rect with the longest dimension being its width and its other being its height.
	fn dimensions_longest(&self) -> RotatableRect;

	/// Returns `dimensions_longest` if `rotate` is true or `dimensions` otherwise.
	fn dimensions_longest_rotated(&self, rotate: bool) -> RotatableRect;
}

impl<T> AtlasRectExt for T
where
	T: AtlasRect,
{
	fn area(&self) -> u64 {
		self.width() as u64 * self.height() as u64
	}

	fn empty(&self) -> bool {
		self.width() == 0 || self.height() == 0
	}

	fn dimensions(&self) -> Rect {
		Rect::new(self.width(), self.height())
	}

	fn dimensions_rotated(&self) -> Rect {
		Rect::new(self.height(), self.width())
	}

	fn dimensions_longest(&self) -> RotatableRect {
		self.dimensions_longest_rotated(true)
	}

	fn dimensions_longest_rotated(&self, rotate: bool) -> RotatableRect {
		if (self.width() >= self.height() && rotate) || !rotate {
			RotatableRect::new(self.width(), self.height(), false)
		} else {
			RotatableRect::new(self.height(), self.width(), true)
		}
	}
}
