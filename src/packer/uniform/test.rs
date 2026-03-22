use std::num::NonZero;

use crate::AtlasOptions;
use crate::AtlasPacker;
use crate::AtlasPackerOp;
use crate::MAX_HEIGHT;
use crate::MAX_WIDTH;
use crate::Pos2;
use crate::Size2;
use crate::UniformPacker;
use crate::assert_add_overflow;
use crate::assert_add_underflow;
use crate::new_bin;

fn new_options() -> AtlasOptions {
	AtlasOptions::with_max_size(NonZero::new(MAX_WIDTH).unwrap(), NonZero::new(MAX_HEIGHT).unwrap())
}

#[test]
fn add_overflow() {
	let options = new_options();
	let packer = UniformPacker::<Size2>::new();
	assert_add_overflow(&options, packer);
}

#[test]
fn add_underflow() {
	let options = new_options();
	let packer = UniformPacker::<Size2>::new();
	assert_add_underflow(&options, packer.clone());
}

#[test]
fn add_rows() {
	let options = new_options();
	let mut packer = UniformPacker::<Size2>::new();
	assert_fill_bin(&options, &mut packer, 0);
	assert_fill_bin(&options, &mut packer, 1);
}

fn assert_fill_bin(options: &AtlasOptions, packer: &mut UniformPacker<Size2>, bin_index: usize) {
	assert_eq!(
		packer.add(options, &Size2::new(MAX_WIDTH / 2, MAX_HEIGHT / 2)),
		Ok(new_bin()),
		"Bin {} initial item",
		bin_index
	);
	assert_eq!(
		packer.add(options, &Size2::new(MAX_WIDTH / 2, MAX_HEIGHT / 2)),
		Ok(AtlasPackerOp::ExistingBin((
			bin_index,
			Pos2 {
				x: MAX_WIDTH / 2,
				y: 0
			}
		))),
		"Bin {} first row second item",
		bin_index
	);
	assert_eq!(
		packer.add(options, &Size2::new(MAX_WIDTH / 2, MAX_HEIGHT / 2)),
		Ok(AtlasPackerOp::ExistingBin((
			bin_index,
			Pos2 {
				x: 0,
				y: MAX_HEIGHT / 2,
			}
		))),
		"Bin {} second row first item",
		bin_index
	);
	assert_eq!(
		packer.add(options, &Size2::new(MAX_WIDTH / 2, MAX_HEIGHT / 2)),
		Ok(AtlasPackerOp::ExistingBin((
			bin_index,
			Pos2 {
				x: MAX_WIDTH / 2,
				y: MAX_HEIGHT / 2,
			}
		))),
		"Bin {} second row second item",
		bin_index
	);
}
