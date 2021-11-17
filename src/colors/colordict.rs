use std::path::PathBuf;

pub struct ColorDict {
	pub wallpaper: PathBuf,
	pub alpha: u8,
	pub background: String,
	pub foreground: String,
	pub cursor: String,
	pub colorvec: Vec<String>,
}

impl ColorDict {
	pub fn new<S: Into<String>, P: Into<PathBuf>> (wallpaper: P, alpha: u8, background: S, foreground: S, cursor: S, colorvec: Vec<String>) -> Self { Self { wallpaper: wallpaper.into(), alpha, background: background.into(), foreground: foreground.into(), cursor: cursor.into(), colorvec } }
}
