use std::num::NonZero;

use image::GenericImage;
use image::ImageBuffer;
use image::ImageError;
use image::Pixel;
use image::imageops::rotate90;

use crate::Bin;
use crate::Rotate2;

// TODO: Could do DynamicImage if we have a special constructor.
impl<P> Bin<ImageBuffer<P, Vec<P::Subpixel>>> for ImageBuffer<P, Vec<P::Subpixel>>
where
	P: Pixel + 'static,
{
	type Params = Rotate2;
	type Error = ImageError;

	fn new(width: NonZero<u32>, height: NonZero<u32>) -> Self {
		ImageBuffer::<P, Vec<P::Subpixel>>::new(width.get(), height.get())
	}

	fn item_add(
		&mut self,
		rect: &ImageBuffer<P, Vec<P::Subpixel>>,
		params: &Self::Params,
	) -> Result<(), Self::Error> {
		if !params.rotate {
			self.copy_from(rect, params.pos.x, params.pos.y)
		} else {
			let image_rotated = rotate90(rect);
			self.copy_from(&image_rotated, params.pos.x, params.pos.y)
		}
	}
}
