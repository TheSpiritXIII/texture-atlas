#[cfg(test)]
mod test;

use std::borrow::Borrow;
use std::marker::PhantomData;

use crate::AtlasOptions;
use crate::AtlasRect;
use crate::Packer;
use crate::PackerOp;
use crate::Pos2;

/// A packer that packs every item into its own bin at position (0, 0). This is useful for testing
/// and debugging.
#[derive(Clone, Copy, Debug)]
pub struct PassthroughPacker<Item>
where
	Item: AtlasRect,
{
	phantom: PhantomData<Item>,
}

impl<Item> PassthroughPacker<Item>
where
	Item: AtlasRect,
{
	pub fn new() -> Self {
		Self {
			phantom: PhantomData,
		}
	}
}

impl<Item> Default for PassthroughPacker<Item>
where
	Item: AtlasRect,
{
	fn default() -> Self {
		Self::new()
	}
}

impl<Item> Packer<Item> for PassthroughPacker<Item>
where
	Item: AtlasRect,
{
	type Output = Pos2;
	type Error = ();

	fn add(&mut self, _: &AtlasOptions, _: &Item) -> Result<PackerOp<Self::Output>, Self::Error> {
		Ok(PackerOp::NewBin(Pos2 {
			x: 0,
			y: 0,
		}))
	}

	fn add_all<T: Borrow<Item>>(
		&mut self,
		options: &AtlasOptions,
		group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, PackerOp<Self::Output>), Self::Error>> {
		(0..group.len()).map(|index| {
			let output = self.add(options, group[index].borrow());
			output.map(|x| (index, x))
		})
	}
}
