use std::fs;
use std::io;
use std::num::NonZero;
use std::path::PathBuf;

use clap::Parser;
use image::ImageReader;
use image::RgbaImage;
use texture_atlas::AtlasOptions;
use texture_atlas::DynamicAtlas;
use texture_atlas::PassthroughPacker;

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
}

fn main() -> io::Result<()> {
	let cli = Cli::parse();

	let mut image_list = Vec::new();
	for entry in cli.input_dir.read_dir()? {
		let entry = entry?;
		let path = entry.path();
		if path.is_file() {
			let image = ImageReader::open(path)?.decode().unwrap();
			image_list.push(image.to_rgba8());
		}
	}

	let options = AtlasOptions::with_max_size(cli.max_width, cli.max_height);
	let mut atlas = DynamicAtlas::<_, RgbaImage, RgbaImage>::new(options, PassthroughPacker::new());
	atlas.add_all(&image_list).unwrap();
	let page_list = atlas.consume();

	fs::create_dir_all(&cli.output_dir)?;
	for (i, page) in page_list.into_iter().enumerate() {
		let output_path = cli.output_dir.join(format!("atlas_{}.png", i));
		page.save(output_path).unwrap();
	}

	Ok(())
}
