use std::ops::Deref;

use image::DynamicImage;
use image::ImageBuffer;
use image::Pixel;

use crate::Item2;

impl Item2 for DynamicImage {
	fn width(&self) -> u32 {
		DynamicImage::width(self)
	}
	fn height(&self) -> u32 {
		DynamicImage::height(self)
	}
}

impl<P, Container> Item2 for ImageBuffer<P, Container>
where
	P: Pixel,
	Container: Deref<Target = [P::Subpixel]>,
{
	fn width(&self) -> u32 {
		ImageBuffer::width(self)
	}
	fn height(&self) -> u32 {
		ImageBuffer::height(self)
	}
}
