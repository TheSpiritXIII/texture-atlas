use std::borrow::Borrow;
use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::Bin as AtlasBin;
use crate::BinAdd;
use crate::Packer as AtlasPacker;
use crate::PackerOp;

pub enum BuilderError<BinError, PackerError> {
	Bin(BinError),
	Packer(PackerError),
}

impl<BinError, PackerError> Display for BuilderError<BinError, PackerError>
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

impl<BinError, PackerError> Debug for BuilderError<BinError, PackerError>
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

pub type BuilderResult<T, BinError, PackerError> = Result<T, BuilderError<BinError, PackerError>>;

#[derive(Debug)]
pub struct BuilderAdd<T> {
	/// The bin index of the added entry.
	pub bin_index: usize,
	/// The entry data.
	pub output: T,
}

impl<T> BuilderAdd<T> {
	pub(crate) fn with_item_index(self, item_index: usize) -> BuilderAddMulti<T> {
		BuilderAddMulti {
			bin_index: self.bin_index,
			item_index,
			output: self.output,
		}
	}
}

#[derive(Debug)]
#[cfg_attr(
	feature = "serde",
	derive(
		Serialize,
		Deserialize
	)
)]
pub struct BuilderAddMulti<T> {
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
pub struct DynamicBuilder<Packer, Bin, Item, Output>
where
	Packer: AtlasPacker<Item, Output, Bin::Options>,
	Bin: AtlasBin<Item> + BinAdd<Item, Output>,
{
	options: Bin::Options,
	packer: Packer,
	bin_list: Vec<Bin>,
	phantom_item: PhantomData<Item>,
	phantom_output: PhantomData<Output>,
}

impl<Packer, Bin, Item, Output> DynamicBuilder<Packer, Bin, Item, Output>
where
	Packer: AtlasPacker<Item, Output, Bin::Options>,
	Bin: AtlasBin<Item> + BinAdd<Item, Output>,
{
	pub fn new(options: Bin::Options, packer: Packer) -> Self {
		Self {
			options,
			packer,
			bin_list: Vec::new(),
			phantom_item: PhantomData,
			phantom_output: PhantomData,
		}
	}

	pub fn add(
		&mut self,
		item: &Item,
	) -> BuilderResult<BuilderAdd<Output>, Bin::Error, Packer::Error> {
		let op = self.packer.add(&self.options, item).map_err(BuilderError::Packer)?;
		let output = Self::add_item_to(&self.options, &mut self.bin_list, item, op)?;
		Ok(output)
	}

	pub fn add_all<T: Borrow<Item>>(
		&mut self,
		item_list: &[T],
	) -> BuilderResult<Vec<BuilderAddMulti<Output>>, Bin::Error, Packer::Error> {
		let mut output = Vec::new();
		for entry in self.packer.add_all(&self.options, item_list) {
			let (item_index, op) = entry.map_err(BuilderError::Packer)?;
			let item = item_list[item_index].borrow();

			let entry = Self::add_item_to(&self.options, &mut self.bin_list, item, op)?;
			output.push(entry.with_item_index(item_index));
		}
		Ok(output)
	}

	pub fn add_group<T: Borrow<Item>>(
		&mut self,
		item_list: &[&Item],
	) -> BuilderResult<Vec<BuilderAddMulti<Output>>, Bin::Error, Packer::Error> {
		let mut output = Vec::new();
		for entry in self.packer.add_group(&self.options, item_list) {
			let (item_index, op) = entry.map_err(BuilderError::Packer)?;
			let item = item_list[item_index];

			let entry = Self::add_item_to(&self.options, &mut self.bin_list, item, op)?;
			output.push(entry.with_item_index(item_index));
		}
		Ok(output)
	}

	fn add_item_to(
		options: &Bin::Options,
		bin_list: &mut Vec<Bin>,
		item: &Item,
		op: PackerOp<Output>,
	) -> BuilderResult<BuilderAdd<Output>, Bin::Error, Packer::Error> {
		let (index, params) = match op {
			PackerOp::NewBin(params) => {
				let bin = Bin::new(options);
				bin_list.push(bin);
				let last_index = bin_list.len() - 1;
				(last_index, params)
			}
			PackerOp::ExistingBin((bin, params)) => (bin, params),
		};
		bin_list[index].item_add(item, &params).map_err(BuilderError::Bin)?;
		Ok(BuilderAdd {
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
