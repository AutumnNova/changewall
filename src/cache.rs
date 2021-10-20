use super::colors::colordict::ColorDict;
use anyhow::Result;
use home::home_dir;
use std::{fs::{create_dir_all, read_to_string, write}, path::Path};

pub fn writecache(dict: &ColorDict) {
	let cachedir = home_dir().unwrap().join(".cache/wal/palette");
	create_dir_all(&cachedir).unwrap();

	let mut tmp = String::with_capacity(152);
	for color in dict.colorvec.to_vec().into_iter() {
		tmp.push_str(&color);
		tmp.push('\n');
	}
	tmp.push_str(&dict.foreground);
	tmp.push('\n');
	tmp.push_str(&dict.background);
	tmp.push('\n');
	tmp.push_str(&dict.cursor);
	tmp.push('\n');
	write(cachedir.join(dict.wallpaper.to_str().unwrap().replace('/', "%")), tmp).unwrap();
}

pub fn readcache(wallpaper: &Path, alpha: &usize) -> Result<ColorDict> {
	let cachedir = home_dir().unwrap().join(".cache/wal/palette");
	let data = read_to_string(cachedir.join(&wallpaper.to_str().unwrap().replace('/', "%")))?;
	let mut ln = data.lines();
	let mut colorvec = Vec::<String>::with_capacity(16);
	while colorvec.len() != 16 {
		colorvec.push(ln.next().unwrap().to_string());
	}
	let foreground = ln.next().unwrap().to_string();
	let background = ln.next().unwrap().to_string();
	let cursor = ln.next().unwrap().to_string();
	let dict = ColorDict::new(wallpaper.to_path_buf(), *alpha, background, foreground, cursor, colorvec);
	Ok(dict)
}
