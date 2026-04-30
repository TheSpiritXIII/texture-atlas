use std::borrow::Borrow;
use std::convert::Infallible;
use std::iter;

use crate::Item2;
use crate::Options2;
use crate::Packer;
use crate::PackerOp;
use crate::Pos2;

// A packer that always returns `PackerOp::ExistingBin((0, Pos2::new(0, 0)))` when an item is added.
pub(crate) struct AlwaysExistingBinPacker;

impl<Item> Packer<Item, Pos2, Options2> for AlwaysExistingBinPacker
where
	Item: Item2,
{
	type Error = Infallible;

	fn add(
		&mut self,
		_options: &Options2,
		_item: &Item,
	) -> Result<crate::PackerOp<Pos2>, Self::Error> {
		Ok(PackerOp::ExistingBin((0, Pos2::new(0, 0))))
	}

	fn add_all<T: Borrow<Item>>(
		&mut self,
		_options: &Options2,
		_group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, PackerOp<Pos2>), Self::Error>> {
		iter::once(Ok((0, PackerOp::ExistingBin((0, Pos2::new(0, 0))))))
	}
}

// A packer that always returns an error when an item is added.
pub(crate) struct AlwaysErrorPacker;

impl<Item> Packer<Item, Pos2, Options2> for AlwaysErrorPacker
where
	Item: Item2,
{
	type Error = ();

	fn add(
		&mut self,
		_options: &Options2,
		_item: &Item,
	) -> Result<crate::PackerOp<Pos2>, Self::Error> {
		Err(())
	}

	fn add_all<T: Borrow<Item>>(
		&mut self,
		_options: &Options2,
		_group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, PackerOp<Pos2>), Self::Error>> {
		Err(())
	}
}
