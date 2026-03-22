use image::GenericImage;
use image::ImageBuffer;
use image::ImageError;
use image::Pixel;

use crate::AtlasPage;

// TODO: Document why DynamicImage does not have an implementation.
impl<P: Pixel> AtlasPage<ImageBuffer<P, Vec<P::Subpixel>>> for ImageBuffer<P, Vec<P::Subpixel>> {
	type Error = ImageError;

	fn new(width: u32, height: u32) -> Self {
		ImageBuffer::<P, Vec<P::Subpixel>>::new(width, height)
	}

	fn rect_add(
		&mut self,
		rect: &ImageBuffer<P, Vec<P::Subpixel>>,
		x: u32,
		y: u32,
	) -> Result<(), Self::Error> {
		self.copy_from(rect, x, y)
	}
}
