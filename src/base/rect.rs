use crate::util::Rect;
use crate::util::RotatableRect;

/// Represents an axis aligned rectangle to be packed in a bin.
pub trait AtlasRect {
	/// The width size dimension of the rectangle.
	fn width(&self) -> u32;

	/// The height size dimension of the rectangle.
	fn height(&self) -> u32;
}

// Common methods for all `AtlasRect` types.
pub trait AtlasRectExt: AtlasRect {
	fn fits(&self, other: &Rect) -> bool {
		self.width() >= other.width && self.height() >= other.height
	}

	/// Returns the total number of pixels this rectangle takes up.
	fn area(&self) -> u64 {
		self.width() as u64 * self.height() as u64
	}

	/// Returns true if this rectangle has an area of 0.
	fn empty(&self) -> bool {
		self.width() == 0 || self.height() == 0
	}

	/// Returns the dimensions of this rectangle.
	fn dimensions(&self) -> Rect {
		Rect::new(self.width(), self.height())
	}

	/// Returns the dimensions of this rect with width and height inverted.
	fn dimensions_rotated(&self) -> Rect {
		Rect::new(self.height(), self.width())
	}

	/// Returns a rect with the longest dimension being its width and its other being its height.
	fn dimensions_longest(&self) -> RotatableRect {
		if self.width() >= self.height() {
			RotatableRect::new(self.width(), self.height(), false)
		} else {
			RotatableRect::new(self.height(), self.width(), true)
		}
	}
}

impl<T> AtlasRectExt for T where T: AtlasRect {}

/// An axis-aligned rectangle.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Size2 {
	/// The width dimension of the rectangle.
	pub width: u32,

	/// The height dimension of the rectangle.
	pub height: u32,
}

impl Size2 {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width,
			height,
		}
	}
}

impl AtlasRect for Size2 {
	fn width(&self) -> u32 {
		self.width
	}

	fn height(&self) -> u32 {
		self.height
	}
}
