use anyhow::Result;
use rand::{thread_rng, Rng};
use std::{fs::read_dir, path::Path, process::exit};
use tree_magic_mini::from_filepath;
pub fn file(path: String) -> Result<String> {
	let pathdir = Path::new(&path);
	if pathdir.is_dir() {
		rand(path)
	} else if pathdir.is_file() && is_img(pathdir) {
		Ok(path)
	} else {
		println!("Path does not point to a valid file/directory");
		exit(0);
	}
}

fn dir(path: String) -> Result<Vec<String>> {
	let mut vec = vec![];
	for dir in read_dir(path)? {
		let file = dir?.path();
		if is_img(&file) {
			vec.push(file.display().to_string());
		}
	}
	Ok(vec)
}

fn is_img(file: &Path) -> bool {
	matches!(from_filepath(file).unwrap(), "image/jpeg" | "image/png" | "image/avif" | "image/bmp" | "image/webp" | "image/tiff")
}

fn rand(path: String) -> Result<String> {
	let valid = dir(path)?;
	let num = thread_rng().gen_range(0..valid.len());
	Ok(valid.into_iter().nth(num).unwrap())
}
