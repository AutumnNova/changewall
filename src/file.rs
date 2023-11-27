use anyhow::Result;
use fastrand::usize;
use std::{fs::{canonicalize, read_dir}, path::{Path, PathBuf}, process::exit};
use tree_magic_mini::from_filepath;
pub fn file(path: &String) -> Result<PathBuf> {
	let path = PathBuf::from(path);
	if path.is_dir() {
		Ok(canonicalize(rand(path)?)?)
	} else if path.is_file() && is_img(&path) {
		Ok(canonicalize(path)?)
	} else {
		println!("Path does not point to a valid file/directory");
		exit(0);
	}
}

fn dir(path: PathBuf) -> Result<Vec<PathBuf>> {
	let mut vec = vec![];
	for dir in read_dir(path)? {
		let file = dir?.path();
		if is_img(&file) {
			vec.push(file);
		}
	}
	Ok(vec)
}

fn is_img(file: &Path) -> bool {
	matches!(from_filepath(file).unwrap(), "image/jpeg" | "image/png" | "image/avif" | "image/bmp" | "image/webp" | "image/tiff")
}

fn rand(path: PathBuf) -> Result<PathBuf> {
	let valid = dir(path)?;
	let num = usize(..valid.len());
	Ok(valid.into_iter().nth(num).unwrap())
}
