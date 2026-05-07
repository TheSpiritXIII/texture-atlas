use std::convert::Infallible;

use clap::Subcommand;
use texture_atlas::BinaryPacker;
use texture_atlas::Item2;
use texture_atlas::Options2;
use texture_atlas::Packer;
use texture_atlas::PackerOp;
use texture_atlas::PassthroughPacker;
use texture_atlas::Pos2;
use texture_atlas::Rotate2;
use texture_atlas::UniformPacker;

/// Encapsulates every built-in packer type.
#[derive(Subcommand)]
pub enum Algorithm {
	Binary,
	Passthrough,
	Uniform,
}

impl Algorithm {
	pub fn into_packer(self) -> GenericPacker {
		match self {
			Self::Binary => GenericPacker::Binary(BinaryPacker::new()),
			Self::Passthrough => GenericPacker::Passthrough(PassthroughPacker::new()),
			Self::Uniform => GenericPacker::Uniform(UniformPacker::new()),
		}
	}
}

// TODO: Consider making this private. Return impl Packer.
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
