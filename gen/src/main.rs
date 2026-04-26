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

fn main() -> io::Result<()> {
	let cli = Cli::parse();

	fs::create_dir_all(&cli.output_dir)?;
	for i in 0..cli.amount {
		let output_path = cli.output_dir.join(format!("image_{}.png", i));
		let image =
			generate::generate((cli.min_width, cli.min_height), (cli.max_width, cli.max_height));
		image.save(&output_path).unwrap();
	}
	Ok(())
}
