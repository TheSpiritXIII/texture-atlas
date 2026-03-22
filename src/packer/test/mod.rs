mod overflow;
mod underflow;

pub use overflow::*;
pub use underflow::*;

use crate::AtlasPackerOp;
use crate::Pos2;

pub(crate) fn new_bin() -> AtlasPackerOp<Pos2> {
	AtlasPackerOp::NewBin(Pos2 {
		x: 0,
		y: 0,
	})
}
