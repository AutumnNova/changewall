use std::path::PathBuf;

pub struct ColorDict {
	pub wallpaper: PathBuf,
	pub alpha: u8,
	pub colorvec: [String; 16],
}

impl ColorDict {
	pub fn new<P: Into<PathBuf>> (wallpaper: P, alpha: u8, colorvec: [String; 16]) -> Self { Self { wallpaper: wallpaper.into(), alpha, colorvec } }
}

pub fn to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
	v.try_into()
		.unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
