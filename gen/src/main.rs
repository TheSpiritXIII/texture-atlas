use std::fs;
use std::io;
use std::path::PathBuf;

use clap::Parser;
use image_generator::GenerateArgs;
use log::info;

#[derive(Parser)]
struct Cli {
	/// The output directory to place generated images.
	#[arg(long)]
	output_dir: PathBuf,

	#[command(flatten)]
	generate: GenerateArgs,
}

fn main() -> io::Result<()> {
	env_logger::init();
	let cli = Cli::parse();
	let (mut rng, seed) = cli.generate.rng();
	if cli.generate.seed.is_none() {
		info!("Generated seed: {seed}");
	}

	fs::create_dir_all(&cli.output_dir)?;
	for (i, image) in cli.generate.generate(&mut rng).enumerate() {
		let output_path = cli.output_dir.join(format!("image_{}.png", i));
		image.save(&output_path).unwrap();
	}
	Ok(())
}
