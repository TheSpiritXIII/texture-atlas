use std::num::NonZero;

use crate::AtlasOptions;
use crate::Packer;
use crate::PackerOp;
use crate::PassthroughPacker;
use crate::Pos2;
use crate::Size2;
use crate::assert_add_overflow;

fn new_options() -> AtlasOptions {
	AtlasOptions::with_max_size(NonZero::new(1024).unwrap(), NonZero::new(1024).unwrap())
}

fn new_packer() -> PassthroughPacker<Size2> {
	PassthroughPacker::<Size2>::new()
}

#[test]
fn add_overflow() {
	let options = new_options();
	let packer = new_packer();
	assert_add_overflow(&options, packer);
}

#[test]
fn add_underflow() {
	let options = new_options();
	let rect_1 = Size2::new(10, 10);
	let rect_2 = Size2::new(5, 5);
	let rect_3 = Size2::new(20, 20);

	let mut packer = new_packer();
	assert_eq!(
		packer.add(&options, &rect_1),
		Ok(PackerOp::NewBin(Pos2 {
			x: 0,
			y: 0
		}))
	);
	assert_eq!(
		packer.add(&options, &rect_2),
		Ok(PackerOp::NewBin(Pos2 {
			x: 0,
			y: 0
		}))
	);
	assert_eq!(
		packer.add(&options, &rect_3),
		Ok(PackerOp::NewBin(Pos2 {
			x: 0,
			y: 0
		}))
	);
}
