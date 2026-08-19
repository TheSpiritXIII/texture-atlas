use std::borrow::Borrow;
use std::fmt::Debug;
use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;
use thiserror::Error;

use crate::Bin as AtlasBin;
use crate::BinAdd;
use crate::Packer as AtlasPacker;
use crate::PackerOp;

#[derive(Error, Debug)]
pub enum BuilderError<BinError, PackerError> {
	#[error("Bin error: {0}")]
	Bin(#[source] BinError),
	#[error("Packer error: {0}")]
	Packer(#[source] PackerError),
}

pub type BuilderResult<T, BinError, PackerError> = Result<T, BuilderError<BinError, PackerError>>;

#[derive(Debug)]
pub struct BuilderAdd<T> {
	/// The bin index of the added entry.
	pub bin_index: usize,
	/// The entry data.
	pub layout: T,
}

impl<T> BuilderAdd<T> {
	pub(crate) fn with_item_index(self, item_index: usize) -> BuilderAddMulti<T> {
		BuilderAddMulti {
			bin_index: self.bin_index,
			item_index,
			layout: self.layout,
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
	pub layout: T,
}

// TODO: Add static atlas variant.

// TODO: Add unit tests.

/// An atlas builder which allows unlimited bins.
pub struct DynamicBuilder<Packer, Bin, Item, Layout>
where
	Packer: AtlasPacker<Item, Layout, Bin::Options>,
	Bin: AtlasBin + BinAdd<Item, Layout>,
{
	options: Bin::Options,
	packer: Packer,
	bin_list: Vec<Bin>,
	phantom_item: PhantomData<Item>,
	phantom_layout: PhantomData<Layout>,
}

impl<Packer, Bin, Item, Layout> DynamicBuilder<Packer, Bin, Item, Layout>
where
	Packer: AtlasPacker<Item, Layout, Bin::Options>,
	Bin: AtlasBin + BinAdd<Item, Layout>,
{
	pub fn new(options: Bin::Options, packer: Packer) -> Self {
		Self {
			options,
			packer,
			bin_list: Vec::new(),
			phantom_item: PhantomData,
			phantom_layout: PhantomData,
		}
	}

	pub fn add(
		&mut self,
		item: &Item,
	) -> BuilderResult<BuilderAdd<Layout>, Bin::Error, Packer::Error> {
		let op = self.packer.add(&self.options, item).map_err(BuilderError::Packer)?;
		let layout = Self::add_item_to(&self.options, &mut self.bin_list, item, op)?;
		Ok(layout)
	}

	pub fn add_all<T: Borrow<Item>>(
		&mut self,
		item_list: &[T],
	) -> BuilderResult<Vec<BuilderAddMulti<Layout>>, Bin::Error, Packer::Error> {
		let mut layout_list = Vec::new();
		for entry in self.packer.add_all(&self.options, item_list) {
			let (item_index, op) = entry.map_err(BuilderError::Packer)?;
			let item = item_list[item_index].borrow();

			let entry = Self::add_item_to(&self.options, &mut self.bin_list, item, op)?;
			layout_list.push(entry.with_item_index(item_index));
		}
		Ok(layout_list)
	}

	fn add_item_to(
		options: &Bin::Options,
		bin_list: &mut Vec<Bin>,
		item: &Item,
		op: PackerOp<Layout>,
	) -> BuilderResult<BuilderAdd<Layout>, Bin::Error, Packer::Error> {
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
			layout: params,
		})
	}

	pub fn bin_list(&self) -> &[Bin] {
		&self.bin_list
	}

	pub fn build(self) -> Vec<Bin> {
		self.bin_list
	}
}
