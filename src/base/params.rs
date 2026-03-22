/// Represents a position of a rect added to a 2d bin.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
