use std::num::NonZero;

use crate::AtlasOptions;
use crate::BinaryPacker;
use crate::MAX_HEIGHT;
use crate::MAX_WIDTH;
use crate::Packer;
use crate::PackerOp;
use crate::Pos2;
use crate::Size2;
use crate::assert_add_overflow;
use crate::assert_add_underflow;
use crate::new_bin;

fn new_options() -> AtlasOptions {
	AtlasOptions::with_max_size(NonZero::new(MAX_WIDTH).unwrap(), NonZero::new(MAX_HEIGHT).unwrap())
}

#[test]
fn add_overflow() {
	let options = new_options();
	let packer = BinaryPacker::<Size2>::new();
	assert_add_overflow(&options, packer);
}

#[test]
fn add_underflow() {
	let options = new_options();
	let packer = BinaryPacker::<Size2>::new();
	assert_add_underflow(&options, packer);
}

#[test]
fn add_single_underflow_then_underflow() {
	let options = new_options();
	let rect_1 = Size2::new(600, 600);
	let rect_2 = Size2::new(100, 100);
	let rect_3 = Size2::new(400, 400);

	let mut packer = BinaryPacker::<Size2>::new();
	assert_eq!(packer.add(&options, &rect_1), Ok(new_bin()));
	assert_eq!(
		packer.add(&options, &rect_2),
		Ok(PackerOp::ExistingBin((
			0,
			Pos2 {
				x: 600,
				y: 0
			}
		)))
	);
	assert_eq!(
		packer.add(&options, &rect_3),
		Ok(PackerOp::ExistingBin((
			0,
			Pos2 {
				x: 600,
				y: 100,
			}
		)))
	);
}

#[test]
fn add_single_underflow_then_overflow_bin() {
	let options = new_options();
	let rect_1 = Size2::new(600, 600);
	let rect_2 = Size2::new(100, 100);
	let rect_3 = Size2::new(500, 500);

	let mut packer = BinaryPacker::<Size2>::new();
	assert_eq!(packer.add(&options, &rect_1), Ok(new_bin()));
	assert_eq!(
		packer.add(&options, &rect_2),
		Ok(PackerOp::ExistingBin((
			0,
			Pos2 {
				x: 600,
				y: 0
			}
		)))
	);
	assert_eq!(packer.add(&options, &rect_3), Ok(new_bin()));
}

#[test]
fn add_single_underflow_then_overflow_space() {
	let options = new_options();
	let rect_1 = Size2::new(600, 600);
	let rect_2 = Size2::new(100, 100);
	let rect_3 = Size2::new(50, 50);

	let mut packer = BinaryPacker::<Size2>::new();
	assert_eq!(packer.add(&options, &rect_1), Ok(new_bin()));
	assert_eq!(
		packer.add(&options, &rect_2),
		Ok(PackerOp::ExistingBin((
			0,
			Pos2 {
				x: 600,
				y: 0
			}
		)))
	);
	assert_eq!(
		packer.add(&options, &rect_3),
		Ok(PackerOp::ExistingBin((
			0,
			Pos2 {
				x: 700,
				y: 0
			}
		)))
	);
}

#[test]
fn add_multiple_max_size() {
	let options = new_options();
	let rect_1 = Size2::new(1024, 1024);
	let rect_2 = Size2::new(100, 100);

	let mut packer = BinaryPacker::<Size2>::new();
	let result: Vec<_> = packer
		.add_all(
			&options,
			&[
				&rect_1,
				&rect_2,
			],
		)
		.into_iter()
		.collect();
	assert_eq!(
		result,
		&[
			Ok((0, new_bin())),
			Ok((1, new_bin()))
		],
	);
}

#[test]
fn add_multiple_max_width() {
	let options = new_options();
	let rect_1 = Size2::new(1024, 600);
	let rect_2 = Size2::new(100, 100);

	let mut packer = BinaryPacker::<Size2>::new();
	let result: Vec<_> = packer
		.add_all(
			&options,
			&[
				&rect_1,
				&rect_2,
			],
		)
		.into_iter()
		.collect();
	assert_eq!(
		result,
		&[
			Ok((0, new_bin())),
			Ok((
				1,
				PackerOp::ExistingBin((
					0,
					Pos2 {
						x: 0,
						y: 600,
					}
				))
			))
		],
	);
}

#[test]
fn add_multiple_overflow_bin() {
	let options = new_options();
	let rect_1 = Size2::new(600, 600);
	let rect_2 = Size2::new(599, 599);

	let mut packer = BinaryPacker::<Size2>::new();
	let result: Vec<_> = packer
		.add_all(
			&options,
			&[
				&rect_1,
				&rect_2,
			],
		)
		.into_iter()
		.collect();
	assert_eq!(
		result,
		&[
			Ok((0, new_bin())),
			Ok((1, new_bin()))
		],
	);
}

#[test]
fn add_multiple_overflow_space() {
	let options = new_options();
	let rect_1 = Size2::new(600, 600);
	let rect_2 = Size2::new(600, 100);

	let mut packer = BinaryPacker::<Size2>::new();
	let result: Vec<_> = packer
		.add_all(
			&options,
			&[
				&rect_1,
				&rect_2,
			],
		)
		.into_iter()
		.collect();
	assert_eq!(
		result,
		&[
			Ok((0, new_bin())),
			Ok((
				1,
				PackerOp::ExistingBin((
					0,
					Pos2 {
						x: 0,
						y: 600
					}
				))
			))
		],
	);
}

#[test]
fn add_multiple_underflow_then_underflow() {
	let options = new_options();
	let rect_1 = Size2::new(600, 600);
	let rect_2 = Size2::new(100, 100);
	let rect_3 = Size2::new(300, 300);

	let mut packer = BinaryPacker::<Size2>::new();
	let result: Vec<_> = packer
		.add_all(
			&options,
			&[
				&rect_1,
				&rect_2,
				&rect_3,
			],
		)
		.into_iter()
		.collect();
	assert_eq!(
		result,
		&[
			Ok((0, new_bin())),
			Ok((
				2,
				PackerOp::ExistingBin((
					0,
					Pos2 {
						x: 600,
						y: 0
					}
				))
			)),
			Ok((
				1,
				PackerOp::ExistingBin((
					0,
					Pos2 {
						x: 900,
						y: 000
					}
				))
			))
		],
	);
}

#[test]
fn add_multiple_underflow_then_overflow_bin() {
	let options = new_options();
	let rect_1 = Size2::new(600, 600);
	let rect_2 = Size2::new(100, 100);
	let rect_3 = Size2::new(500, 500);

	let mut packer = BinaryPacker::<Size2>::new();
	let result: Vec<_> = packer
		.add_all(
			&options,
			&[
				&rect_1,
				&rect_2,
				&rect_3,
			],
		)
		.into_iter()
		.collect();
	assert_eq!(
		result,
		&[
			Ok((0, new_bin())),
			Ok((2, new_bin())),
			Ok((
				1,
				PackerOp::ExistingBin((
					0,
					Pos2 {
						x: 600,
						y: 0
					}
				))
			)),
		],
	);
}

#[test]
fn add_multiple_underflow_then_overflow_space() {
	let options = new_options();
	let rect_1 = Size2::new(600, 600);
	let rect_2 = Size2::new(100, 100);
	let rect_3 = Size2::new(50, 50);

	let mut packer = BinaryPacker::<Size2>::new();
	let result: Vec<_> = packer
		.add_all(
			&options,
			&[
				&rect_1,
				&rect_2,
				&rect_3,
			],
		)
		.into_iter()
		.collect();
	assert_eq!(
		result,
		&[
			Ok((0, new_bin())),
			Ok((
				1,
				PackerOp::ExistingBin((
					0,
					Pos2 {
						x: 600,
						y: 0
					}
				))
			)),
			Ok((
				2,
				PackerOp::ExistingBin((
					0,
					Pos2 {
						x: 700,
						y: 0
					}
				))
			))
		],
	);
}
