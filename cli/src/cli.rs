use std::collections::VecDeque;
use std::num::NonZero;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use clap::Parser;
use clap::ValueEnum;
use image::DynamicImage;
use image::ImageReader;
use log::info;
use serde::Deserialize;
use serde::Serialize;

use crate::generic::Algorithm;

// TODO: Config file.
/// Combines multiple images into fewer large atlas images.
#[derive(Deserialize, Parser)]
pub struct Cli {
	#[command(flatten)]
	pub atlas: AtlasArgs,

	#[command(flatten)]
	pub input: InputArgs,

	#[command(flatten)]
	pub output: OutputArgs,
}

#[derive(Args, Deserialize)]
pub struct AtlasArgs {
	#[command(subcommand)]
	pub algorithm: Algorithm,

	/// Maximum width of each atlas.
	#[arg(long)]
	pub max_width: NonZero<u32>,

	/// Maximum height of each atlas.
	#[arg(long)]
	pub max_height: NonZero<u32>,

	/// Margin around each atlas.
	#[arg(
		long,
		default_value_t = 1
	)]
	pub margin: u32,

	/// Spacing between items when packed into an atlas.
	#[arg(
		long,
		default_value_t = 1
	)]
	pub spacing: u32,

	/// Allow rotation of items to improve utilization and potentially reduce the total number of
	/// atlases.
	#[arg(
		long,
		default_value_t = false
	)]
	pub rotatable: bool,
}

#[derive(Args, Deserialize)]
pub struct InputArgs {
	/// Directory containing input images. If any file is not an image, it will be skipped.
	#[arg(long)]
	pub input_dir: Vec<PathBuf>,

	/// Recursively search all input directories for images.
	#[arg(
		long,
		default_value_t = false
	)]
	pub recursive: bool,
}

impl InputArgs {
	pub fn load(&self) -> anyhow::Result<(Vec<PathBuf>, Vec<DynamicImage>)> {
		let mut file_path_list = Vec::new();
		let mut image_list = Vec::new();
		let mut queue: VecDeque<PathBuf> = self.input_dir.iter().cloned().collect();

		while let Some(dir) = queue.pop_front() {
			let entries = dir
				.read_dir()
				.with_context(|| format!("Failed to read input directory: {}", dir.display()))?;
			for entry in entries {
				let entry = entry.with_context(|| "Failed to read directory entry")?;
				let path = entry.path();
				if path.is_dir() {
					if self.recursive {
						queue.push_back(path);
					}
					continue;
				}
				if !path.is_file() {
					continue;
				}
				match parse(&path) {
					Ok(image) => {
						file_path_list.push(path);
						image_list.push(image);
					}
					Err(err) => {
						info!("Skipping unsupported file due to {}: {}", err, path.display());
					}
				}
			}
		}

		Ok((file_path_list, image_list))
	}
}

fn parse(path: impl AsRef<Path>) -> anyhow::Result<DynamicImage> {
	let image = ImageReader::open(path.as_ref())
		.with_context(|| format!("Failed to open image: {}", path.as_ref().display()))?
		.decode()
		.with_context(|| format!("Failed to decode image: {}", path.as_ref().display()))?;
	Ok(image)
}

#[derive(Args, Deserialize)]
pub struct OutputArgs {
	/// Directory to save output atlas images.
	#[arg(long)]
	pub output_dir: PathBuf,

	/// Whether to crop the atlas images.
	#[arg(
		long,
		default_value_t = false
	)]
	pub crop: bool,

	/// File path to save the layout output.
	#[arg(long)]
	pub output_file: Option<PathBuf>,

	/// Format for the layout output file.
	#[arg(long)]
	pub format: Format,
	// TODO: Support larger color channels than 8-bit.
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, ValueEnum)]
pub enum Format {
	/// TOML format
	Toml,
	/// JSON format
	Json,
}

impl Format {
	pub fn serialize_to_string(self, serialize: &impl Serialize) -> anyhow::Result<String> {
		match self {
			Self::Toml => toml::to_string(serialize).with_context(|| "Failed to serialize TOML"),
			Self::Json => {
				serde_json::to_string_pretty(serialize).with_context(|| "Failed to serialize JSON")
			}
		}
	}
}
