use std::num::NonZero;

use crate::Item2;
use crate::Pos2;
use crate::Size2;

/// Options for 2-dimensional bins.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Options2 {
	/// The max width of the bin.
	max_width: NonZero<u32>,
	/// The max height of the bin.
	max_height: NonZero<u32>,
	/// The space between items in the bin. This is only the inner border between items. The
	/// outer border is specified via `margin`.
	spacing: u32,
	/// The margin size around the atlas page. This is only the outer border of the atlas. To
	/// specify the inner border between items, use `spacing`.
	margin: u32,
}

impl Options2 {
	/// Creates options with the given max size.
	pub fn with_max_size(max_width: NonZero<u32>, max_height: NonZero<u32>) -> Self {
		Self {
			max_width,
			max_height,
			spacing: 0,
			margin: 0,
		}
	}

	/// Creates options with a padding around each item. This sets the spacing and margin to the
	/// same size.
	pub fn and_padding(&mut self, size: u32) -> Self {
		self.spacing = size;
		self.margin = size;
		self.clone()
	}

	/// Creates options with the given spacing between items.
	pub fn and_spacing(&mut self, size: u32) -> Self {
		self.spacing = size;
		self.clone()
	}

	/// Creates options with the given margin around the page.
	pub fn and_margin(&mut self, size: u32) -> Self {
		self.margin = size;
		self.clone()
	}

	/// Returns the max width of the bin. Packers should use [`max_logical_width`].
	pub fn max_width(&self) -> u32 {
		self.max_width.get()
	}

	/// Returns the max height of the bin. Packers should use [`max_logical_height`].
	pub fn max_height(&self) -> u32 {
		self.max_height.get()
	}

	/// Returns the max size of the bin. Packers should use [`max_logical_size`].
	pub fn max_size(&self) -> Size2 {
		Size2::new(self.max_width(), self.max_height())
	}

	/// Returns the max logical width of the bin, accounting for margins.
	pub fn max_logical_width(&self) -> u32 {
		self.max_width.get().saturating_sub(self.margin)
	}

	/// Returns the max logical height of the bin, accounting for margins.
	pub fn max_logical_height(&self) -> u32 {
		self.max_height.get().saturating_sub(self.margin)
	}

	/// Returns the max logical size of the bin, accounting for margins.
	pub fn max_logical_size(&self) -> Size2 {
		Size2::new(self.max_logical_height(), self.max_logical_width())
	}

	/// Returns the item width, accounting for spacing.
	pub fn item_width(&self, item: &impl Item2) -> u32 {
		item.width() + self.spacing
	}

	/// Returns the item height, accounting for spacing.
	pub fn item_height(&self, item: &impl Item2) -> u32 {
		item.height() + self.spacing
	}

	/// Returns the item size, accounting for spacing.
	pub fn item_size(&self, item: &impl Item2) -> Size2 {
		Size2::new(self.item_width(item), self.item_height(item))
	}

	pub fn margin(&self) -> Pos2 {
		Pos2::new(self.margin, self.margin)
	}

	pub fn pos(&self, x: u32, y: u32) -> Pos2 {
		Pos2::new(x + self.margin, y + self.margin)
	}
}
