mod generate;

use std::fs;
use std::io;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Cli {
	/// The output directory to place generated images.
	#[arg(long)]
	output_dir: PathBuf,

	/// How many images to generate.
	#[arg(long)]
	amount: u16,
}

fn main() -> io::Result<()> {
	let cli = Cli::parse();
	println!("output: {}", cli.output_dir.display());

	fs::create_dir_all(&cli.output_dir)?;
	for i in 0..cli.amount {
		let output_path = cli.output_dir.join(format!("image_{}.png", i));
		let image = generate::generate();
		image.save(&output_path).unwrap();
	}
	Ok(())
}
