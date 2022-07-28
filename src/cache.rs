use super::colors::colordict::to_array;
use super::colors::colordict::ColorDict;
use anyhow::Result;
use home::home_dir;
use std::{fs::{create_dir_all, read_to_string, write}, path::Path};

pub fn writecache(dict: &ColorDict) -> Result<()> {
	let cachedir = home_dir().unwrap().join(".cache/wal/palette");
	create_dir_all(&cachedir)?;

	let mut tmp = String::with_capacity(127);
	tmp.push_str(&dict.colorvec.join("\n"));
	write(cachedir.join(dict.wallpaper.to_str().unwrap().replace('/', "%")), tmp)?;
	Ok(())
}

pub fn readcache(wallpaper: &Path, alpha: &u8) -> Result<ColorDict> {
	let cachedir = home_dir().unwrap().join(".cache/wal/palette");
	let data = read_to_string(cachedir.join(&wallpaper.to_str().unwrap().replace('/', "%")))?;
	let colorvec: [String; 16] = to_array(data.lines().map(|x|x.to_string()).collect());
	let dict = ColorDict::new(wallpaper, *alpha, colorvec);
	Ok(dict)
}
