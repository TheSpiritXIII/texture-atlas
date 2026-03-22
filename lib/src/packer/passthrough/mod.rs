#[cfg(test)]
mod test;

use std::borrow::Borrow;
use std::convert::Infallible;

use crate::Item2;
use crate::Options2;
use crate::Packer;
use crate::PackerOp;
use crate::Pos2;

/// A packer that packs every item into its own bin at position (0, 0). This is useful for testing
/// and debugging.
#[derive(Clone, Copy, Debug)]
pub struct PassthroughPacker;

impl PassthroughPacker {
	pub fn new() -> Self {
		Self {}
	}
}

impl Default for PassthroughPacker {
	fn default() -> Self {
		Self::new()
	}
}

impl<Item, Output> Packer<Item, Output, Options2> for PassthroughPacker
where
	Item: Item2,
	Output: From<Pos2>,
{
	type Error = Infallible;

	fn add(&mut self, options: &Options2, _: &Item) -> Result<PackerOp<Output>, Self::Error> {
		Ok(PackerOp::NewBin(options.margin().into()))
	}

	fn add_all<T: Borrow<Item>>(
		&mut self,
		options: &Options2,
		group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, PackerOp<Output>), Self::Error>> {
		(0..group.len()).map(|index| {
			let output = self.add(options, group[index].borrow());
			output.map(|x| (index, x))
		})
	}
}
