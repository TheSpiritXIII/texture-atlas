mod generate;

use std::fs;
use std::io;
use std::path::PathBuf;

use chacha20::ChaCha20Rng;
use clap::Parser;
use rand::SeedableRng;
use rand::rngs::ThreadRng;

#[derive(Parser)]
struct Cli {
	/// The output directory to place generated images.
	#[arg(long)]
	output_dir: PathBuf,

	/// How many images to generate.
	#[arg(long)]
	amount: u16,

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

	/// The seed to use to generate images.
	#[arg(long)]
	seed: Option<String>,
}

pub fn str_to_seed(s: &str) -> [u8; 32] {
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

pub fn seed_to_str(seed: &[u8; 32]) -> String {
	seed.iter().map(|x| format!("{:02x}", x)).collect::<Vec<String>>().join("")
}

fn main() -> io::Result<()> {
	let cli = Cli::parse();
	let mut rng = if let Some(seed) = cli.seed {
		ChaCha20Rng::from_seed(str_to_seed(&seed))
	} else {
		let rng = ChaCha20Rng::from_rng(&mut ThreadRng::default());
		println!("Generated seed: {}", seed_to_str(&rng.get_seed()));
		rng
	};

	fs::create_dir_all(&cli.output_dir)?;
	for i in 0..cli.amount {
		let output_path = cli.output_dir.join(format!("image_{}.png", i));
		let image = generate::generate(
			&mut rng,
			(cli.min_width, cli.min_height),
			(cli.max_width, cli.max_height),
		);
		image.save(&output_path).unwrap();
	}
	Ok(())
}
