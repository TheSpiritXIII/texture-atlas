use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Cli {
	#[arg(long)]
	output_dir: PathBuf,
}

fn main() {
	let cli = Cli::parse();
	println!("output: {}", cli.output_dir.display());
}
