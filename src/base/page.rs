use crate::AtlasRect;

/// Represents a single page in a texture atlas.
pub trait AtlasPage<T: AtlasRect>: AtlasRect {
	/// The error type when adding a rect to the page.
	type Error;

	/// Creates a new page with the given maximum size.
	fn new(width: u32, height: u32) -> Self;

	/// Adds a new rect to the page at the given position. The given rect should not overlap with
	/// any other rects previously pass into this function.
	fn rect_add(&mut self, rect: &T, x: u32, y: u32) -> Result<(), Self::Error>;
}

/// References an axis aligned rect placed in a page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageRect<T> {
	/// The original rect.
	rect: T,

	/// The x-position where this rect is located in the bin.
	x: u32,

	/// The y-position where this rect is located in the bin.
	y: u32,
}

/// A utility type that implements `AtlasPage`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page<T: AtlasRect> {
	width: u32,
	height: u32,
	rect_list: Vec<PageRect<T>>,
}

impl<T: AtlasRect> Page<T> {
	/// Returns the list of rects stored in this page.
	pub fn rect_list(&self) -> &[PageRect<T>] {
		&self.rect_list
	}
}

impl<T: AtlasRect + Clone> AtlasPage<T> for Page<T> {
	type Error = ();

	fn new(width: u32, height: u32) -> Self {
		Self {
			width,
			height,
			rect_list: Vec::new(),
		}
	}

	fn rect_add(&mut self, rect: &T, x: u32, y: u32) -> Result<(), Self::Error> {
		self.rect_list.push(PageRect {
			rect: rect.clone(),
			x,
			y,
		});
		Ok(())
	}
}

impl<T: AtlasRect> AtlasRect for Page<T> {
	fn width(&self) -> u32 {
		self.width
	}

	fn height(&self) -> u32 {
		self.height
	}
}
