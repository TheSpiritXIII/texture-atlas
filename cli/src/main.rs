//! A CLI for the `texture-atlas` library.
//!
//! ## Example Usage:
//!
//! The follow script takes all images in `input/` and outputs atlases in `output/`. The layout data
//! is stored in `output/output.json`. See `texture-atlas-cli-types` for `serde`-compatible types
//! from this crate for parsing this data.
//!
//! ```shell
//! RUST_LOG=INFO texture-atlas-cli --input-dir input --output-dir output --output-file output/output.json \
//!   --max-width 256 --max-height 256 --rotatable --format json binary
//! ```
//!
//! To see all possible arguments, run `texture-atlas-cli --help`.

mod cli;
mod generic;

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use image::DynamicImage;
use image::GenericImageView;
use image::RgbaImage;
use log::info;
use serde::Serialize;
use texture_atlas::Bin;
use texture_atlas::BinAdd;
use texture_atlas::DynamicBuilder;
use texture_atlas::ImageExt;
use texture_atlas::Options2;
use texture_atlas::Packer;
use texture_atlas::Pos2;
use texture_atlas::Rotate2;
use texture_atlas::Utilization;
use texture_atlas::UtilizationBin2;
use texture_atlas_cli_types::Config;
use texture_atlas_cli_types::Item;

use crate::cli::Cli;
use crate::cli::Format;
use crate::generic::GenericPacker;

fn create_atlas<Output>(
	options: Options2,
	packer: GenericPacker,
	image_list: &[DynamicImage],
	file_path_list: &[PathBuf],
	output_dir: &Path,
	format: Format,
) -> anyhow::Result<(String, Vec<UtilizationBin2<DynamicImage, RgbaImage>>)>
where
	Output: Serialize,
	GenericPacker: Packer<DynamicImage, Output, Options2>,
	<GenericPacker as Packer<DynamicImage, Output, Options2>>::Error:
		std::error::Error + Send + Sync + 'static,
	UtilizationBin2<DynamicImage, RgbaImage>: Bin + BinAdd<DynamicImage, Output>,
{
	let mut atlas =
		DynamicBuilder::<_, UtilizationBin2<DynamicImage, RgbaImage>, DynamicImage, Output>::new(
			options,
			packer,
		);
	let data: Vec<Item<Output>> = atlas
		.add_all(image_list)
		.with_context(|| "Failed to pack images into atlas")?
		.into_iter()
		.map(|result| {
			let output_path = output_dir.join(format!("atlas_{}.png", result.bin_index));
			let item_path = file_path_list
				.get(result.item_index)
				.ok_or_else(|| anyhow::anyhow!("Invalid item index: {}", result.item_index))?;
			Ok(Item {
				bin_path: output_path.to_string_lossy().into_owned(),
				item_path: item_path.to_string_lossy().into_owned(),
				layout: result.output,
			})
		})
		.collect::<anyhow::Result<Vec<_>>>()?;
	let bin_list = atlas.build();
	let value = format.serialize_to_string(&Config {
		item_list: data,
	})?;
	Ok((value, bin_list))
}

fn main() -> anyhow::Result<()> {
	env_logger::init();
	let cli = Cli::parse();

	let (file_path_list, image_list) = cli.input.load()?;

	let options = Options2::with_max_size(cli.atlas.max_width, cli.atlas.max_height)
		.and_margin(cli.atlas.margin)
		.and_spacing(cli.atlas.spacing);
	let packer = cli.atlas.algorithm.into_packer();
	let (value, bin_list) = if cli.atlas.rotatable {
		create_atlas::<Rotate2>(
			options,
			packer,
			&image_list,
			&file_path_list,
			&cli.output.output_dir,
			cli.output.format,
		)?
	} else {
		create_atlas::<Pos2>(
			options,
			packer,
			&image_list,
			&file_path_list,
			&cli.output.output_dir,
			cli.output.format,
		)?
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
	info!("Input images: {}", image_list.len());
	info!("Output images: {}", bin_list.len());
	info!("Utilization: {:.2}%", bin_list.as_slice().utilization() * 100.0);

	Ok(())
}
