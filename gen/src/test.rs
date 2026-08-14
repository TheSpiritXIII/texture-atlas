use crate::ImageOptions;
use crate::rng_with_random_seed;
use crate::rng_with_seed;
use crate::seed_to_str;
use crate::str_to_seed;

#[test]
fn image_options_generate_dimensions() {
	let options = ImageOptions {
		min_width: 10,
		min_height: 20,
		max_width: 30,
		max_height: 40,
	};
	let (mut rng, _) = rng_with_random_seed();
	for _ in 0..50 {
		let img = options.generate(&mut rng);
		assert!(img.width() >= 10 && img.width() <= 30);
		assert!(img.height() >= 20 && img.height() <= 40);
	}
}

#[test]
fn seed_deterministic() {
	let seed_str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
	let options = ImageOptions {
		min_width: 16,
		min_height: 16,
		max_width: 64,
		max_height: 64,
	};

	let mut rng1 = rng_with_seed(seed_str);
	let img1 = options.generate(&mut rng1);

	let mut rng2 = rng_with_seed(seed_str);
	let img2 = options.generate(&mut rng2);

	assert_eq!(img1.dimensions(), img2.dimensions());
	assert_eq!(img1.as_raw(), img2.as_raw());
}

#[test]
fn seed_random() {
	let (_, seed_str) = rng_with_random_seed();
	assert_eq!(seed_str.len(), 64);
	let seed_bytes = str_to_seed(&seed_str);
	assert_eq!(seed_to_str(&seed_bytes), seed_str);
}

#[test]
fn generate_args_iterator() {
	let args = crate::GenerateArgs {
		amount: 15,
		options: ImageOptions {
			min_width: 8,
			min_height: 8,
			max_width: 32,
			max_height: 32,
		},
		seed: None,
	};
	let (mut rng, _) = args.rng();
	let images: Vec<_> = args.generate(&mut rng).collect();
	assert_eq!(images.len(), 15);
	for img in images {
		assert!(img.width() >= 8 && img.width() <= 32);
		assert!(img.height() >= 8 && img.height() <= 32);
	}
}
