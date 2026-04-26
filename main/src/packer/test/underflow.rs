use std::fmt::Debug;

use crate::AtlasOptions;
use crate::MAX_HEIGHT;
use crate::MAX_WIDTH;
use crate::Packer as AtlasPacker;
use crate::PackerOp;
use crate::Pos2;
use crate::Size2;
use crate::new_bin;

/// Asserts underflow behavior for various algorithm-agnostic size scenarios. This primarily helps
/// algorithms which tightly and deterministically pack items.
pub fn assert_add_underflow<Packer, Error>(options: &AtlasOptions, packer: Packer)
where
	Packer: AtlasPacker<Size2, Output = Pos2, Error = Error> + Clone,
	Error: Debug + PartialEq,
{
	assert_add_underflow_small(options, packer.clone());
	assert_add_underflow_max_width(options, packer.clone());
	assert_add_underflow_max_height(options, packer);
}

pub fn assert_add_underflow_small<Packer, Error>(options: &AtlasOptions, mut packer: Packer)
where
	Packer: AtlasPacker<Size2, Output = Pos2, Error = Error> + Clone,
	Error: Debug + PartialEq,
{
	assert_eq!(
		packer.add(options, &Size2::new(1, 1)),
		Ok(new_bin()),
		"initial item must have new bin"
	);
	assert_eq!(
		packer.add(options, &Size2::new(1, 1)),
		Ok(PackerOp::ExistingBin((
			0,
			Pos2 {
				x: 1,
				y: 0
			}
		))),
		"second item must be next to first item",
	);
	assert_eq!(
		packer.add(options, &Size2::new(1, 1)),
		Ok(PackerOp::ExistingBin((
			0,
			Pos2 {
				x: 2,
				y: 0
			}
		))),
		"third item must be next to second item",
	);
}

pub fn assert_add_underflow_max_width<T: Debug + PartialEq>(
	options: &AtlasOptions,
	mut packer: impl AtlasPacker<Size2, Output = Pos2, Error = T>,
) {
	assert_eq!(packer.add(options, &Size2::new(MAX_WIDTH - 1, MAX_HEIGHT)), Ok(new_bin()),);
	assert_eq!(
		packer.add(options, &Size2::new(1, 1)),
		Ok(PackerOp::ExistingBin((
			0,
			Pos2 {
				x: MAX_WIDTH - 1,
				y: 0
			}
		))),
	);
}

pub fn assert_add_underflow_max_height<T: Debug + PartialEq>(
	options: &AtlasOptions,
	mut packer: impl AtlasPacker<Size2, Output = Pos2, Error = T>,
) {
	assert_eq!(packer.add(options, &Size2::new(MAX_WIDTH, MAX_HEIGHT - 1)), Ok(new_bin()),);
	assert_eq!(
		packer.add(options, &Size2::new(1, 1)),
		Ok(PackerOp::ExistingBin((
			0,
			Pos2 {
				x: 0,
				y: MAX_HEIGHT - 1,
			}
		))),
	);
}
