use std::num::NonZero;

use crate::AtlasOptions;
use crate::AtlasPacker;
use crate::AtlasPackerOp;
use crate::PassthroughPacker;
use crate::Pos2;
use crate::util::Rect;

fn new_options() -> AtlasOptions {
	AtlasOptions::with_max_size(NonZero::new(1024).unwrap(), NonZero::new(1024).unwrap())
}

fn new_packer() -> PassthroughPacker<Rect> {
	PassthroughPacker::<Rect>::new()
}

#[test]
fn add() {
	let options = new_options();
	let rect_1 = Rect::new(10, 10);
	let rect_2 = Rect::new(5, 5);
	let rect_3 = Rect::new(20, 20);

	let mut packer = new_packer();
	assert_eq!(
		packer.add(&options, &rect_1),
		Ok(AtlasPackerOp::NewBin(Pos2 {
			x: 0,
			y: 0
		}))
	);
	assert_eq!(
		packer.add(&options, &rect_2),
		Ok(AtlasPackerOp::NewBin(Pos2 {
			x: 0,
			y: 0
		}))
	);
	assert_eq!(
		packer.add(&options, &rect_3),
		Ok(AtlasPackerOp::NewBin(Pos2 {
			x: 0,
			y: 0
		}))
	);
}
