use crate::Bin as AtlasBin;
use crate::BinAdd;
use crate::Size2;
use crate::Utilization;
use crate::UtilizationBin2;

const FLOAT_PRECISION: f64 = 0.01;

#[test]
fn bin_empty() {
	let bin = UtilizationBin2::<Size2, Size2>::new(&Size2::new(10, 10));
	assert!(bin.utilization().abs() < FLOAT_PRECISION);
}

#[test]
fn bin_part() {
	let mut bin = UtilizationBin2::<Size2, Size2>::new(&Size2::new(10, 10));
	bin.item_add(&Size2::new(5, 5), &()).unwrap();
	assert!((bin.utilization() - 0.25).abs() < FLOAT_PRECISION);
}

#[test]
fn bin_full() {
	let mut bin = UtilizationBin2::<Size2, Size2>::new(&Size2::new(10, 10));
	bin.item_add(&Size2::new(10, 10), &()).unwrap();
	assert!((bin.utilization() - 1.0).abs() < FLOAT_PRECISION);
}

#[test]
fn slice() {
	let mut bin1 = UtilizationBin2::<Size2, Size2>::new(&Size2::new(10, 10));
	let mut bin2 = UtilizationBin2::<Size2, Size2>::new(&Size2::new(10, 10));

	bin1.item_add(&Size2::new(10, 5), &()).unwrap();
	bin2.item_add(&Size2::new(10, 10), &()).unwrap();

	let bin_list = [
		bin1,
		bin2,
	];
	assert!((bin_list.as_slice().utilization() - 0.75).abs() < FLOAT_PRECISION);
}
