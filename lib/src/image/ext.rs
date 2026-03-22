use image::GenericImageView;
use image::Pixel;
use image::Primitive;
use image::SubImage;

/// Represents a boundary of an image.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Margins {
	pub left: u32,
	pub right: u32,
	pub top: u32,
	pub bottom: u32,
}

pub trait ImageExt {
	/// Returns the amount of empty space at the left of the given image.
	fn margin_left(&self) -> u32;

	/// Returns the amount of empty space at the right of the given image.
	fn margin_right(&self) -> u32;

	/// Returns the amount of empty space at the top of the given image.
	fn margin_top(&self) -> u32;

	/// Returns the amount of empty space at the bottom of the given image.
	fn margin_bottom(&self) -> u32;

	/// Returns the empty image boundary. If no boundary is found, returns a boundary where `right`
	/// and `bottom` are 0, `left`` is the image width and `top` is the image height.
	fn margins(&self) -> Margins;

	/// Returns a tuple of the cropped image and the empty borders. If no empty borders are found,
	/// returns `None`.
	fn crop(&self) -> Option<(SubImage<&Self>, Margins)>;

	/// Returns a tuple of the cropped image and the empty borders. The cropped image includes at
	/// least `margin` amount of margins on all sides, if possible. If no empty borders are found,
	/// returns `None`.
	fn crop_margin(&self, margin: u32) -> Option<(SubImage<&Self>, Margins)>;
}

impl<T> ImageExt for T
where
	T: GenericImageView,
{
	fn margin_left(&self) -> u32 {
		for x in 0..self.width() {
			for y in 0..self.height() {
				if self.get_pixel(x, y).alpha() != <T::Pixel as Pixel>::Subpixel::DEFAULT_MIN_VALUE
				{
					return x;
				}
			}
		}
		self.width()
	}

	/// Returns the amount of empty space at the right of the given image.
	fn margin_right(&self) -> u32 {
		for x in (0..self.width()).rev() {
			for y in 0..self.height() {
				if self.get_pixel(x, y).alpha() != <T::Pixel as Pixel>::Subpixel::DEFAULT_MIN_VALUE
				{
					return x;
				}
			}
		}
		0
	}

	fn margin_top(&self) -> u32 {
		for y in 0..self.height() {
			for x in 0..self.width() {
				if self.get_pixel(x, y).alpha() != <T::Pixel as Pixel>::Subpixel::DEFAULT_MIN_VALUE
				{
					return y;
				}
			}
		}
		self.height()
	}

	fn margin_bottom(&self) -> u32 {
		for y in (0..self.height()).rev() {
			for x in 0..self.width() {
				if self.get_pixel(x, y).alpha() != <T::Pixel as Pixel>::Subpixel::DEFAULT_MIN_VALUE
				{
					return y;
				}
			}
		}
		0
	}

	fn margins(&self) -> Margins {
		Margins {
			left: self.margin_left(),
			right: self.margin_right(),
			top: self.margin_top(),
			bottom: self.margin_bottom(),
		}
	}

	fn crop(&self) -> Option<(SubImage<&Self>, Margins)> {
		let margins = self.margins();
		if margins.right == 0
			&& margins.bottom == 0
			&& margins.left == self.width()
			&& margins.top == self.height()
		{
			return None;
		}
		let width = margins.right - margins.left + 1;
		let height = margins.bottom - margins.top + 1;
		Some((self.view(margins.left, margins.top, width, height), margins))
	}

	// TODO: Add unit tests.
	fn crop_margin(&self, margin: u32) -> Option<(SubImage<&Self>, Margins)> {
		let margins = {
			let margins = self.margins();
			Margins {
				left: margins.left.saturating_sub(margin),
				right: margins.right.saturating_add(margin).min(self.width()),
				top: margins.top.saturating_sub(margin),
				bottom: margins.bottom.saturating_add(margin).min(self.height()),
			}
		};
		if margins.right == 0
			&& margins.bottom == 0
			&& margins.left == self.width()
			&& margins.top == self.height()
		{
			return None;
		}
		let width = margins.right - margins.left + 1;
		let height = margins.bottom - margins.top + 1;
		Some((self.view(margins.left, margins.top, width, height), margins))
	}
}
