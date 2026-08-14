use std::fs;
use std::io;
use std::path::PathBuf;

use clap::Parser;
use image_generator::ImageOptions;
use image_generator::rng_with_random_seed;
use image_generator::rng_with_seed;

#[derive(Parser)]
struct Cli {
	/// The output directory to place generated images.
	#[arg(long)]
	output_dir: PathBuf,

	/// How many images to generate.
	#[arg(long)]
	amount: u16,

	#[command(flatten)]
	options: ImageOptions,

	/// The seed to use to generate images.
	#[arg(long)]
	seed: Option<String>,
}

fn main() -> io::Result<()> {
	let cli = Cli::parse();
	let mut rng = if let Some(seed) = &cli.seed {
		rng_with_seed(seed)
	} else {
		let (rng, seed) = rng_with_random_seed();
		println!("Generated seed: {seed}");
		rng
	};

	fs::create_dir_all(&cli.output_dir)?;
	for i in 0..cli.amount {
		let output_path = cli.output_dir.join(format!("image_{}.png", i));
		let image = cli.options.generate(&mut rng);
		image.save(&output_path).unwrap();
	}
	Ok(())
}
