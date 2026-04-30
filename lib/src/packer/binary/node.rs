use crate::Item2;
use crate::Pos2;
use crate::Size2;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Node {
	pub size: Size2,
	pub position: Pos2,
}

impl Node {
	pub fn new(size: Size2) -> Self {
		Self {
			size,
			position: Pos2 {
				x: 0,
				y: 0,
			},
		}
	}

	pub fn split(&self, add: &Size2) -> (Self, Self) {
		(
			self.split_horizontal(add),
			Self {
				position: Pos2 {
					x: self.position.x,
					y: self.position.y + add.height,
				},
				size: Size2::new(self.size.width, self.size.height - add.height),
			},
		)
	}

	pub fn split_horizontal(&self, add: &Size2) -> Self {
		Self {
			position: Pos2 {
				x: self.position.x + add.width,
				y: self.position.y,
			},
			size: Size2::new(self.size.width - add.width, add.height),
		}
	}

	pub fn split_vertical(&self, add: &Size2) -> Self {
		Self {
			position: Pos2 {
				x: self.position.x,
				y: self.position.y + add.height,
			},
			size: Size2::new(add.width, self.size.height - add.height),
		}
	}
}

impl Item2 for Node {
	fn width(&self) -> u32 {
		self.size.width
	}

	fn height(&self) -> u32 {
		self.size.height
	}
}

#[test]
fn split() {
	let node = Node {
		position: Pos2 {
			x: 0,
			y: 0,
		},
		size: Size2::new(10, 10),
	};
	let (a, b) = node.split(&Size2::new(2, 3));
	assert_eq!(
		a,
		Node {
			position: Pos2 {
				x: 2,
				y: 0
			},
			size: Size2::new(8, 3),
		}
	);
	assert_eq!(
		b,
		Node {
			position: Pos2 {
				x: 0,
				y: 3
			},
			size: Size2::new(10, 7),
		}
	);
}
