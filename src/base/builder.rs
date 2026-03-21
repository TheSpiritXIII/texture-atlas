use std::borrow::Borrow;
use std::cmp::max;

#[cfg(feature = "image")]
use image::DynamicImage;

use crate::AtlasRect;
use crate::AtlasRectExt;
use crate::util;
use crate::util::Rect;

/// References an axis aligned rect placed in a bin by index.
pub struct AtlasPart {
	/// The index of the original rect list that this class references.
	pub rect_index: usize,

	/// The x-position where this rect is located in the bin.
	pub x: u32,

	/// The y-position where this rect is located in the bin.
	pub y: u32,

	/// Whether the rect is rotated 90 degrees clockwise.
	pub rotate: bool,
}

/// A packed bin containing references to rects.
///
/// This class tracks the rects added to itself. After each rect is added, it increases its
/// bounding size if necessary. However, it does not make any guarantees. It is expected that all
/// atlas generators play nicely and conform to all rules. The bin size should be the minimum
/// bounding size, capable of encapsulating all objects. Each object should also not pass through
/// any boundaries and should be disjoint.
pub struct AtlasBin {
	/// The bounding dimensions of the bin.
	pub dimensions: Rect,

	/// The list of referenced rects in this bin.
	pub part_list: Vec<AtlasPart>,
}

impl AtlasBin {
	/// Initializes a new bin with the given rect at the top right of the bin.
	fn new(rect_index: usize, dimensions: Rect, rotate: bool) -> Self {
		let part = AtlasPart {
			rect_index,
			x: 0,
			y: 0,
			rotate,
		};
		AtlasBin {
			dimensions,
			part_list: vec![part],
		}
	}

	/// Returns the current bounding dimensions of the bin.
	pub fn dimensions(&self) -> Rect {
		self.dimensions
	}

	/// Returns the parts in this bin.
	pub fn part_list(&self) -> &[AtlasPart] {
		&self.part_list
	}

	/// Adds a new rect to the bin. The size of the bin increases if mandatory.
	fn part_add(&mut self, rect_index: usize, x: u32, y: u32, dimensions: Rect, rotate: bool) {
		self.dimensions.width = max(self.dimensions.width, x + dimensions.width);
		self.dimensions.height = max(self.dimensions.height, y + dimensions.height);
		self.part_list.push(AtlasPart {
			rect_index,
			x,
			y,
			rotate,
		});
	}
}

impl AsRef<Rect> for AtlasBin {
	fn as_ref(&self) -> &Rect {
		&self.dimensions
	}
}

/// Generates a texture atlas using a bin packing algorithm.
pub trait AtlasGenerator {
	/// Generates a list of bins for the given atlas.
	fn generate<T: AtlasRect>(&self, atlas: &mut Atlas<T>, width: u32, height: u32, rotate: bool);
}

/// List data structure for adding rects.
///
/// This data structure is essentially a wrapper on `Vec<T>` with the difference that it tracks
/// total area of all rects in order to calculate a lower bound of the number of bins. By using a
/// lower bound, an atlas may potentially re-allocate less.
pub struct AtlasRectList<T: AtlasRect> {
	rect_list: Vec<T>,
	total_area: u64,
}

impl<T> AtlasRectList<T>
where
	T: AtlasRect,
{
	/// Constructs a new, empty list.
	pub fn new() -> Self {
		AtlasRectList {
			rect_list: Vec::new(),
			total_area: 0,
		}
	}

	/// Constructs a new, empty list with the specified capacity.
	pub fn with_capacity(capacity: usize) -> Self {
		AtlasRectList {
			rect_list: Vec::with_capacity(capacity),
			total_area: 0,
		}
	}

	/// Returns the number of rects in the list.
	pub fn len(&self) -> usize {
		self.rect_list.len()
	}

	/// Adds the given rect to the list and potentially increases the lower bound.
	pub fn add(&mut self, rect: T) {
		self.total_area += rect.area();
		self.rect_list.push(rect);
	}

	/// Returns the total area of all rects in this list combined.
	pub fn total_area(&self) -> u64 {
		self.total_area
	}

	/// Returns the lower bound of bins needed for the rects in this list.
	pub fn lower_bound(&self, size: Rect) -> usize {
		assert_eq!(size.empty(), false);
		((self.total_area / size.area()) + 1) as usize
	}

	/// Returns an atlas builder using this rect list and given constraints.
	pub fn build(&self, width: u32, height: u32, rotate: bool) -> AtlasBuilder<T> {
		let lower_bound = self.lower_bound(Rect::new(width, height));
		AtlasBuilder::new(&self.rect_list, width, height, rotate, lower_bound)
	}
}

/// Stores settings for generating an `Atlas`.
///
/// The builder takes a few constraints. It takes a maximal width and height constraint, which atlas
/// generators are not expected to exceed. It also takes a flag indicating whether or not rotations
/// should be allowed by generators.
pub struct AtlasBuilder<'a, T>
where
	T: 'a + AtlasRect,
{
	rect_list: &'a [T],
	width: u32,
	height: u32,
	lower_bound: usize,
	rotate: bool,
}

impl<'a, T> AtlasBuilder<'a, T>
where
	T: 'a + AtlasRect,
{
	fn new(rect_list: &'a [T], width: u32, height: u32, rotate: bool, lower_bound: usize) -> Self {
		AtlasBuilder {
			rect_list,
			width,
			height,
			lower_bound,
			rotate,
		}
	}

	/// Generates bins using the given generator.
	pub fn generate<G: AtlasGenerator>(self, generator: &G) -> Atlas<'a, T> {
		let mut atlas = Atlas {
			rect_list: self.rect_list,
			bin_list: Vec::with_capacity(self.lower_bound),
		};
		generator.generate(&mut atlas, self.width, self.height, self.rotate);
		atlas
	}
}

/// Encapsulates axis aligned rectangles and generated bins.
pub struct Atlas<'a, T: 'a + AtlasRect> {
	rect_list: &'a [T],
	bin_list: Vec<AtlasBin>,
}

impl<'a, T> Atlas<'a, T>
where
	T: 'a + AtlasRect,
{
	/// Returns a builder instance with the given size constraints.
	pub fn build(rect_list: &'a [T], width: u32, height: u32, rotate: bool) -> AtlasBuilder<T> {
		AtlasBuilder::new(rect_list, width, height, rotate, 1)
	}

	/// Creates a new atlas with the given axis-aligned rectangles.
	pub fn new(rect_list: &'a [T]) -> Self {
		Self {
			rect_list,
			bin_list: Vec::new(),
		}
	}

	/// Returns the list of axis-aligned rectangles that are part of the atlas.
	pub fn rect_list(&self) -> &[T] {
		&self.rect_list
	}

	/// Returns the bins that reference the rects.
	pub fn bin_list(&self) -> &[AtlasBin] {
		&self.bin_list
	}

	/// Creates a new bin with the given rect at the top left.
	pub fn bin_add_new(&mut self, rect_index: usize, rotate: bool) -> usize {
		let bin_index = self.bin_list.len();
		let dimensions = if rotate {
			self.rect_list[rect_index].dimensions_rotated()
		} else {
			self.rect_list[rect_index].dimensions()
		};
		self.bin_list.push(AtlasBin::new(rect_index, dimensions, rotate));
		bin_index
	}

	/// Adds a new rect to the indicated bin.
	pub fn bin_add_rect(
		&mut self,
		bin_index: usize,
		rect_index: usize,
		x: u32,
		y: u32,
		rotate: bool,
	) {
		let dimensions = if rotate {
			self.rect_list[rect_index].dimensions_rotated()
		} else {
			self.rect_list[rect_index].dimensions()
		};
		self.bin_list[bin_index].part_add(rect_index, x, y, dimensions, rotate);
	}

	#[cfg(feature = "image")]
	/// Generates an image from the indicated bin with uniformly separated colors.
	pub fn bin_as_colors(&self, bin_index: usize) -> DynamicImage {
		use crate::util;

		let weight = util::colors_weight(self.rect_list.len());
		util::colors_from_bin(weight, self.rect_list, &self.bin_list[bin_index])
	}

	#[cfg(feature = "image")]
	/// Generates images from the generated bins with uniformly separated colors.
	pub fn as_colors(&self) -> Vec<DynamicImage> {
		let weight = util::colors_weight(self.rect_list.len());
		let mut image_list = Vec::with_capacity(self.rect_list.len());

		for bin in &self.bin_list {
			image_list.push(util::colors_from_bin(weight, self.rect_list, bin));
		}
		image_list
	}
}

#[cfg(feature = "image")]
impl<'a, T> Atlas<'a, T>
where
	T: AtlasRect + Borrow<DynamicImage>,
{
	/// Returns the given bin as an image.
	pub fn bin_as_image(&self, bin_index: usize) -> DynamicImage {
		util::image_from_bin(self.rect_list, &self.bin_list[bin_index])
	}

	/// Generates images from the generated bin using the given image objects.
	pub fn as_images(&self) -> Vec<DynamicImage> {
		let mut image_list = Vec::with_capacity(self.rect_list.len());

		for bin in &self.bin_list {
			image_list.push(util::image_from_bin(self.rect_list, bin));
		}
		image_list
	}
}
