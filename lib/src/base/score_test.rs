use crate::Bin as AtlasBin;
use crate::BinAdd;
use crate::Scored;
use crate::ScoredBin2;
use crate::Size2;

#[test]
fn bin_empty() {
	let bin = ScoredBin2::<Size2, Size2>::new(&Size2::new(10, 10));
	assert_eq!(bin.score(), 0.0);
}

#[test]
fn bin_part() {
	let mut bin = ScoredBin2::<Size2, Size2>::new(&Size2::new(10, 10));
	bin.item_add(&Size2::new(5, 5), &()).unwrap();
	assert_eq!(bin.score(), 0.25);
}

#[test]
fn bin_full() {
	let mut bin = ScoredBin2::<Size2, Size2>::new(&Size2::new(10, 10));
	bin.item_add(&Size2::new(10, 10), &()).unwrap();
	assert_eq!(bin.score(), 1.0);
}

#[test]
fn slice() {
	let mut bin1 = ScoredBin2::<Size2, Size2>::new(&Size2::new(10, 10));
	let mut bin2 = ScoredBin2::<Size2, Size2>::new(&Size2::new(10, 10));

	bin1.item_add(&Size2::new(10, 5), &()).unwrap();
	bin2.item_add(&Size2::new(10, 10), &()).unwrap();

	let bins = [
		bin1,
		bin2,
	];
	assert_eq!(bins.as_slice().score(), 0.75);
}
