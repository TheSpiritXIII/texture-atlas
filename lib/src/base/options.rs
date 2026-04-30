use std::num::NonZero;

/// Options for 2-dimensional bins.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Options2 {
	/// The max width of the atlas page.
	pub max_width: NonZero<u32>,
	/// The max height of the atlas page.
	pub max_height: NonZero<u32>,
}

impl Options2 {
	/// Creates options with the given max size.
	pub fn with_max_size(max_width: NonZero<u32>, max_height: NonZero<u32>) -> Self {
		Options2 {
			max_width,
			max_height,
		}
	}
}
