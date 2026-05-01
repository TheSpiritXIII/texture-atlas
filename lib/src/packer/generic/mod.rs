use std::convert::Infallible;

use crate::BinaryPacker;
use crate::Item2;
use crate::Options2;
use crate::Packer;
use crate::PackerOp;
use crate::PassthroughPacker;
use crate::Pos2;
use crate::Rotate2;
use crate::UniformPacker;

/// Encapsulates every built-in packer.
#[derive(Debug)]
pub enum GenericPacker {
	Uniform(UniformPacker),
	Passthrough(PassthroughPacker),
	Binary(BinaryPacker),
}

impl<Item> Packer<Item, Pos2, Options2> for GenericPacker
where
	Item: Item2,
{
	type Error = Infallible;

	fn add(&mut self, options: &Options2, item: &Item) -> Result<PackerOp<Pos2>, Self::Error> {
		match self {
			Self::Passthrough(packer) => packer.add(options, item),
			Self::Binary(packer) => packer.add(options, item),
			Self::Uniform(packer) => packer.add(options, item),
		}
	}

	fn add_all<T: std::borrow::Borrow<Item>>(
		&mut self,
		options: &Options2,
		group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, PackerOp<Pos2>), Self::Error>> {
		// TODO: Avoid extra allocations.
		let items: Vec<_> = match self {
			Self::Binary(packer) => {
				Packer::<Item, Pos2, Options2>::add_all(packer, options, group)
					.into_iter()
					.collect()
			}
			Self::Passthrough(packer) => packer.add_all(options, group).into_iter().collect(),
			Self::Uniform(packer) => packer.add_all(options, group).into_iter().collect(),
		};
		items.into_iter()
	}

	// TODO: Reintroduce add_group
	// fn add_group<T: std::borrow::Borrow<Item>>(
	// 	&mut self,
	// 	options: &Options2,
	// 	group: &[T],
	// ) -> impl IntoIterator<Item = Result<(usize, PackerOp<Pos2>), Self::Error>> {
	// 	// TODO: Avoid extra allocations.
	// 	let items: Vec<_> = match self {
	// 		Self::Binary(packer) => {
	// 			Packer::<Item, Pos2, Options2>::add_group(packer, options, group)
	// 				.into_iter()
	// 				.collect()
	// 		}
	// 		Self::Passthrough(packer) => packer.add_group(options, group).into_iter().collect(),
	// 		Self::Uniform(packer) => packer.add_group(options, group).into_iter().collect(),
	// 	};
	// 	items.into_iter()
	// }
}

impl<Item> Packer<Item, Rotate2, Options2> for GenericPacker
where
	Item: Item2,
{
	type Error = Infallible;

	fn add(&mut self, options: &Options2, item: &Item) -> Result<PackerOp<Rotate2>, Self::Error> {
		match self {
			Self::Binary(packer) => packer.add(options, item),
			Self::Passthrough(packer) => packer.add(options, item),
			Self::Uniform(packer) => packer.add(options, item),
		}
	}

	fn add_all<T: std::borrow::Borrow<Item>>(
		&mut self,
		options: &Options2,
		group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, PackerOp<Rotate2>), Self::Error>> {
		// TODO: Avoid extra allocations.
		let items: Vec<_> = match self {
			Self::Binary(packer) => {
				Packer::<Item, Rotate2, Options2>::add_all(packer, options, group)
					.into_iter()
					.collect()
			}
			Self::Passthrough(packer) => packer.add_all(options, group).into_iter().collect(),
			Self::Uniform(packer) => packer.add_all(options, group).into_iter().collect(),
		};
		items.into_iter()
	}

	// TODO: Reintroduce add_group
	// fn add_group<T: std::borrow::Borrow<Item>>(
	// 	&mut self,
	// 	options: &Options2,
	// 	group: &[T],
	// ) -> impl IntoIterator<Item = Result<(usize, PackerOp<Rotate2>), Self::Error>> {
	// 	// TODO: Avoid extra allocations.
	// 	let items: Vec<_> = match self {
	// 		Self::Binary(packer) => {
	// 			Packer::<Item, Rotate2, Options2>::add_group(packer, options, group)
	// 				.into_iter()
	// 				.collect()
	// 		}
	// 		Self::Passthrough(packer) => packer.add_group(options, group).into_iter().collect(),
	// 		Self::Uniform(packer) => packer.add_group(options, group).into_iter().collect(),
	// 	};
	// 	items.into_iter()
	// }
}
