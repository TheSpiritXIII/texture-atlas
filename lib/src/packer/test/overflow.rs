use std::fmt::Debug;

use crate::AtlasOptions;
use crate::Packer as AtlasPacker;
use crate::Pos2;
use crate::Size2;
use crate::new_bin;

pub const MAX_WIDTH: u32 = 1024;
pub const MAX_HEIGHT: u32 = 1024;

/// Asserts overflow behavior for various algorithm-agnostic size scenarios.
pub fn assert_add_overflow<Packer, Error>(options: &AtlasOptions, packer: Packer)
where
	Packer: AtlasPacker<Size2, Pos2, AtlasOptions, Error = Error> + Clone,
	Error: Debug + PartialEq,
{
	assert_add_overflow_with(
		options,
		packer.clone(),
		Size2::new(MAX_WIDTH, MAX_HEIGHT),
		"max size",
	);
	assert_add_overflow_with(
		options,
		packer.clone(),
		Size2::new(MAX_WIDTH + 1, MAX_HEIGHT + 1),
		"overflow size",
	);
	assert_add_overflow_with(
		options,
		packer.clone(),
		Size2::new(u32::MAX, u32::MAX),
		"numeric limit",
	);
}

/// Asserts overflow behavior for a specific size and scenario.
pub fn assert_add_overflow_with<Packer, Error>(
	options: &AtlasOptions,
	packer: Packer,
	size: Size2,
	scenario: &str,
) where
	Packer: AtlasPacker<Size2, Pos2, AtlasOptions, Error = Error> + Clone,
	Error: Debug + PartialEq,
{
	assert_add_overflow_then_small(options, packer.clone(), size, scenario);
	assert_add_small_then_overflow(options, packer.clone(), size, scenario);

	assert_add_all_overflow_then_small(options, packer.clone(), size, scenario);
	assert_add_all_small_then_overflow(options, packer.clone(), size, scenario);

	assert_add_group_overflow_then_small(options, packer.clone(), size, scenario);
	assert_add_group_small_then_overflow(options, packer, size, scenario);
}

/// Tests adding a large item first, then a small one, expecting overflow.
pub fn assert_add_overflow_then_small<T: Debug + PartialEq>(
	options: &AtlasOptions,
	mut packer: impl AtlasPacker<Size2, Pos2, AtlasOptions, Error = T>,
	size: Size2,
	scenario: &str,
) {
	assert_eq!(
		packer.add(options, &size),
		Ok(new_bin()),
		"{}: initial large item must have new bin",
		scenario
	);
	assert_eq!(
		packer.add(options, &Size2::new(1, 1)),
		Ok(new_bin()),
		"{}: small item must be overflowed to new bin",
		scenario
	);
}

/// Tests adding a small item first, then a large one, expecting overflow.
pub fn assert_add_small_then_overflow<T: Debug + PartialEq>(
	options: &AtlasOptions,
	mut packer: impl AtlasPacker<Size2, Pos2, AtlasOptions, Error = T>,
	size: Size2,
	scenario: &str,
) {
	assert_eq!(
		packer.add(options, &size),
		Ok(new_bin()),
		"{}: initial small item must have new bin",
		scenario
	);
	assert_eq!(
		packer.add(options, &Size2::new(1, 1)),
		Ok(new_bin()),
		"{}: large item must have overflowed to new bin",
		scenario
	);
}

/// Tests add_all with large item first, then small, expecting overflow.
pub fn assert_add_all_overflow_then_small<T: Debug + PartialEq>(
	options: &AtlasOptions,
	mut packer: impl AtlasPacker<Size2, Pos2, AtlasOptions, Error = T>,
	size: Size2,
	scenario: &str,
) {
	assert_eq!(
		packer.add_all(options, &[size]).into_iter().collect::<Vec<_>>(),
		vec![Ok((0, new_bin()))],
		"{}: initial large item must have new bin",
		scenario
	);
	assert_eq!(
		packer.add_all(options, &[Size2::new(1, 1)]).into_iter().collect::<Vec<_>>(),
		vec![Ok((0, new_bin()))],
		"{}: small item must be overflowed to new bin",
		scenario
	);
}

/// Tests add_all with small item first, then large, expecting overflow.
pub fn assert_add_all_small_then_overflow<T: Debug + PartialEq>(
	options: &AtlasOptions,
	mut packer: impl AtlasPacker<Size2, Pos2, AtlasOptions, Error = T>,
	size: Size2,
	scenario: &str,
) {
	assert_eq!(
		packer.add_all(options, &[Size2::new(1, 1)]).into_iter().collect::<Vec<_>>(),
		vec![Ok((0, new_bin()))],
		"{}: initial small item must have new bin",
		scenario
	);
	assert_eq!(
		packer.add_all(options, &[size]).into_iter().collect::<Vec<_>>(),
		vec![Ok((0, new_bin()))],
		"{}: large item must have overflowed to new bin",
		scenario
	);
}

/// Tests add_group with large item first, then small, expecting overflow.
pub fn assert_add_group_overflow_then_small<T: Debug + PartialEq>(
	options: &AtlasOptions,
	mut packer: impl AtlasPacker<Size2, Pos2, AtlasOptions, Error = T>,
	size: Size2,
	scenario: &str,
) {
	assert_eq!(
		packer.add_group(options, &[size]).into_iter().collect::<Vec<_>>(),
		vec![Ok((0, new_bin()))],
		"{}: initial large item must have new bin",
		scenario
	);
	assert_eq!(
		packer.add_group(options, &[Size2::new(1, 1)]).into_iter().collect::<Vec<_>>(),
		vec![Ok((0, new_bin()))],
		"{}: small item must be overflowed to new bin",
		scenario
	);
}

/// Tests add_group with small item first, then large, expecting overflow.
pub fn assert_add_group_small_then_overflow<T: Debug + PartialEq>(
	options: &AtlasOptions,
	mut packer: impl AtlasPacker<Size2, Pos2, AtlasOptions, Error = T>,
	size: Size2,
	scenario: &str,
) {
	assert_eq!(
		packer.add_group(options, &[Size2::new(1, 1)]).into_iter().collect::<Vec<_>>(),
		vec![Ok((0, new_bin()))],
		"{}: initial small item must have new bin",
		scenario
	);
	assert_eq!(
		packer.add_group(options, &[size]).into_iter().collect::<Vec<_>>(),
		vec![Ok((0, new_bin()))],
		"{}: large item must have overflowed to new bin",
		scenario
	);
}
