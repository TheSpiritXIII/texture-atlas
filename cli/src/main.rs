mod generic;

use std::fs;
use std::num::NonZero;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use clap::Parser;
use clap::ValueEnum;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageReader;
use image::RgbaImage;
use log::info;
use texture_atlas::DynamicBuilder;
use texture_atlas::ImageExt;
use texture_atlas::Options2;
use texture_atlas::Pos2;
use texture_atlas::Rotate2;
use texture_atlas::Scored;
use texture_atlas::ScoredBin2;
use texture_atlas_cli_types::Config;
use texture_atlas_cli_types::Item;

use crate::generic::Algorithm;

#[derive(Parser)]
struct Cli {
	#[command(flatten)]
	pub atlas: AtlasArgs,

	#[command(flatten)]
	pub input: InputArgs,

	#[command(flatten)]
	pub output: OutputArgs,
}

#[derive(Args)]
struct AtlasArgs {
	#[command(subcommand)]
	algorithm: Algorithm,

	/// Maximum width of each atlas.
	#[arg(long)]
	max_width: NonZero<u32>,

	/// Maximum height of each atlas.
	#[arg(long)]
	max_height: NonZero<u32>,

	/// Margin around each atlas.
	#[arg(long)]
	margin: u32,

	/// Spacing between items when packed into an atlas.
	#[arg(
		long,
		default_value_t = 1
	)]
	spacing: u32,

	/// Allow rotation of items to improve utilization and potentially reduce the total number of
	/// atlases.
	#[arg(long)]
	rotatable: bool,
}

#[derive(Args)]
struct InputArgs {
	// TODO: Support recursive inputs.
	/// Directory containing input images. If any file is not an image, it will be skipped.
	/// Directories will also be skipped (recursive inputs are not yet supported).
	#[arg(long)]
	input_dir: PathBuf,
}

#[derive(Args)]
struct OutputArgs {
	/// Directory to save output atlas images.
	#[arg(long)]
	output_dir: PathBuf,

	/// Whether to crop the atlas images.
	#[arg(long)]
	crop: bool,

	/// File path to save the layout output.
	#[arg(long)]
	output_file: Option<PathBuf>,

	/// Format for the layout output file.
	#[arg(long)]
	format: Format,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum Format {
	/// TOML format
	Toml,
	/// JSON format
	Json,
}

enum ConfigType {
	Pos(Vec<Item<Pos2>>),
	Rotate(Vec<Item<Rotate2>>),
}

// TODO: Use try blocks when they're ready.
fn parse(path: &impl AsRef<Path>) -> anyhow::Result<DynamicImage> {
	let image = ImageReader::open(path.as_ref())
		.with_context(|| format!("Failed to open image: {:?}", path.as_ref().display()))?
		.decode()
		.with_context(|| format!("Failed to decode image: {:?}", path.as_ref().display()))?;
	Ok(image)
}

fn main() -> anyhow::Result<()> {
	env_logger::init();
	let cli = Cli::parse();

	let mut file_path_list = Vec::new();
	let mut image_list = Vec::new();
	for entry in cli.input.input_dir.read_dir().with_context(|| "Failed to read input directory")? {
		let entry = entry.with_context(|| "Failed to read directory entry")?;
		let path = entry.path();
		if !path.is_file() {
			continue;
		}
		match parse(&path) {
			Ok(image) => {
				file_path_list.push(path);
				image_list.push(image.to_rgba8());
			}
			Err(err) => {
				info!("Skipping unsupported file due to {:?}: {:?}", err, path.display());
			}
		}
	}

	let options = Options2::with_max_size(cli.atlas.max_width, cli.atlas.max_height)
		.and_margin(cli.atlas.margin)
		.and_spacing(cli.atlas.spacing);
	let packer = cli.atlas.algorithm.into_packer();
	let (data, bin_list) = if cli.atlas.rotatable {
		let mut atlas =
			DynamicBuilder::<_, ScoredBin2<RgbaImage, RgbaImage>, RgbaImage, Rotate2>::new(
				options,
				packer,
			);
		// TODO: Consider thiserror for library errors so we could use with_context.
		let data = atlas
			.add_all(&image_list)
			.unwrap()
			.into_iter()
			.map(|result| {
				let output_path =
					cli.output.output_dir.join(format!("atlas_{}.png", result.bin_index));
				let item_path = &file_path_list[result.item_index];
				Item {
					bin_path: output_path.to_string_lossy().to_string(),
					item_path: item_path.to_string_lossy().to_string(),
					layout: result.output,
				}
			})
			.collect();
		let bin_list = atlas.build();
		(ConfigType::Rotate(data), bin_list)
	} else {
		let mut atlas = DynamicBuilder::<_, ScoredBin2<RgbaImage, RgbaImage>, RgbaImage, Pos2>::new(
			options,
			packer,
		);
		// TODO: Consider thiserror for library errors so we could use with_context.
		let data = atlas
			.add_all(&image_list)
			.unwrap()
			.into_iter()
			.map(|result| {
				let output_path =
					cli.output.output_dir.join(format!("atlas_{}.png", result.bin_index));
				let item_path = &file_path_list[result.item_index];
				Item {
					bin_path: output_path.to_string_lossy().to_string(),
					item_path: item_path.to_string_lossy().to_string(),
					layout: result.output,
				}
			})
			.collect();
		let bin_list = atlas.build();
		(ConfigType::Pos(data), bin_list)
	};

	fs::create_dir_all(&cli.output.output_dir).with_context(|| {
		format!("Failed to create output directory: {:?}", cli.output.output_dir)
	})?;
	for (i, bin) in bin_list.iter().enumerate() {
		let output_path = cli.output.output_dir.join(format!("atlas_{}.png", i));
		let image = bin.bin();
		let image_cropped = if let Some((image, _)) = image.crop_margin(cli.atlas.margin) {
			image
		} else {
			image.view(0, 0, image.width(), image.height())
		};
		image_cropped
			.to_image()
			.save(&output_path)
			.with_context(|| format!("Failed to save atlas image: {:?}", output_path))?;
	}

	let value = match data {
		ConfigType::Pos(data) => {
			match cli.output.format {
				Format::Toml => {
					toml::to_string(&Config {
						item_list: data,
					})
					.with_context(|| "Failed to generate TOML")?
				}
				Format::Json => {
					serde_json::to_string_pretty(&Config {
						item_list: data,
					})
					.with_context(|| "Failed to generate TOML")?
				}
			}
		}
		ConfigType::Rotate(data) => {
			match cli.output.format {
				Format::Toml => {
					toml::to_string(&Config {
						item_list: data,
					})
					.with_context(|| "Failed to generate TOML")?
				}
				Format::Json => {
					serde_json::to_string_pretty(&Config {
						item_list: data,
					})
					.with_context(|| "Failed to generate TOML")?
				}
			}
		}
	};
	if let Some(output_file) = cli.output.output_file {
		if let Some(parent) = output_file.parent() {
			fs::create_dir_all(parent)
				.with_context(|| format!("Failed to create parent directory: {:?}", parent))?;
		}
		fs::write(&output_file, &value)
			.with_context(|| format!("Failed to write config file: {:?}", output_file))?;
	} else {
		println!("{value}");
	}

	info!("Done!");
	info!("Input images: {}%", image_list.len());
	info!("Output images: {}%", bin_list.len());
	info!("Score: {:.2}%", bin_list.as_slice().score() * 100.0);

	Ok(())
}
