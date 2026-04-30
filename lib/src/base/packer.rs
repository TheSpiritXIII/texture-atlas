use std::borrow::Borrow;
use std::fmt::Debug;
use std::fmt::Formatter;

use crate::AtlasOptions;
use crate::AtlasRect;

/// Represents operations from a packer.
pub enum PackerOp<T> {
	/// Indicates to add T to a new bin. Bin indices start at 0 and increments by 1 each time a new
	/// bin is created. After this operation, the same bin can be selected using
	/// [ExistingBin](PackerOp::ExistingBin).
	NewBin(T),
	/// Indicates to add T to an existing bin. Bin indices start at 0 and must be created with a
	/// [NewBin](PackerOp::NewBin) operation first.
	ExistingBin((usize, T)),
}

impl<T: Clone> Clone for PackerOp<T> {
	fn clone(&self) -> Self {
		match self {
			Self::NewBin(x) => Self::NewBin(x.clone()),
			Self::ExistingBin(x) => Self::ExistingBin(x.clone()),
		}
	}
}

impl<T: Debug> Debug for PackerOp<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NewBin(x) => f.debug_tuple("NewBin").field(x).finish(),
			Self::ExistingBin(x) => f.debug_tuple("ExistingBin").field(x).finish(),
		}
	}
}

impl<T: Eq> Eq for PackerOp<T> {}

impl<T: PartialEq> PartialEq for PackerOp<T> {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::NewBin(x), Self::NewBin(y)) => x == y,
			(Self::ExistingBin(x), Self::ExistingBin(y)) => x == y,
			_ => false,
		}
	}
}

/// Packs textures into a bin.
///
/// `Item` is the input that gets added to the bin.
///
/// `Output` is the output after items are added to the bin. This should contain a list of
/// references of the items added with metadata, e.g. position. Most packers will suffice with
/// [`Pos2`](crate::Pos2).
pub trait Packer<Item: AtlasRect, Output> {
	/// The error type of the packer. Generally, this is the error type of the page, but packers may
	/// emit their own errors if needed.
	// TODO: Add default. See: https://github.com/rust-lang/rust/issues/29661
	type Error;

	/// Adds items to be placed on any available bin. `options` is always passed the same value
	/// throughout the lifetime of the packer.
	fn add(&mut self, options: &AtlasOptions, item: &Item)
	-> Result<PackerOp<Output>, Self::Error>;

	/// Adds items to be placed on any available bin, optimizing the placement of items to reduce
	/// the total number of bins. `options` is always passed the same value throughout the lifetime
	/// of the packer.
	///
	/// This method returns an iterator containing a tuple with the item index and the operation
	/// done on it. This is not guaranteed to be linear. For example, some packers may use a
	/// heuristic such as inserting largest items first.
	fn add_all<T: Borrow<Item>>(
		&mut self,
		options: &AtlasOptions,
		group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, PackerOp<Output>), Self::Error>>;

	/// Adds items to be placed on any available bin, prioritizing adding all given items to the
	/// same bin. If a single bin does not have enough space, a new bin will be created and the
	/// items will be placed there, overflowing only if needed. `options` is always passed the same
	/// value throughout the lifetime of the packer.
	///
	/// This method returns an iterator containing a tuple with the item index and the operation
	/// done on it. This is not guaranteed to be linear. For example, some packers may use a
	/// heuristic such as inserting largest items first.
	///
	/// Implementing this is optional for packers. By default, this has the same behavior as
	/// `add_all`.
	fn add_group<T: Borrow<Item>>(
		&mut self,
		options: &AtlasOptions,
		group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, PackerOp<Output>), Self::Error>> {
		self.add_all(options, group)
	}
}
