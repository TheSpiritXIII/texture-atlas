use std::convert::Infallible;

use crate::AtlasOptions;
use crate::AtlasRect;
use crate::BinaryPacker;
use crate::Packer;
use crate::PackerOp;
use crate::PassthroughPacker;
use crate::Pos2;

/// Encapsulates every built-in packer.
pub enum GenericPacker<T: AtlasRect> {
	Passthrough(PassthroughPacker<T>),
	Binary(BinaryPacker<T>),
}

impl<Item> Packer<Item> for GenericPacker<Item>
where
	Item: AtlasRect,
{
	type Output = Pos2;
	type Error = Infallible;

	fn add(
		&mut self,
		options: &AtlasOptions,
		item: &Item,
	) -> Result<PackerOp<Self::Output>, Self::Error> {
		match self {
			Self::Passthrough(packer) => packer.add(options, item),
			Self::Binary(packer) => packer.add(options, item),
		}
	}

	fn add_all<T: std::borrow::Borrow<Item>>(
		&mut self,
		options: &AtlasOptions,
		group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, PackerOp<Self::Output>), Self::Error>> {
		// TODO: Avoid extra allocations.
		let items: Vec<_> = match self {
			Self::Passthrough(packer) => packer.add_all(options, group).into_iter().collect(),
			Self::Binary(packer) => packer.add_all(options, group).into_iter().collect(),
		};
		items.into_iter()
	}

	fn add_group<T: std::borrow::Borrow<Item>>(
		&mut self,
		options: &AtlasOptions,
		group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, PackerOp<Self::Output>), Self::Error>> {
		// TODO: Avoid extra allocations.
		let items: Vec<_> = match self {
			Self::Passthrough(packer) => packer.add_group(options, group).into_iter().collect(),
			Self::Binary(packer) => packer.add_group(options, group).into_iter().collect(),
		};
		items.into_iter()
	}
}
