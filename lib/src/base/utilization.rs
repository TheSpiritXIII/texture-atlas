use std::marker::PhantomData;

use crate::Bin as AtlasBin;
use crate::BinAdd;
use crate::Item2;
use crate::Item2Ext;

/// An item or collection whose space utilization can be measured.
pub trait Utilization {
	/// Returns a value between 0 and 1 for the fraction of space occupied, with 1 being completely
	/// full.
	fn utilization(&self) -> f32;
}

/// A delegating bin which tracks space utilization of the wrapped bin.
pub struct UtilizationBin2<Item, Bin>
where
	Item: Item2,
	Bin: AtlasBin + Item2,
{
	bin: Bin,
	used_area: u64,
	phantom: PhantomData<Item>,
}

impl<Item, Bin> UtilizationBin2<Item, Bin>
where
	Item: Item2,
	Bin: AtlasBin + Item2,
{
	pub fn bin(&self) -> &Bin {
		&self.bin
	}

	pub fn used_area(&self) -> u64 {
		self.used_area
	}
}

impl<Item, Bin> AtlasBin for UtilizationBin2<Item, Bin>
where
	Item: Item2,
	Bin: AtlasBin + Item2,
{
	type Options = Bin::Options;
	type Error = Bin::Error;

	fn new(options: &Self::Options) -> Self {
		Self {
			bin: Bin::new(options),
			used_area: 0,
			phantom: PhantomData,
		}
	}
}

impl<Item, Bin, Params> BinAdd<Item, Params> for UtilizationBin2<Item, Bin>
where
	Item: Item2,
	Bin: AtlasBin + BinAdd<Item, Params> + Item2,
{
	fn item_add(&mut self, item: &Item, params: &Params) -> Result<(), Self::Error> {
		self.bin.item_add(item, params)?;
		self.used_area += item.area();
		Ok(())
	}
}

impl<Item, Bin> Item2 for UtilizationBin2<Item, Bin>
where
	Item: Item2,
	Bin: AtlasBin + Item2,
{
	fn width(&self) -> u32 {
		self.bin.width()
	}

	fn height(&self) -> u32 {
		self.bin.height()
	}
}

impl<Item, Bin> Utilization for UtilizationBin2<Item, Bin>
where
	Item: Item2,
	Bin: AtlasBin + Item2,
{
	fn utilization(&self) -> f32 {
		self.used_area as f32 / self.bin.area() as f32
	}
}

impl<T> Utilization for &[T]
where
	T: Utilization,
{
	fn utilization(&self) -> f32 {
		let mut total_utilization = 0.0;
		for item in *self {
			total_utilization += item.utilization();
		}
		total_utilization / self.len() as f32
	}
}
