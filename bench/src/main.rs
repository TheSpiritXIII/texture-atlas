use clap::Parser;

#[derive(Parser)]
#[command(name = "texture-atlas-bench")]
#[command(about = "Benchmarking suite for texture-atlas library")]
struct Cli {}

fn main() -> anyhow::Result<()> {
	env_logger::init();
	let _cli = Cli::parse();
	Ok(())
}
