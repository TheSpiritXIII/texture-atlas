#[cfg(test)]
mod test;

use chacha20::ChaCha20Rng;
use image::Rgb;
use image::RgbImage;
use rand::Rng;
use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::ThreadRng;

/// Options for configuring image dimensions during generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::Args)]
pub struct ImageOptions {
	/// The minimum width for generated images. The image width will be random between this and
	/// `max_width`.
	#[arg(long)]
	min_width: u32,

	/// The minimum height for generated images. The image height will be random between this and
	/// `max_height`.
	#[arg(long)]
	min_height: u32,

	/// The maximum width for generated images. The image width will be random between `min_width`
	/// and this.
	#[arg(long)]
	max_width: u32,

	/// The maximum height for generated images. The image height will be random between
	/// `min_height` and this.
	#[arg(long)]
	max_height: u32,
}

impl ImageOptions {
	/// Generates a single random-colored image within the configured size range.
	pub fn generate(&self, rng: &mut impl Rng) -> RgbImage {
		let width = rng.random_range(self.min_width..=self.max_width);
		let height = rng.random_range(self.min_height..=self.max_height);

		let rgb = Rgb([
			rng.random(),
			rng.random(),
			rng.random(),
		]);
		let mut image = RgbImage::new(width, height);
		for (_, _, pixel) in image.enumerate_pixels_mut() {
			*pixel = rgb;
		}
		image
	}
}

fn str_to_seed(s: &str) -> [u8; 32] {
	s.as_bytes()
		.chunks(2)
		.map(|chunk| {
			let s = std::str::from_utf8(chunk).unwrap();
			u8::from_str_radix(s, 16)
		})
		.collect::<Result<Vec<_>, _>>()
		.unwrap()
		.try_into()
		.unwrap()
}

fn seed_to_str(seed: &[u8; 32]) -> String {
	seed.iter().map(|x| format!("{:02x}", x)).collect::<Vec<String>>().join("")
}

/// Creates a `ChaCha20Rng` initialized with the given hex seed string.
pub fn rng_with_seed(seed: &str) -> ChaCha20Rng {
	ChaCha20Rng::from_seed(str_to_seed(seed))
}

/// Generates a new random seed and returns an initialized `ChaCha20Rng` alongside its hex seed
/// string.
pub fn rng_with_random_seed() -> (ChaCha20Rng, String) {
	let rng = ChaCha20Rng::from_rng(&mut ThreadRng::default());
	let seed_str = seed_to_str(&rng.get_seed());
	(rng, seed_str)
}
