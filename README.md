# texture_atlas [![Build Status](https://travis-ci.org/TheSpiritXIII/Texture-Atlas.svg?branch=master)](https://travis-ci.org/TheSpiritXIII/Texture-Atlas) [![Coverage Status](https://coveralls.io/repos/github/TheSpiritXIII/Texture-Atlas/badge.svg?branch=master)](https://coveralls.io/github/TheSpiritXIII/Texture-Atlas?branch=main)

This crate provides various algorithms for bin-packing axis-aligned rectangles.

The most common use-case for this library is for game development. To reduce texture swapping on the GPU, multiple textures can be combined into fewer, larger textures.

## Basic Usage

A command-line tool is provided which can generate an atlas from a directory of images.

Alternatively, the library can also be used directly. This can be helpful for writing build scripts or when needing to extend the functionality of this library.

The `image` feature is enabled by default, which allows interoperability with the `image` crate:

```rust
use std::num::NonZeroU32;

use image::RgbaImage;
use texture_atlas::AtlasOptions;
use texture_atlas::BinaryPacker;
use texture_atlas::DynamicBuilder;
use texture_atlas::Pos2;

// Pack a list of images into multiple atlases.
fn pack(image_list: &[&RgbaImage]) -> Vec<RgbaImage> {
	// Output a 1024x1024 image.
	let options =
		AtlasOptions::with_max_size(NonZeroU32::new(1024).unwrap(), NonZeroU32::new(1024).unwrap());

	// Take RgbaImage as input and output. Return the positions of each image.
	let mut builder = DynamicBuilder::<_, RgbaImage, RgbaImage, Pos2>::new(
		options,
		// Use binary packing.
		BinaryPacker::new(),
	);

	// Add the images to the builder. This will give you the aforementioned positions.
	let _position_list = builder.add_all(image_list).unwrap();

	// Output the resulting image atlases.
	builder.build()
}
```

## Nomenclature

- [`Bin`] stores items. For example, a bin could be a larger image which stores smaller images.
- `Item` are the individual units that go into a bin. For example, small images.
- [`Packer`] takes items and places them into bins.
- `Params` is what a packer uses to tell bins how to place an item. For example, [`Pos2`] is the most basic parameter that simply contains an x any y position. The `2` suffix denotes that this is for 2-dimensional bins.
- `Output` is what the packer outputs per-item. This allows you to figure out where am item got placed in which bin.

## Advanced Features

### Params

The earlier example used [`Pos2`]. This adds items to bins as-is. A more flexible approach is to use [`Rotate2`], which rotates your item to help more tightly pack your item.

### Packers

The earlier example used a [`BinaryPacker`]. 

The full list of packers include:
- [`BinaryPacker`]
- [`PassthroughPacker`]
- [`UniformPacker`]

## Contributing

The repository is a workspace with 3 sub-projects:
- `main`: The library.
- `cli`: The command-line tool.
- `gen`: Generates random-colored images within a random range.

The following script generates images and runs the command-line tool on them, which can help inspect how a packer behaves with real-world data:

```shell
cargo run -p image-generator -- --output-dir generated --amount 16 --min-width 16 --min-height 16 --max-width 128 --max-height 128
RUST_LOG=info cargo run -p cli -- --input-dir generated --output-dir atlas --output-file atlas/output.toml --max-width 256 --max-height 256 --rotatable --format json binary
```
