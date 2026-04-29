#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;

/// Represents a position of a rect added to a 2D bin.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
	feature = "serde",
	derive(
		Serialize,
		Deserialize
	)
)]
pub struct Pos2 {
	/// The x-position where this rect is located in the bin.
	pub x: u32,

	/// The y-position where this rect is located in the bin.
	pub y: u32,
}

impl Pos2 {
	pub fn new(x: u32, y: u32) -> Self {
		Self {
			x,
			y,
		}
	}
}

impl From<&Pos2> for Pos2 {
	fn from(pos: &Pos2) -> Self {
		*pos
	}
}

/// Represents the position of a rect added to a 2D bin, possibly rotated.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
	feature = "serde",
	derive(
		Serialize,
		Deserialize
	)
)]
pub struct Rotate2 {
	/// The position of the rect.
	#[cfg_attr(
		feature = "serde",
		serde(flatten)
	)]
	pub pos: Pos2,

	/// Whether the item was rotated 90 degrees clockwise.
	pub rotate: bool,
}

impl From<Pos2> for Rotate2 {
	fn from(pos: Pos2) -> Self {
		Rotate2 {
			pos,
			rotate: false,
		}
	}
}
