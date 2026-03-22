#[cfg(test)]
mod test;

use std::marker::PhantomData;

use crate::AtlasOptions;
use crate::AtlasPacker;
use crate::AtlasPackerOp;
use crate::AtlasRect;
use crate::Pos2;

/// A packer that packs every rect into its own bin at position (0, 0). This is useful for testing
/// and debugging.
#[derive(Debug)]
pub struct PassthroughPacker<Rect>
where
	Rect: AtlasRect,
{
	phantom: PhantomData<Rect>,
}

impl<Rect> PassthroughPacker<Rect>
where
	Rect: AtlasRect,
{
	pub fn new() -> Self {
		Self {
			phantom: PhantomData,
		}
	}
}

impl<Rect> Default for PassthroughPacker<Rect>
where
	Rect: AtlasRect,
{
	fn default() -> Self {
		Self::new()
	}
}

impl<Rect> AtlasPacker<Rect> for PassthroughPacker<Rect>
where
	Rect: AtlasRect,
{
	type Output = Pos2;
	type Error = ();

	fn add(
		&mut self,
		_: &AtlasOptions,
		_: &Rect,
	) -> Result<AtlasPackerOp<Self::Output>, Self::Error> {
		Ok(AtlasPackerOp::NewBin(Pos2 {
			x: 0,
			y: 0,
		}))
	}
}
