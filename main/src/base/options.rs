use std::num::NonZero;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtlasOptions {
	/// The max width of the atlas page.
	pub max_width: NonZero<u32>,
	/// The max height of the atlas page.
	pub max_height: NonZero<u32>,
}

impl AtlasOptions {
	/// Creates options with the given max size.
	pub fn with_max_size(max_width: NonZero<u32>, max_height: NonZero<u32>) -> Self {
		AtlasOptions {
			max_width,
			max_height,
		}
	}
}
