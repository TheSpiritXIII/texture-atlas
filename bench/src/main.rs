mod compare;

use std::io;

use clap::Parser;
use clap::Subcommand;

use crate::compare::CompareArgs;

#[derive(Parser)]
#[command(name = "texture-atlas-bench")]
#[command(about = "Benchmarking suite for texture-atlas library")]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// Compare all packing algorithms against a batch of generated images.
	Compare(CompareArgs),
}

fn main() -> anyhow::Result<()> {
	env_logger::init();
	let cli = Cli::parse();
	let mut stdout = io::stdout();
	match cli.command {
		Commands::Compare(args) => {
			args.run(&mut stdout)?;
		}
	}
	Ok(())
}
