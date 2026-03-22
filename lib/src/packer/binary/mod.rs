mod node;
#[cfg(test)]
mod test;

use std::borrow::Borrow;
use std::convert::Infallible;

use node::Node;

use crate::Fit2;
use crate::Item2;
use crate::Item2Ext;
use crate::Options2;
use crate::Packer;
use crate::PackerOp;
use crate::Pos2;
use crate::Rotate2;
use crate::Size2;
use crate::cmp_by_max;
use crate::cmp_by_width;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BinaryPacker {
	bin_list: Vec<BinaryBin>,
}

impl BinaryPacker {
	pub fn new() -> Self {
		Self {
			bin_list: Vec::new(),
		}
	}

	fn add_bin(&mut self, options: &Options2, item: &Size2) {
		self.bin_list.push(BinaryBin::new(options, item));
	}
}

impl Default for BinaryPacker {
	fn default() -> Self {
		Self::new()
	}
}

impl<Item> Packer<Item, Pos2, Options2> for BinaryPacker
where
	Item: Item2,
{
	type Error = Infallible;

	fn add(&mut self, options: &Options2, item: &Item) -> Result<PackerOp<Pos2>, Self::Error> {
		let size = options.item_size(item);
		for (index, bin) in &mut self.bin_list.iter_mut().enumerate() {
			if let Some(position) = bin.add_to_smallest_node(&size) {
				// TODO: Add test for multiple bins.
				return Ok(PackerOp::ExistingBin((index, options.pos(position.x, position.y))));
			}
		}

		self.add_bin(options, &size);
		Ok(PackerOp::NewBin(options.margin()))
	}

	fn add_all<T: Borrow<Item>>(
		&mut self,
		options: &Options2,
		group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, PackerOp<Pos2>), Self::Error>> {
		let mut index_list: Vec<usize> = (0..group.len()).collect::<Vec<usize>>();
		index_list.sort_by(|a, b| {
			let item_a = group[*a].borrow();
			let item_b = group[*b].borrow();
			cmp_by_width(item_a, item_b)
		});
		index_list.reverse();
		index_list.into_iter().map(move |index| {
			let item = group[index].borrow();
			self.add(options, item).map(|op| (index, op))
		})
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]

struct BinaryBin {
	node_list: Vec<Node>,
}

impl BinaryBin {
	pub fn new(options: &Options2, item: &Size2) -> Self {
		let node = Node::new(options.max_logical_size());

		let node_list = match node.fit(item) {
			Fit2::Total | Fit2::None => vec![],
			Fit2::Within => {
				let (a, b) = node.split(item);
				vec![
					a,
					b,
				]
			}
			Fit2::Vertical => {
				let a = node.split_vertical(item);
				vec![a]
			}
			Fit2::Horizontal => {
				let a = node.split_horizontal(item);
				vec![a]
			}
		};

		Self {
			node_list,
		}
	}

	pub fn add_to_smallest_node(&mut self, item: &Size2) -> Option<Pos2> {
		for (index, node) in self.node_list.iter().enumerate() {
			let fit = node.fit(item);
			if !fit.fits() {
				continue;
			}
			let result = self.replace(index, item, fit);
			self.node_list.sort_by_key(|x| (x.width(), x.height()));
			return Some(result);
		}
		None
	}

	fn replace(&mut self, index: usize, item: &Size2, fit: Fit2) -> Pos2 {
		let node = &self.node_list[index];
		let position = Pos2::new(node.position.x, node.position.y);

		match fit {
			Fit2::Total => {
				self.node_list.remove(index);
				position
			}
			Fit2::Within => {
				let (a, b) = node.split(item);
				self.node_list.remove(index);
				self.node_list.push(a);
				self.node_list.push(b);
				position
			}
			Fit2::Vertical => {
				let a = node.split_vertical(item);
				self.node_list.remove(index);
				self.node_list.push(a);
				position
			}
			Fit2::Horizontal => {
				let a = node.split_horizontal(item);
				self.node_list.remove(index);
				self.node_list.push(a);
				position
			}
			Fit2::None => {
				unreachable!()
			}
		}
	}
}

// // TODO: Consider merging with BinaryPacker if Packer gets more generalized.
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct RotatableBinaryPacker {
// 	packer: BinaryPacker,
// }

impl<Item> Packer<Item, Rotate2, Options2> for BinaryPacker
where
	Item: Item2,
{
	type Error = Infallible;

	fn add(&mut self, options: &Options2, item: &Item) -> Result<PackerOp<Rotate2>, Self::Error> {
		let (item, rotate) = if item.width() > item.height() {
			(Size2::new(item.height(), item.width()), true)
		} else {
			(Size2::new(item.width(), item.height()), false)
		};
		let result = self.add(options, &item)?;
		match result {
			PackerOp::NewBin(pos) => {
				Ok(PackerOp::NewBin(Rotate2 {
					pos,
					rotate,
				}))
			}
			PackerOp::ExistingBin((index, pos)) => {
				Ok(PackerOp::ExistingBin((
					index,
					Rotate2 {
						pos,
						rotate,
					},
				)))
			}
		}
	}

	fn add_all<T: Borrow<Item>>(
		&mut self,
		options: &Options2,
		group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, PackerOp<Rotate2>), Self::Error>> {
		let mut index_list: Vec<usize> = (0..group.len()).collect::<Vec<usize>>();
		index_list.sort_by(|a, b| {
			let item_a = group[*a].borrow();
			let item_b = group[*b].borrow();
			cmp_by_max(item_a, item_b)
		});
		index_list.reverse();
		index_list.into_iter().map(move |index| {
			let item = group[index].borrow();
			self.add(options, item).map(|op| (index, op))
		})
	}
}
