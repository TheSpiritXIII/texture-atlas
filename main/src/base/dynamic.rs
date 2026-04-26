use std::borrow::Borrow;
use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::AtlasOptions;
use crate::AtlasRect;
use crate::Bin as AtlasBin;
use crate::Packer as AtlasPacker;
use crate::PackerOp;

pub enum AtlasError<BinError, PackerError> {
	Bin(BinError),
	Packer(PackerError),
}

impl<BinError, PackerError> Display for AtlasError<BinError, PackerError>
where
	BinError: Display,
	PackerError: Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Bin(e) => write!(f, "Bin error: {}", e),
			Self::Packer(e) => write!(f, "Packer error: {}", e),
		}
	}
}

impl<BinError, PackerError> Debug for AtlasError<BinError, PackerError>
where
	BinError: Debug,
	PackerError: Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Bin(e) => f.debug_tuple("Bin").field(e).finish(),
			Self::Packer(e) => f.debug_tuple("Packer").field(e).finish(),
		}
	}
}

pub type AtlasResult<T, BinError, PackerError> = Result<T, AtlasError<BinError, PackerError>>;

#[derive(Debug)]
pub struct AtlasAdd<T> {
	/// The bin index of the added entry.
	pub bin_index: usize,
	/// The entry data.
	pub output: T,
}

impl<T> AtlasAdd<T> {
	pub(crate) fn with_item_index(self, item_index: usize) -> AtlasAddMulti<T> {
		AtlasAddMulti {
			bin_index: self.bin_index,
			item_index,
			output: self.output,
		}
	}
}

#[derive(Debug)]
pub struct AtlasAddMulti<T> {
	/// The bin index of the added entry.
	pub bin_index: usize,
	/// The item index from the original slice that was added.
	pub item_index: usize,
	/// The entry data.
	pub output: T,
}

// TODO: Add static atlas variant.

// TODO: Add unit tests.

/// An atlas builder which allows unlimited bins.
pub struct DynamicAtlas<Packer, Bin, Item>
where
	Packer: AtlasPacker<Item>,
	Bin: AtlasBin<Item, Params = Packer::Output>,
	Item: AtlasRect,
{
	options: AtlasOptions,
	packer: Packer,
	bin_list: Vec<Bin>,
	phantom: PhantomData<Item>,
}

impl<Packer, Bin, Item> DynamicAtlas<Packer, Bin, Item>
where
	Packer: AtlasPacker<Item>,
	Bin: AtlasBin<Item, Params = Packer::Output>,
	Item: AtlasRect,
{
	pub fn new(options: AtlasOptions, packer: Packer) -> Self {
		Self {
			options,
			packer,
			phantom: PhantomData,
			bin_list: Vec::new(),
		}
	}

	pub fn add(
		&mut self,
		item: &Item,
	) -> AtlasResult<AtlasAdd<Packer::Output>, Bin::Error, Packer::Error> {
		let op = self.packer.add(&self.options, item).map_err(AtlasError::Packer)?;
		let output = Self::add_item_to(&self.options, &mut self.bin_list, item, op)?;
		Ok(output)
	}

	pub fn add_all<T: Borrow<Item>>(
		&mut self,
		item_list: &[T],
	) -> AtlasResult<Vec<AtlasAddMulti<Packer::Output>>, Bin::Error, Packer::Error> {
		let mut output = Vec::new();
		for entry in self.packer.add_all(&self.options, item_list) {
			let (item_index, op) = entry.map_err(AtlasError::Packer)?;
			let item = item_list[item_index].borrow();

			let entry = Self::add_item_to(&self.options, &mut self.bin_list, item, op)?;
			output.push(entry.with_item_index(item_index));
		}
		Ok(output)
	}

	pub fn add_group<T: Borrow<Item>>(
		&mut self,
		item_list: &[&Item],
	) -> AtlasResult<Vec<AtlasAddMulti<Packer::Output>>, Bin::Error, Packer::Error> {
		let mut output = Vec::new();
		for entry in self.packer.add_group(&self.options, item_list) {
			let (item_index, op) = entry.map_err(AtlasError::Packer)?;
			let item = item_list[item_index];

			let entry = Self::add_item_to(&self.options, &mut self.bin_list, item, op)?;
			output.push(entry.with_item_index(item_index));
		}
		Ok(output)
	}

	fn add_item_to(
		options: &AtlasOptions,
		bin_list: &mut Vec<Bin>,
		item: &Item,
		op: PackerOp<Packer::Output>,
	) -> AtlasResult<AtlasAdd<Packer::Output>, Bin::Error, Packer::Error> {
		let (index, params) = match op {
			PackerOp::NewBin(params) => {
				let bin = Bin::new(options.max_width, options.max_height);
				bin_list.push(bin);
				let last_index = bin_list.len() - 1;
				(last_index, params)
			}
			PackerOp::ExistingBin((bin, params)) => (bin, params),
		};
		bin_list[index].item_add(item, &params).map_err(AtlasError::Bin)?;
		Ok(AtlasAdd {
			bin_index: index,
			output: params,
		})
	}

	pub fn bin_list(&self) -> &[Bin] {
		&self.bin_list
	}

	pub fn build(self) -> Vec<Bin> {
		self.bin_list
	}
}
