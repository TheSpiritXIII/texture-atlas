use std::fmt::Debug;
use std::fmt::Formatter;
use std::iter;

use crate::AtlasOptions;
use crate::AtlasRect;

/// Represents operations from a packer.
pub enum AtlasPackerOp<T> {
	/// Indicates to add T to a new bin. Bin indices start at 0 and increments by 1 each time a new
	/// bin is created. After this operation, the same bin can be selected using
	/// [ExistingBin](AtlasPackerOp::ExistingBin).
	NewBin(T),
	/// Indicates to add T to an existing bin. Bin indices start at 0 and must be created with a
	/// [NewBin](AtlasPackerOp::NewBin) operation first.
	ExistingBin((usize, T)),
}

impl<T: Clone> Clone for AtlasPackerOp<T> {
	fn clone(&self) -> Self {
		match self {
			Self::NewBin(x) => Self::NewBin(x.clone()),
			Self::ExistingBin(x) => Self::ExistingBin(x.clone()),
		}
	}
}

impl<T: Debug> Debug for AtlasPackerOp<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NewBin(x) => f.debug_tuple("NewBin").field(x).finish(),
			Self::ExistingBin(x) => f.debug_tuple("ExistingBin").field(x).finish(),
		}
	}
}

impl<T: Eq> Eq for AtlasPackerOp<T> {}

impl<T: PartialEq> PartialEq for AtlasPackerOp<T> {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::NewBin(x), Self::NewBin(y)) => x == y,
			(Self::ExistingBin(x), Self::ExistingBin(y)) => x == y,
			_ => false,
		}
	}
}

/// Packs textures into a bin.
pub trait AtlasPacker<Rect: AtlasRect> {
	/// The output type of the packer. This should contain a list of references of the rects added
	/// with metadata, e.g. position. Most packers will suffice with [Pos2].
	// TODO: Add default. See: https://github.com/rust-lang/rust/issues/29661
	type Output;

	/// The error type of the packer. Generally, this is the error type of the page, but packers may
	/// emit their own errors if needed.
	// TODO: Add default. See: https://github.com/rust-lang/rust/issues/29661
	type Error;

	/// Adds rects to be placed on the atlas pages. `options` is always passed the same value
	/// throughout the lifetime of the packer.
	fn add(
		&mut self,
		options: &AtlasOptions,
		rect: &Rect,
	) -> Result<AtlasPackerOp<Self::Output>, Self::Error>;

	/// Adds rects to be placed on the same atlas page if possible. If any existing page has enough
	/// space for all the rects, they will be placed there. Otherwise, a new page will be created
	/// and the rects will be placed there, overflowing only if needed. `options` is always passed
	/// the same value throughout the lifetime of the packer.
	///
	/// This method returns an iterator containing a tuple with the rect index and the operation
	/// done on it. This is not guaranteed to happen linearly. For example, some packers may use a
	/// heuristic, such as inserting largest rects first.
	///
	/// Implementing this is optional for packers. By default, this has the same behavior as `add`.
	fn add_group(
		&mut self,
		options: &AtlasOptions,
		group: &[&Rect],
	) -> impl IntoIterator<Item = Result<(usize, AtlasPackerOp<Self::Output>), Self::Error>> {
		let mut index = 0;
		// TODO: Custom iterator to ensure properties, e.g. size hint + fuse?
		iter::from_fn(move || {
			if index >= group.len() {
				return None;
			}
			let last_index = index;
			index += 1;

			let output = self.add(options, group[index - 1]);
			Some(output.map(|x| (last_index, x)))
		})
	}
}

/// Represents a position of a rect added to a 2d bin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pos2 {
	/// The x-position where this rect is located in the bin.
	pub x: u32,

	/// The y-position where this rect is located in the bin.
	pub y: u32,
}

impl AsRef<Pos2> for Pos2 {
	fn as_ref(&self) -> &Pos2 {
		self
	}
}
