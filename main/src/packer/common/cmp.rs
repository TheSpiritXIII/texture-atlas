use std::cmp::Ordering;

use crate::AtlasRect;
use crate::AtlasRectExt;

/// Represents the different ways to prioritize items to be added.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortKind {
	/// Sorts such that the greatest area comes first.
	Area,
	/// Sorts such that the greatest width comes first.
	Width,
	/// Sorts such that the greatest height comes first.
	Height,
	/// Sorts such that the greatest maximum side comes first.
	Maximum,
}

impl SortKind {
	pub fn cmp<T: AtlasRect>(&self, a: &T, b: &T) -> Ordering {
		match self {
			SortKind::Width => cmp_by_width(a, b),
			SortKind::Height => cmp_by_height(a, b),
			SortKind::Maximum => cmp_by_max(a, b),
			SortKind::Area => cmp_by_area(a, b),
		}
	}
}

/// Compares the areas of the two items. If the area is the same, uses [`cmp_by_max`].
pub fn cmp_by_area<T: AtlasRect>(a: &T, b: &T) -> Ordering {
	let area_ordering = a.area().cmp(&b.area());
	if area_ordering == Ordering::Equal {
		cmp_by_max(a, b)
	} else {
		area_ordering
	}
}

fn cmp_by_size(a: (u32, u32), b: (u32, u32)) -> Ordering {
	let width_ordering = a.0.cmp(&b.0);
	if width_ordering == Ordering::Equal {
		a.1.cmp(&b.1)
	} else {
		width_ordering
	}
}

/// Sorts by the item's width.
pub fn cmp_by_width<T: AtlasRect>(a: &T, b: &T) -> Ordering {
	cmp_by_size((a.width(), a.height()), (b.width(), b.height()))
}

/// Sorts by the item's height.
pub fn cmp_by_height<T: AtlasRect>(a: &T, b: &T) -> Ordering {
	cmp_by_size((a.height(), a.width()), (b.height(), b.width()))
}

/// Sorts by the item's largest side. If the largest sides are the same, uses [`cmp_by_width`].
pub fn cmp_by_max<T: AtlasRect>(a: &T, b: &T) -> Ordering {
	let size_a = if a.width() > a.height() {
		(a.width(), a.height())
	} else {
		(a.height(), a.width())
	};
	let size_b = if b.width() > b.height() {
		(b.width(), b.height())
	} else {
		(b.height(), b.width())
	};
	let size_ordering = cmp_by_size(size_a, size_b);
	if size_ordering == Ordering::Equal {
		cmp_by_width(a, b)
	} else {
		size_ordering
	}
}
