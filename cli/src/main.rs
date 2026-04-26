use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Cli {
	#[arg(long)]
	input_dir: PathBuf,

	#[arg(long)]
	output_dir: PathBuf,
}

fn main() {
	let cli = Cli::parse();
	println!("input: {}, output: {}", cli.input_dir.display(), cli.output_dir.display());
}
