use std::path::PathBuf;

#[derive(Clone)]
pub struct ColorDict {
	pub wallpaper: PathBuf,
	pub alpha: usize,
	pub background: String,
	pub foreground: String,
	pub cursor: String,
	pub colorvec: Vec<String>,
}

impl ColorDict {
	pub fn new(wallpaper: PathBuf, alpha: usize, background: String, foreground: String, cursor: String, colorvec: Vec<String>) -> Self { Self { wallpaper, alpha, background, foreground, cursor, colorvec } }
}
