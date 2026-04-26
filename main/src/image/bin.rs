use std::num::NonZero;

use image::GenericImage;
use image::ImageBuffer;
use image::ImageError;
use image::Pixel;

use crate::Bin;
use crate::Pos2;

// TODO: Could do DynamicImage if we have a special constructor.
impl<P: Pixel> Bin<ImageBuffer<P, Vec<P::Subpixel>>> for ImageBuffer<P, Vec<P::Subpixel>> {
	// TODO: Custom params, e.g. rotate?
	type Params = Pos2;
	type Error = ImageError;

	fn new(width: NonZero<u32>, height: NonZero<u32>) -> Self {
		ImageBuffer::<P, Vec<P::Subpixel>>::new(width.get(), height.get())
	}

	fn item_add(
		&mut self,
		rect: &ImageBuffer<P, Vec<P::Subpixel>>,
		params: &Self::Params,
	) -> Result<(), Self::Error> {
		self.copy_from(rect, params.x, params.y)
	}
}
