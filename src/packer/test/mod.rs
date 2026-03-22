mod overflow;
mod packer;
mod underflow;

pub use overflow::*;
// TODO: Might be worth putting this elsewhere.
pub(crate) use packer::*;
pub use underflow::*;

use crate::PackerOp;
use crate::Pos2;

pub(crate) fn new_bin() -> PackerOp<Pos2> {
	PackerOp::NewBin(Pos2 {
		x: 0,
		y: 0,
	})
}
