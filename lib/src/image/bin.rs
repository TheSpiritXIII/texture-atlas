use image::GenericImage;
use image::ImageBuffer;
use image::ImageError;
use image::Pixel;
use image::imageops::rotate90;

use crate::Bin;
use crate::BinAdd;
use crate::Options2;
use crate::Pos2;
use crate::Rotate2;

// TODO: Could do DynamicImage if we have a special constructor.
impl<P> Bin<ImageBuffer<P, Vec<P::Subpixel>>> for ImageBuffer<P, Vec<P::Subpixel>>
where
	P: Pixel + 'static,
{
	type Options = Options2;
	type Error = ImageError;

	fn new(options: &Self::Options) -> Self {
		ImageBuffer::<P, Vec<P::Subpixel>>::new(options.max_width(), options.max_height())
	}
}

impl<P> BinAdd<ImageBuffer<P, Vec<P::Subpixel>>, Pos2> for ImageBuffer<P, Vec<P::Subpixel>>
where
	P: Pixel + 'static,
{
	fn item_add(
		&mut self,
		rect: &ImageBuffer<P, Vec<P::Subpixel>>,
		params: &Pos2,
	) -> Result<(), Self::Error> {
		self.copy_from(rect, params.x, params.y)
	}
}

impl<P> BinAdd<ImageBuffer<P, Vec<P::Subpixel>>, Rotate2> for ImageBuffer<P, Vec<P::Subpixel>>
where
	P: Pixel + 'static,
{
	fn item_add(
		&mut self,
		rect: &ImageBuffer<P, Vec<P::Subpixel>>,
		params: &Rotate2,
	) -> Result<(), Self::Error> {
		if !params.rotate {
			self.copy_from(rect, params.pos.x, params.pos.y)
		} else {
			let image_rotated = rotate90(rect);
			self.copy_from(&image_rotated, params.pos.x, params.pos.y)
		}
	}
}
