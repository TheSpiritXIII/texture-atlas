use crate::AtlasRect;

pub trait AtlasPage<T: AtlasRect> {
	type Error;

	fn new(width: u32, height: u32) -> Self;
	fn rect_add(&mut self, rect: &T, x: u32, y: u32) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page<T: AtlasRect> {
	pub width: usize,
	pub height: usize,
	pub rects: Vec<(T, usize, usize)>,
}

impl<T: AtlasRect + Clone> AtlasPage<T> for Page<T> {
	type Error = ();

	fn new(width: u32, height: u32) -> Self {
		Self {
			width: width as usize,
			height: height as usize,
			rects: Vec::new(),
		}
	}

	fn rect_add(&mut self, rect: &T, x: u32, y: u32) -> Result<(), Self::Error> {
		self.rects.push((rect.clone(), x as usize, y as usize));
		Ok(())
	}
}
