//! # texture-atlas-cli-types
//!
//! Provides types for deserializing output from the `texture-atlas-cli` command-line tool.
//!
//! ## Example Usage
//!
//! Deserialize CLI output from JSON:
//!
//! ```rust
//! use texture_atlas::Pos2;
//! use texture_atlas_cli_types::Config;
//! use texture_atlas_cli_types::Item;
//!
//! let json_output = r#"{
//!   "item_list": [
//!     {
//!       "bin_path": "atlas.png",
//!       "item_path": "items.png",
//!       "layout": {
//!         "x": 0,
//!         "y": 0
//!       }
//!     }
//!   ]
//! }"#;
//!
//! let config: Config<Pos2> = serde_json::from_str(json_output).expect("Failed to deserialize");
//! ```
//!
//! ## Generic Type Parameter
//!
//! The `T` type parameter in both [`Item<T>`] and [`Config<T>`] allows you to specify
//! what type the `output` field should deserialize into.
//!
//! `texture-atlas-cli` either outputs:
//! - [`Pos2`][texture_atlas::Pos2] if `rotatable` is unset.
//! - [`Rotate2`][texture_atlas::Rotate2] if `rotatable` is set.

use serde::Deserialize;
use serde::Serialize;

/// Represents a single input to a bin packer.
#[derive(Deserialize, Serialize)]
pub struct Item<T> {
	/// The output bin path. Multiple items are generally located in the same bin.
	pub bin_path: String,
	/// The input item path. There is generally only 1 output per-item.
	pub item_path: String,
	/// The output parameters.
	pub layout: T,
}

/// The output of the `texture-atlas-cli` command-line tool.
#[derive(Deserialize, Serialize)]
pub struct Config<T>
where
	T: Serialize,
{
	/// The list of output items.
	pub item_list: Vec<Item<T>>,
}
