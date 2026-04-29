use std::fs;
use std::num::NonZero;
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use clap::Subcommand;
use image::ImageReader;
use image::RgbaImage;
use serde::Deserialize;
use serde::Serialize;
use texture_atlas::AtlasAddMulti;
use texture_atlas::AtlasOptions;
use texture_atlas::BinaryPacker;
use texture_atlas::DynamicAtlas;
use texture_atlas::GenericPacker;
use texture_atlas::PassthroughPacker;
use texture_atlas::Pos2;
use texture_atlas::Scored;
use texture_atlas::ScoredBin2;
use texture_atlas::UniformPacker;

#[derive(Parser)]
struct Cli {
	#[arg(long)]
	input_dir: PathBuf,

	#[arg(long)]
	output_dir: PathBuf,

	#[arg(long)]
	max_width: NonZero<u32>,

	#[arg(long)]
	max_height: NonZero<u32>,

	#[command(subcommand)]
	algorithm: Algorithm,
}

#[derive(Subcommand)]
enum Algorithm {
	Binary,
	Passthrough,
	Uniform,
}

#[derive(Serialize, Deserialize)]
struct Config {
	items: Vec<AtlasAddMulti<Pos2>>,
}

fn main() -> anyhow::Result<()> {
	let cli = Cli::parse();

	let mut image_list = Vec::new();
	for entry in cli.input_dir.read_dir()? {
		let entry = entry?;
		let path = entry.path();
		if path.is_file() {
			let image = ImageReader::open(path)?.decode()?;
			image_list.push(image.to_rgba8());
		}
	}

	let options = AtlasOptions::with_max_size(cli.max_width, cli.max_height);
	let packer: GenericPacker<RgbaImage> = match cli.algorithm {
		Algorithm::Binary => GenericPacker::Binary(BinaryPacker::new()),
		Algorithm::Passthrough => GenericPacker::Passthrough(PassthroughPacker::new()),
		Algorithm::Uniform => GenericPacker::Uniform(UniformPacker::new()),
	};
	let mut atlas =
		DynamicAtlas::<_, ScoredBin2<RgbaImage, RgbaImage>, RgbaImage>::new(options, packer);
	let data = atlas.add_all(&image_list).unwrap();
	let bin_list = atlas.build();

	fs::create_dir_all(&cli.output_dir)?;
	for (i, bin) in bin_list.iter().enumerate() {
		let output_path = cli.output_dir.join(format!("atlas_{}.png", i));
		bin.bin().save(output_path)?;
	}
	println!("Score: {:.2}%", bin_list.as_slice().score() * 100.0);

	let value = toml::to_string(&Config {
		items: data,
	})
	.with_context(|| "Generating TOML")?;
	println!("{value}");

	println!("Done!");

	Ok(())
}
