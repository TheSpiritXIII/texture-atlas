mod node;
#[cfg(test)]
mod test;

use std::borrow::Borrow;
use std::marker::PhantomData;

use node::Node;

use crate::AtlasRect;
use crate::AtlasRectExt;
use crate::Fit2;
use crate::Packer;
use crate::Pos2;
use crate::Size2;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BinaryPacker<Item>
where
	Item: AtlasRect,
{
	bin_list: Vec<BinaryBin>,
	phantom: PhantomData<Item>,
}

impl<Item> BinaryPacker<Item>
where
	Item: AtlasRect,
{
	pub fn new() -> Self {
		Self {
			bin_list: Vec::new(),
			phantom: PhantomData,
		}
	}

	fn add_bin(&mut self, options: &crate::AtlasOptions, item: &Size2) {
		self.bin_list.push(BinaryBin::new(options, item));
	}
}

impl<Item> Default for BinaryPacker<Item>
where
	Item: AtlasRect,
{
	fn default() -> Self {
		Self::new()
	}
}

impl<Item> Packer<Item> for BinaryPacker<Item>
where
	Item: AtlasRect,
{
	type Output = Pos2;
	type Error = ();

	fn add(
		&mut self,
		options: &crate::AtlasOptions,
		item: &Item,
	) -> Result<crate::PackerOp<Self::Output>, Self::Error> {
		let size = Size2::new(item.width(), item.height());
		for bin in &mut self.bin_list {
			if let Some(position) = bin.add_to_smallest_node(&size) {
				return Ok(crate::PackerOp::ExistingBin((0, position)));
			}
		}

		self.add_bin(options, &size);
		Ok(crate::PackerOp::NewBin(Pos2 {
			x: 0,
			y: 0,
		}))
	}

	fn add_all<T: Borrow<Item>>(
		&mut self,
		options: &crate::AtlasOptions,
		group: &[T],
	) -> impl IntoIterator<Item = Result<(usize, crate::PackerOp<Self::Output>), Self::Error>> {
		let mut index_list: Vec<usize> = (0..group.len()).collect::<Vec<usize>>();
		index_list.sort_by_key(|x| {
			let item = group[*x].borrow();
			(item.width(), item.height())
		});
		index_list.reverse();
		index_list.into_iter().map(move |index| {
			let item = group[index].borrow();
			self.add(options, item).map(|x| (index, x))
		})
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]

struct BinaryBin {
	node_list: Vec<Node>,
}

impl BinaryBin {
	pub fn new(options: &crate::AtlasOptions, item: &Size2) -> Self {
		let node = Node::new(Size2 {
			width: options.max_width.get(),
			height: options.max_height.get(),
		});

		let node_list = match node.fit2(item) {
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
			let fit = node.fit2(item);
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
