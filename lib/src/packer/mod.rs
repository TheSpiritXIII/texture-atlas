mod common;
// TODO: Add test feature for external packers.
#[cfg(test)]
mod test;

#[cfg(feature = "packer-binary")]
mod binary;
#[cfg(feature = "packer-passthrough")]
mod passthrough;
#[cfg(feature = "packer-uniform")]
mod uniform;

#[cfg(feature = "packer-binary")]
pub use binary::*;
pub use common::*;
#[cfg(
	any(
		test,
		feature = "packer-passthrough"
	)
)]
pub use passthrough::*;
#[cfg(test)]
pub use test::*;
#[cfg(
	any(
		test,
		feature = "packer-uniform"
	)
)]
pub use uniform::*;
