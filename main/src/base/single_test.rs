use std::num::NonZero;

use crate::AlwaysErrorPacker;
use crate::AlwaysExistingBinPacker;
use crate::AtlasOptions;
use crate::AtlasRect;
use crate::Bin;
use crate::BinAdd;
use crate::Pos2;
use crate::SingleAtlas;
use crate::SingleAtlasError;
use crate::Size2;
use crate::UniformPacker;

fn new_options() -> AtlasOptions {
	AtlasOptions::with_max_size(NonZero::new(1024).unwrap(), NonZero::new(1024).unwrap())
}

// TODO: Might be useful for other tests. Maybe even code?
struct IndexedItem<T> {
	index: usize,
	item: T,
}

impl<T> AtlasRect for IndexedItem<T>
where
	T: AtlasRect,
{
	fn width(&self) -> u32 {
		self.item.width()
	}

	fn height(&self) -> u32 {
		self.item.height()
	}
}

// TODO: Might be useful for other tests.
struct IndexedBin<T> {
	width: NonZero<u32>,
	height: NonZero<u32>,
	data: Vec<(usize, T)>,
}

impl<T> Bin<IndexedItem<T>> for IndexedBin<Pos2>
where
	T: AtlasRect + Clone,
{
	type Error = ();

	fn new(width: NonZero<u32>, height: NonZero<u32>) -> Self {
		Self {
			width,
			height,
			data: Vec::new(),
		}
	}
}

impl<T> BinAdd<IndexedItem<T>, Pos2> for IndexedBin<Pos2>
where
	T: AtlasRect + Clone,
{
	fn item_add(&mut self, item: &IndexedItem<T>, params: &Pos2) -> Result<(), Self::Error> {
		self.data.push((item.index, *params));
		Ok(())
	}
}

impl<T> AtlasRect for IndexedBin<T> {
	fn width(&self) -> u32 {
		self.width.get()
	}

	fn height(&self) -> u32 {
		self.height.get()
	}
}

#[test]
fn empty() {
	let packer = UniformPacker::new();
	let atlas = SingleAtlas::<_, _, IndexedItem<Size2>, Pos2>::new(new_options(), packer);

	let bin: Option<IndexedBin<Pos2>> = atlas.build();
	assert!(bin.is_none());
}

#[test]
fn missing_bin() {
	let mut atlas =
		SingleAtlas::<_, IndexedBin<Pos2>, _, Pos2>::new(new_options(), AlwaysExistingBinPacker);

	// TODO: Switch to assert_matches.
	assert!(matches!(
		atlas.add(&IndexedItem {
			index: 0,
			item: Size2 {
				width: 1,
				height: 1
			},
		}),
		Err(SingleAtlasError::MissingBin)
	));
}

#[test]
fn packer_error() {
	let mut atlas =
		SingleAtlas::<_, IndexedBin<Pos2>, _, Pos2>::new(new_options(), AlwaysErrorPacker);

	// TODO: Switch to assert_matches.
	assert!(matches!(
		atlas.add(&IndexedItem {
			index: 0,
			item: Size2 {
				width: 1,
				height: 1
			},
		}),
		Err(SingleAtlasError::Packer(()))
	));
}

#[test]
fn add_single_once() {
	let packer = UniformPacker::new();
	let mut atlas = SingleAtlas::<_, _, IndexedItem<Size2>, Pos2>::new(new_options(), packer);

	assert!(
		atlas
			.add(&IndexedItem {
				index: 0,
				item: Size2 {
					width: 1,
					height: 1
				},
			})
			.is_ok()
	);
	let bin: Option<IndexedBin<Pos2>> = atlas.build();
	assert!(bin.is_some());
	assert_eq!(
		bin.unwrap().data,
		Vec::from([
			(
				0,
				Pos2 {
					x: 0,
					y: 0
				}
			)
		])
	);
}

#[test]
fn add_single_many() {
	let packer = UniformPacker::new();
	let mut atlas = SingleAtlas::<_, _, IndexedItem<Size2>, Pos2>::new(new_options(), packer);

	assert!(
		atlas
			.add(&IndexedItem {
				index: 0,
				item: Size2 {
					width: 1,
					height: 1
				}
			})
			.is_ok()
	);
	assert!(
		atlas
			.add(&IndexedItem {
				index: 1,
				item: Size2 {
					width: 1,
					height: 1
				}
			})
			.is_ok()
	);
	assert!(
		atlas
			.add(&IndexedItem {
				index: 2,
				item: Size2 {
					width: 1,
					height: 1
				}
			})
			.is_ok()
	);
	let bin: Option<IndexedBin<Pos2>> = atlas.build();
	assert!(bin.is_some());
	assert_eq!(
		bin.unwrap().data,
		Vec::from([
			(
				0,
				Pos2 {
					x: 0,
					y: 0
				}
			),
			(
				1,
				Pos2 {
					x: 1,
					y: 0
				}
			),
			(
				2,
				Pos2 {
					x: 2,
					y: 0
				}
			)
		])
	);
}

#[test]
fn add_all_single_once() {
	let packer = UniformPacker::new();
	let mut atlas = SingleAtlas::<_, _, IndexedItem<Size2>, Pos2>::new(new_options(), packer);

	assert!(
		atlas
			.add_all(&[
				IndexedItem {
					index: 0,
					item: Size2 {
						width: 1,
						height: 1
					}
				}
			])
			.is_ok()
	);
	let bin: Option<IndexedBin<Pos2>> = atlas.build();
	assert!(bin.is_some());
	assert_eq!(
		bin.unwrap().data,
		Vec::from([
			(
				0,
				Pos2 {
					x: 0,
					y: 0
				}
			)
		])
	);
}

#[test]
fn add_all_single_many() {
	let packer = UniformPacker::new();
	let mut atlas = SingleAtlas::<_, _, IndexedItem<Size2>, Pos2>::new(new_options(), packer);

	assert!(
		atlas
			.add_all(&[
				IndexedItem {
					index: 0,
					item: Size2 {
						width: 1,
						height: 1
					}
				}
			])
			.is_ok()
	);
	assert!(
		atlas
			.add_all(&[
				IndexedItem {
					index: 1,
					item: Size2 {
						width: 1,
						height: 1
					}
				}
			])
			.is_ok()
	);
	assert!(
		atlas
			.add_all(&[
				IndexedItem {
					index: 2,
					item: Size2 {
						width: 1,
						height: 1
					}
				}
			])
			.is_ok()
	);
	let bin: Option<IndexedBin<Pos2>> = atlas.build();
	assert!(bin.is_some());
	assert_eq!(
		bin.unwrap().data,
		Vec::from([
			(
				0,
				Pos2 {
					x: 0,
					y: 0
				}
			),
			(
				1,
				Pos2 {
					x: 1,
					y: 0
				}
			),
			(
				2,
				Pos2 {
					x: 2,
					y: 0
				}
			)
		])
	);
}

#[test]
fn add_all_multi() {
	let packer = UniformPacker::new();
	let mut atlas = SingleAtlas::<_, _, IndexedItem<Size2>, Pos2>::new(new_options(), packer);

	assert!(
		atlas
			.add_all(&[
				IndexedItem {
					index: 0,
					item: Size2 {
						width: 1,
						height: 1
					}
				},
				IndexedItem {
					index: 1,
					item: Size2 {
						width: 1,
						height: 1
					}
				},
				IndexedItem {
					index: 2,
					item: Size2 {
						width: 1,
						height: 1
					}
				},
			])
			.is_ok()
	);
	let bin: Option<IndexedBin<Pos2>> = atlas.build();
	assert!(bin.is_some());
	assert_eq!(
		bin.unwrap().data,
		Vec::from([
			(
				0,
				Pos2 {
					x: 0,
					y: 0
				}
			),
			(
				1,
				Pos2 {
					x: 1,
					y: 0
				}
			),
			(
				2,
				Pos2 {
					x: 2,
					y: 0
				}
			)
		])
	);
}
