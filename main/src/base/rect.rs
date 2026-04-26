use crate::util::Rect;
use crate::util::RotatableRect;

/// Represents an axis aligned rectangle to be packed in a bin.
pub trait AtlasRect {
	/// The width size dimension of the rectangle.
	fn width(&self) -> u32;

	/// The height size dimension of the rectangle.
	fn height(&self) -> u32;
}

/// Describes how an item can fit inside 2D space.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Fit2 {
	/// The item does not fit.
	None,
	/// The item fits exactly, consuming the entire space.
	Total,
	/// The item fits vertically, consuming the entire width.
	Vertical,
	/// The item fits horizontally, consuming the entire height.
	Horizontal,
	/// The item fits inside the bin without consuming the entire width or height.
	Within,
}

impl Fit2 {
	/// Returns true if the item can be placed in the space.
	pub fn fits(&self) -> bool {
		!matches!(*self, Fit2::None)
	}
}

// Common methods for all `AtlasRect` types.
pub trait AtlasRectExt: AtlasRect {
	fn fit2(&self, other: &Size2) -> Fit2 {
		match self.width().cmp(&other.width) {
			std::cmp::Ordering::Equal => {
				match self.height().cmp(&other.height) {
					std::cmp::Ordering::Equal => Fit2::Total,
					std::cmp::Ordering::Greater => Fit2::Vertical,
					std::cmp::Ordering::Less => Fit2::None,
				}
			}
			std::cmp::Ordering::Greater => {
				match self.height().cmp(&other.height) {
					std::cmp::Ordering::Equal => Fit2::Horizontal,
					std::cmp::Ordering::Greater => Fit2::Within,
					std::cmp::Ordering::Less => Fit2::None,
				}
			}
			std::cmp::Ordering::Less => Fit2::None,
		}
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
