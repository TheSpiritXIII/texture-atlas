use std::borrow::Borrow;
use std::marker::PhantomData;

use crate::AtlasOptions;
use crate::AtlasRect;
use crate::Bin as AtlasBin;
use crate::Packer as AtlasPacker;
use crate::PackerOp;

// An atlas builder which only creates one bin. Does not allocate any heap memory.
pub struct SingleAtlas<Packer, Bin, Item>
where
	Packer: AtlasPacker<Item>,
	Bin: AtlasBin<Item, Params = Packer::Output>,
	Item: AtlasRect,
{
	options: AtlasOptions,
	packer: Packer,
	bin: Option<Bin>,
	phantom: PhantomData<Item>,
}

pub enum SingleAtlasError<BinError, PackerError> {
	Bin(BinError),
	Packer(PackerError),
	MissingBin,
	DoesNotFit,
}

pub type SingleAtlasResult<T, BinError, PackerError> =
	Result<T, SingleAtlasError<BinError, PackerError>>;

#[derive(Debug)]
pub struct SingleAtlasEntry<T> {
	/// The item index from the original slice that was added.
	pub item_index: usize,
	/// The entry data.
	pub output: T,
}

impl<Packer, Bin, Item> SingleAtlas<Packer, Bin, Item>
where
	Packer: AtlasPacker<Item>,
	Bin: AtlasBin<Item, Params = Packer::Output>,
	Item: AtlasRect,
{
	/// Creates a new atlas.
	pub fn new(options: AtlasOptions, packer: Packer) -> Self {
		Self {
			options,
			packer,
			bin: None,
			phantom: PhantomData,
		}
	}

	/// Adds a new item to the atlas. Prefer `add_all`, which allows additional optimizations to
	/// ensure items are tightly packed
	pub fn add(
		&mut self,
		item: &Item,
	) -> SingleAtlasResult<Packer::Output, Bin::Error, Packer::Error> {
		let op = self.packer.add(&self.options, item).map_err(SingleAtlasError::Packer)?;
		let output = Self::add_item_to(&self.options, &mut self.bin, item, op)?;
		Ok(output)
	}

	/// Adds multiple items to the atlas, optimizing the placement of items by more tightly packing
	/// them.
	pub fn add_all<T: Borrow<Item>>(
		&mut self,
		item_list: &[T],
	) -> SingleAtlasResult<Vec<SingleAtlasEntry<Packer::Output>>, Bin::Error, Packer::Error> {
		let mut output = Vec::with_capacity(item_list.len());
		for entry in self.packer.add_all(&self.options, item_list) {
			let (item_index, op) = entry.map_err(SingleAtlasError::Packer)?;
			let item = item_list[item_index].borrow();

			let entry = Self::add_item_to(&self.options, &mut self.bin, item, op)?;
			output.push(SingleAtlasEntry {
				item_index,
				output: entry,
			});
		}
		Ok(output)
	}

	// TODO: add `add_all_array` for nostd
	// TODO: add `add_all_best_effort` and `add_all_array_best_effort` for nostd

	/// Returns the bin generated from added items.
	pub fn build(self) -> Option<Bin> {
		self.bin
	}

	fn add_item_to(
		options: &AtlasOptions,
		bin: &mut Option<Bin>,
		item: &Item,
		op: PackerOp<Packer::Output>,
	) -> SingleAtlasResult<Packer::Output, Bin::Error, Packer::Error> {
		let (bin, params) = match bin {
			Some(bin) => {
				match op {
					PackerOp::NewBin(_) => {
						return Err(SingleAtlasError::DoesNotFit);
					}
					PackerOp::ExistingBin((_, params)) => (bin, params),
				}
			}
			None => {
				// TODO: Might not need to handle this case if we create bin in constructor somehow.
				let PackerOp::NewBin(params) = op else {
					return Err(SingleAtlasError::MissingBin);
				};
				let bin = bin.insert(Bin::new(options.max_width, options.max_height));
				(bin, params)
			}
		};
		bin.item_add(item, &params).map_err(SingleAtlasError::Bin)?;
		Ok(params)
	}
}
