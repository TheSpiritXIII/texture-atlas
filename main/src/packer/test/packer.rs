use std::borrow::Borrow;
use std::convert::Infallible;
use std::iter;

use crate::AtlasOptions;
use crate::AtlasRect;
use crate::Packer;
use crate::PackerOp;
use crate::Pos2;

// A packer that always returns `PackerOp::ExistingBin((0, Pos2::new(0, 0)))` when an item is added.
pub(crate) struct AlwaysExistingBinPacker;

impl<Item> Packer<Item> for AlwaysExistingBinPacker
where
	Item: AtlasRect,
{
	type Output = Pos2;
	type Error = Infallible;

	fn add(
		&mut self,
		_options: &AtlasOptions,
		_item: &Item,
	) -> Result<crate::PackerOp<Self::Output>, Self::Error> {
		Ok(PackerOp::ExistingBin((0, Pos2::new(0, 0))))
	}

	fn add_all<T: Borrow<Item>>(
		&mut self,
		_options: &AtlasOptions,
		_group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, PackerOp<Self::Output>), Self::Error>> {
		iter::once(Ok((0, PackerOp::ExistingBin((0, Pos2::new(0, 0))))))
	}
}

// A packer that always returns an error when an item is added.
pub(crate) struct AlwaysErrorPacker;

impl<Item> Packer<Item> for AlwaysErrorPacker
where
	Item: AtlasRect,
{
	type Output = Pos2;
	type Error = ();

	fn add(
		&mut self,
		_options: &AtlasOptions,
		_item: &Item,
	) -> Result<crate::PackerOp<Self::Output>, Self::Error> {
		Err(())
	}

	fn add_all<T: Borrow<Item>>(
		&mut self,
		_options: &AtlasOptions,
		_group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, PackerOp<Self::Output>), Self::Error>> {
		Err(())
	}
}
