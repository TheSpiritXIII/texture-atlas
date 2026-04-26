use std::num::NonZero;

use crate::AtlasRect;

// TODO: Add options type that goes into the constructor.

/// Represents a single bin in an atlas.
pub trait Bin<Item: AtlasRect>: AtlasRect {
	/// The parameter type when adding an item to the bin.
	type Params;

	/// The error type when adding an item to the bin.
	type Error;

	/// Creates a new bin with the given maximum size.
	fn new(width: NonZero<u32>, height: NonZero<u32>) -> Self;

	/// Adds a new item to the bin at the given position. The given item should not overlap with
	/// any other items previously passed into this function.
	fn item_add(&mut self, item: &Item, params: &Self::Params) -> Result<(), Self::Error>;
}
