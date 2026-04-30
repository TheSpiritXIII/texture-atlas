use std::borrow::Borrow;
use std::marker::PhantomData;

use crate::Bin as AtlasBin;
use crate::BinAdd;
use crate::Packer as AtlasPacker;
use crate::PackerOp;

// An atlas builder which only creates one bin. Does not allocate any heap memory.
pub struct SingleBuilder<Packer, Bin, Item, Output>
where
	Packer: AtlasPacker<Item, Output, Bin::Options>,
	Bin: AtlasBin<Item> + BinAdd<Item, Output>,
{
	options: Bin::Options,
	packer: Packer,
	bin: Option<Bin>,
	phantom_item: PhantomData<Item>,
	phantom_output: PhantomData<Output>,
}

pub enum SingleBuilderError<BinError, PackerError> {
	Bin(BinError),
	Packer(PackerError),
	MissingBin,
	DoesNotFit,
}

pub type SingleBuilderResult<T, BinError, PackerError> =
	Result<T, SingleBuilderError<BinError, PackerError>>;

#[derive(Debug)]
pub struct SingleBuilderEntry<T> {
	/// The item index from the original slice that was added.
	pub item_index: usize,
	/// The entry data.
	pub output: T,
}

impl<Packer, Bin, Item, Output> SingleBuilder<Packer, Bin, Item, Output>
where
	Packer: AtlasPacker<Item, Output, Bin::Options>,
	Bin: AtlasBin<Item> + BinAdd<Item, Output>,
{
	/// Creates a new atlas.
	pub fn new(options: Bin::Options, packer: Packer) -> Self {
		Self {
			options,
			packer,
			bin: None,
			phantom_item: PhantomData,
			phantom_output: PhantomData,
		}
	}

	/// Adds a new item to the atlas. Prefer `add_all`, which allows additional optimizations to
	/// ensure items are tightly packed
	pub fn add(&mut self, item: &Item) -> SingleBuilderResult<Output, Bin::Error, Packer::Error> {
		let op = self.packer.add(&self.options, item).map_err(SingleBuilderError::Packer)?;
		let output = Self::add_item_to(&self.options, &mut self.bin, item, op)?;
		Ok(output)
	}

	/// Adds multiple items to the atlas, optimizing the placement of items by more tightly packing
	/// them.
	pub fn add_all<T: Borrow<Item>>(
		&mut self,
		item_list: &[T],
	) -> SingleBuilderResult<Vec<SingleBuilderEntry<Output>>, Bin::Error, Packer::Error> {
		let mut output = Vec::with_capacity(item_list.len());
		for entry in self.packer.add_all(&self.options, item_list) {
			let (item_index, op) = entry.map_err(SingleBuilderError::Packer)?;
			let item = item_list[item_index].borrow();

			let entry = Self::add_item_to(&self.options, &mut self.bin, item, op)?;
			output.push(SingleBuilderEntry {
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
		options: &Bin::Options,
		bin: &mut Option<Bin>,
		item: &Item,
		op: PackerOp<Output>,
	) -> SingleBuilderResult<Output, Bin::Error, Packer::Error> {
		let (bin, params) = match bin {
			Some(bin) => {
				match op {
					PackerOp::NewBin(_) => {
						return Err(SingleBuilderError::DoesNotFit);
					}
					PackerOp::ExistingBin((_, params)) => (bin, params),
				}
			}
			None => {
				// TODO: Might not need to handle this case if we create bin in constructor somehow.
				let PackerOp::NewBin(params) = op else {
					return Err(SingleBuilderError::MissingBin);
				};
				let bin = bin.insert(Bin::new(options));
				(bin, params)
			}
		};
		bin.item_add(item, &params).map_err(SingleBuilderError::Bin)?;
		Ok(params)
	}
}
