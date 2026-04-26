use std::marker::PhantomData;

use crate::AtlasRect;
use crate::AtlasRectExt;
use crate::Bin as AtlasBin;

/// An algorithm which can be scored.
pub trait Scored {
	/// Returns a value between 0 and 1 for how well an algorithm performed, with 1 being the best.
	fn score(&self) -> f32;
}

/// A delegating bin which tracks a score of the wrapped bin.
pub struct ScoredBin2<Item, Bin>
where
	Item: AtlasRect,
	Bin: AtlasBin<Item>,
{
	bin: Bin,
	score: usize,
	phantom: PhantomData<Item>,
}

impl<Item, Bin> ScoredBin2<Item, Bin>
where
	Item: AtlasRect,
	Bin: AtlasBin<Item>,
{
	pub fn bin(&self) -> &Bin {
		&self.bin
	}
}

impl<Item, Bin> AtlasBin<Item> for ScoredBin2<Item, Bin>
where
	Item: AtlasRect,
	Bin: AtlasBin<Item>,
{
	type Params = Bin::Params;
	type Error = Bin::Error;

	fn new(width: std::num::NonZero<u32>, height: std::num::NonZero<u32>) -> Self {
		Self {
			bin: Bin::new(width, height),
			score: 0,
			phantom: PhantomData,
		}
	}

	fn item_add(&mut self, item: &Item, params: &Self::Params) -> Result<(), Self::Error> {
		self.bin.item_add(item, params)?;
		// TODO: Avoid casting.
		self.score += item.area() as usize;
		Ok(())
	}
}

impl<Item, Bin> AtlasRect for ScoredBin2<Item, Bin>
where
	Item: AtlasRect,
	Bin: AtlasBin<Item>,
{
	fn width(&self) -> u32 {
		self.bin.width()
	}

	fn height(&self) -> u32 {
		self.bin.height()
	}
}

impl<Item, Bin> Scored for ScoredBin2<Item, Bin>
where
	Item: AtlasRect,
	Bin: AtlasBin<Item>,
{
	fn score(&self) -> f32 {
		self.score as f32 / self.bin.area() as f32
	}
}

impl<T> Scored for &[T]
where
	T: Scored,
{
	fn score(&self) -> f32 {
		let mut score = 0.0;
		for item in *self {
			score += item.score();
		}
		score / self.len() as f32
	}
}
