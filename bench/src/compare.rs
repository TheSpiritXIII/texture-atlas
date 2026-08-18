use std::fmt::Debug;
use std::io;
use std::io::Write;
use std::num::NonZero;
use std::time::Duration;
use std::time::Instant;

use clap::Args;
use image::DynamicImage;
use image::RgbaImage;
use image_generator::GenerateArgs;
use log::info;
use texture_atlas::BinAdd;
use texture_atlas::BinaryPacker;
use texture_atlas::DynamicBuilder;
use texture_atlas::Options2;
use texture_atlas::Packer as AtlasPacker;
use texture_atlas::PassthroughPacker;
use texture_atlas::Pos2;
use texture_atlas::Rotate2;
use texture_atlas::UniformPacker;
use texture_atlas::Utilization;
use texture_atlas::UtilizationBin2;

/// CLI arguments for running packing algorithm comparison benchmarks.
#[derive(Args, Clone, Debug)]
pub struct CompareArgs {
	#[command(flatten)]
	pub generate: GenerateArgs,

	/// Maximum width of each atlas bin.
	#[arg(
		long,
		default_value = "512"
	)]
	pub atlas_max_width: NonZero<u32>,

	/// Maximum height of each atlas bin.
	#[arg(
		long,
		default_value = "512"
	)]
	pub atlas_max_height: NonZero<u32>,

	/// Margin around each atlas bin.
	#[arg(
		long,
		default_value_t = 1
	)]
	pub margin: u32,

	/// Spacing between items when packed into an atlas bin.
	#[arg(
		long,
		default_value_t = 1
	)]
	pub spacing: u32,

	/// Number of benchmark iterations per algorithm to average timing over.
	#[arg(
		long,
		default_value = "5"
	)]
	pub iterations: NonZero<u32>,
}

impl CompareArgs {
	pub fn run(&self, writer: &mut impl Write) -> anyhow::Result<()> {
		let (mut rng, seed) = self.generate.rng();
		if self.generate.seed.is_none() {
			info!("Generated seed: {seed}");
		}

		info!("Generating {} images...", self.generate.amount);
		let image_list: Vec<DynamicImage> =
			self.generate.generate(&mut rng).map(DynamicImage::ImageRgb8).collect();

		let options = Options2::with_max_size(self.atlas_max_width, self.atlas_max_height)
			.and_margin(self.margin)
			.and_spacing(self.spacing);

		info!("Running benchmarks with rotatable = false...");
		let bench_pos2 = build_table::<Pos2>(&options, &image_list, self.iterations)?;

		info!("Running benchmarks with rotatable = true...");
		let bench_rotate2 = build_table::<Rotate2>(&options, &image_list, self.iterations)?;

		info!("Done. Printing results...");
		writeln!(writer, "### Benchmark Configuration\n")?;
		self.write_markdown(writer)?;

		writeln!(writer, "### Benchmark Results (Non-Rotatable)\n")?;
		BenchResult::write_markdown(writer, &bench_pos2)?;
		writeln!(writer)?;

		writeln!(writer, "### Benchmark Results (Rotatable)\n")?;
		BenchResult::write_markdown(writer, &bench_rotate2)?;
		writeln!(writer)?;

		Ok(())
	}

	fn write_markdown(&self, writer: &mut impl Write) -> io::Result<()> {
		writeln!(writer, "| Parameter | Value |")?;
		writeln!(writer, "|---|---|")?;
		writeln!(writer, "| Images | {} |", self.generate.amount)?;
		writeln!(writer, "| Iterations | {} |", self.iterations)?;
		writeln!(
			writer,
			"| Atlas Max Size | {}x{} |",
			self.atlas_max_width, self.atlas_max_height
		)?;
		writeln!(
			writer,
			"| Image Size Range | {}x{} to {}x{} |",
			self.generate.options.min_width,
			self.generate.options.min_height,
			self.generate.options.max_width,
			self.generate.options.max_height
		)?;
		writeln!(writer, "| Margin | {} |", self.margin)?;
		writeln!(writer, "| Spacing | {} |\n", self.spacing)
	}
}

/// Represents the results of a benchmark for a single algorithm configuration.
#[derive(Copy, Clone, Debug, PartialEq)]
struct BenchResult {
	pub algorithm_name: &'static str,
	pub bin_count: usize,
	pub average_time_sec: f64,
	pub utilization: f64,
}

impl BenchResult {
	/// Writes the benchmark result as a markdown table row to the provided writer.
	fn write_markdown_row(
		&self,
		writer: &mut impl Write,
		is_best_bin_count: bool,
		is_best_time: bool,
		is_best_utilization: bool,
	) -> io::Result<()> {
		write!(writer, "| {} |", self.algorithm_name)?;
		if is_best_bin_count {
			write!(writer, "**{}**", self.bin_count)?;
		} else {
			write!(writer, "{}", self.bin_count)?;
		}

		write!(writer, " | ")?;
		if is_best_time {
			write!(writer, "**{:.3}**", self.average_time_sec * 1000.0)?;
		} else {
			write!(writer, "{:.3}", self.average_time_sec * 1000.0)?;
		}

		write!(writer, " | ")?;
		if is_best_utilization {
			write!(writer, "**{:.2}%**", self.utilization * 100.0)?;
		} else {
			write!(writer, "{:.2}%", self.utilization * 100.0)?;
		}

		writeln!(writer, "|")
	}

	fn write_markdown(writer: &mut impl Write, result_list: &[Self]) -> io::Result<()> {
		writeln!(writer, "| Algorithm | Bins Created | Time (ms) | Utilization (%) |")?;
		writeln!(writer, "|---|---|---|---|")?;

		let mut best = Self {
			algorithm_name: "Best",
			average_time_sec: f64::MAX,
			bin_count: usize::MAX,
			utilization: 0.0,
		};
		for result in result_list {
			best.average_time_sec = best.average_time_sec.min(result.average_time_sec);
			best.bin_count = best.bin_count.min(result.bin_count);
			best.utilization = best.utilization.max(result.utilization);
		}

		for result in result_list {
			result.write_markdown_row(
				writer,
				result.bin_count == best.bin_count,
				result.average_time_sec == best.average_time_sec,
				result.utilization == best.utilization,
			)?;
		}
		Ok(())
	}
}

fn benchmark<Packer, Layout>(
	algorithm: &'static str,
	options: &Options2,
	packer: &Packer,
	images: &[DynamicImage],
	iterations: NonZero<u32>,
) -> anyhow::Result<BenchResult>
where
	Packer: AtlasPacker<DynamicImage, Layout, Options2> + Clone,
	<Packer as AtlasPacker<DynamicImage, Layout, Options2>>::Error: Debug,
	RgbaImage: BinAdd<DynamicImage, Layout>,
{
	let mut total_duration = Duration::ZERO;
	let mut bin_count = 0;
	let mut utilization = 0.0f64;

	info!("Running benchmark for {algorithm}...");
	for _ in 0..iterations.get() {
		let mut atlas = DynamicBuilder::<
			_,
			UtilizationBin2<DynamicImage, RgbaImage>,
			DynamicImage,
			Layout,
		>::new(options.clone(), packer.clone());

		let start = Instant::now();
		atlas.add_all(images).map_err(|e| {
			anyhow::anyhow!("Failed to pack images into atlas with algorithm {algorithm}: {e:?}")
		})?;
		let bin_list = atlas.build();
		let elapsed = start.elapsed();

		total_duration += elapsed;
		// TODO: This does not take into account these being potentially indeterministic.
		bin_count = bin_list.len();
		utilization = f64::from(bin_list.as_slice().utilization());
	}

	Ok(BenchResult {
		algorithm_name: algorithm,
		bin_count,
		average_time_sec: total_duration.as_secs_f64() / f64::from(iterations.get()),
		utilization,
	})
}

fn build_table<Layout>(
	options: &Options2,
	image_list: &[DynamicImage],
	iterations: NonZero<u32>,
) -> anyhow::Result<Vec<BenchResult>>
where
	RgbaImage: BinAdd<DynamicImage, Layout>,
	BinaryPacker: AtlasPacker<DynamicImage, Layout, Options2>,
	<BinaryPacker as AtlasPacker<DynamicImage, Layout, Options2>>::Error: Debug,
	PassthroughPacker: AtlasPacker<DynamicImage, Layout, Options2>,
	<PassthroughPacker as AtlasPacker<DynamicImage, Layout, Options2>>::Error: Debug,
	UniformPacker: AtlasPacker<DynamicImage, Layout, Options2>,
	<UniformPacker as AtlasPacker<DynamicImage, Layout, Options2>>::Error: Debug,
{
	Ok(vec![
		benchmark::<_, Layout>("Binary", options, &BinaryPacker::new(), image_list, iterations)?,
		benchmark::<_, Layout>(
			"Passthrough",
			options,
			&PassthroughPacker::new(),
			image_list,
			iterations,
		)?,
		benchmark::<_, Layout>("Uniform", options, &UniformPacker::new(), image_list, iterations)?,
	])
}
