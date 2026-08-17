use std::num::NonZero;

use crate::AlwaysErrorPacker;
use crate::AlwaysExistingBinPacker;
use crate::Options2;
use crate::Pos2;
use crate::SingleBuilder;
use crate::SingleBuilderError;
use crate::Size2;
use crate::UniformPacker;
use crate::test::IndexedBin;
use crate::test::IndexedItem;

fn new_options() -> Options2 {
	Options2::with_max_size(NonZero::new(1024).unwrap(), NonZero::new(1024).unwrap())
}

#[test]
fn empty() {
	let packer = UniformPacker::new();
	let atlas = SingleBuilder::<_, _, IndexedItem<Size2>, Pos2>::new(new_options(), packer);

	let bin: Option<IndexedBin<Pos2>> = atlas.build();
	assert!(bin.is_none());
}

#[test]
fn missing_bin() {
	let mut atlas =
		SingleBuilder::<_, IndexedBin<Pos2>, _, Pos2>::new(new_options(), AlwaysExistingBinPacker);

	// TODO: Switch to assert_matches.
	assert!(matches!(
		atlas.add(&IndexedItem {
			index: 0,
			item: Size2 {
				width: 1,
				height: 1
			},
		}),
		Err(SingleBuilderError::MissingBin)
	));
}

#[test]
fn packer_error() {
	let mut atlas =
		SingleBuilder::<_, IndexedBin<Pos2>, _, Pos2>::new(new_options(), AlwaysErrorPacker);

	// TODO: Switch to assert_matches.
	assert!(matches!(
		atlas.add(&IndexedItem {
			index: 0,
			item: Size2 {
				width: 1,
				height: 1
			},
		}),
		Err(SingleBuilderError::Packer(()))
	));
}

#[test]
fn add_single_once() {
	let packer = UniformPacker::new();
	let mut atlas = SingleBuilder::<_, _, IndexedItem<Size2>, Pos2>::new(new_options(), packer);

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
	let mut atlas = SingleBuilder::<_, _, IndexedItem<Size2>, Pos2>::new(new_options(), packer);

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
	let mut atlas = SingleBuilder::<_, _, IndexedItem<Size2>, Pos2>::new(new_options(), packer);

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
	let mut atlas = SingleBuilder::<_, _, IndexedItem<Size2>, Pos2>::new(new_options(), packer);

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
	let mut atlas = SingleBuilder::<_, _, IndexedItem<Size2>, Pos2>::new(new_options(), packer);

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
