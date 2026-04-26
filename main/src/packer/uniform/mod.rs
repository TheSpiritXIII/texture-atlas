#[cfg(test)]
mod test;

use std::borrow::Borrow;
use std::convert::Infallible;
use std::marker::PhantomData;

use crate::AtlasOptions;
use crate::AtlasRect;
use crate::Packer;
use crate::PackerOp;
use crate::Pos2;
use crate::Size2;

/// A packer that packs items a row at a time, wrapping to the next row when it would otherwise
/// overflow. This algorithm is best used when items are uniformly sized since gaps would be
/// impossible. As such, older bins are never reused once a new bin is created.
#[derive(Clone, Debug)]
pub struct UniformPacker<Item>
where
	Item: AtlasRect,
{
	used: Size2,
	highest: u32,
	bin_len: usize,
	phantom: PhantomData<Item>,
}

impl<Item> UniformPacker<Item>
where
	Item: AtlasRect,
{
	pub fn new() -> Self {
		Self {
			used: Size2::new(u32::MAX, u32::MAX),
			highest: 0,
			bin_len: 0,
			phantom: PhantomData,
		}
	}
}

impl<Item> Default for UniformPacker<Item>
where
	Item: AtlasRect,
{
	fn default() -> Self {
		Self::new()
	}
}

impl<Item> Packer<Item> for UniformPacker<Item>
where
	Item: AtlasRect,
{
	type Output = Pos2;
	type Error = Infallible;

	fn add(
		&mut self,
		options: &AtlasOptions,
		item: &Item,
	) -> Result<PackerOp<Self::Output>, Self::Error> {
		let mut y = self.used.height;
		if item.width() > options.max_width.get()
			|| self.used.width > options.max_width.get() - item.width()
		{
			self.used.width = 0;
			self.used.height += self.highest;
			self.highest = 0;

			y = self.used.height;
		}
		if item.height() > options.max_height.get() || y > options.max_height.get() - item.height()
		{
			let op = PackerOp::NewBin(Pos2 {
				x: 0,
				y: 0,
			});

			self.bin_len += 1;
			self.used.height = 0;
			self.used.width = item.width();
			self.highest = item.height();
			return Ok(op);
		}

		let op = PackerOp::ExistingBin((
			self.bin_len - 1,
			Pos2 {
				x: self.used.width,
				y,
			},
		));

		self.used.width += item.width();
		self.highest = self.highest.max(item.height());
		Ok(op)
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
