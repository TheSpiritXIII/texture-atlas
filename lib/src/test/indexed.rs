use crate::Bin;
use crate::BinAdd;
use crate::Item2;
use crate::Options2;
use crate::Pos2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexedItem<T> {
	pub index: usize,
	pub item: T,
}

impl<T> Item2 for IndexedItem<T>
where
	T: Item2,
{
	fn width(&self) -> u32 {
		self.item.width()
	}

	fn height(&self) -> u32 {
		self.item.height()
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexedBin<T> {
	pub width: u32,
	pub height: u32,
	pub data: Vec<(usize, T)>,
}

impl Bin for IndexedBin<Pos2> {
	type Options = Options2;
	type Error = ();

	fn new(options: &Self::Options) -> Self {
		Self {
			width: options.max_width(),
			height: options.max_height(),
			data: Vec::new(),
		}
	}
}

impl<T> BinAdd<IndexedItem<T>, Pos2> for IndexedBin<Pos2>
where
	T: Item2 + Clone,
{
	fn item_add(&mut self, item: &IndexedItem<T>, params: &Pos2) -> Result<(), Self::Error> {
		self.data.push((item.index, *params));
		Ok(())
	}
}

impl<T> Item2 for IndexedBin<T> {
	fn width(&self) -> u32 {
		self.width
	}

	fn height(&self) -> u32 {
		self.height
	}
}
