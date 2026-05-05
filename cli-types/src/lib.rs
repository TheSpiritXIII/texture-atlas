use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct Output<T> {
	pub bin_path: String,
	pub item_path: String,
	pub output: T,
}

#[derive(Deserialize, Serialize)]
pub struct Config<T>
where
	T: Serialize,
{
	pub output_list: Vec<Output<T>>,
}
