use rand::Rng;
use std::{fs::read_dir, path::Path, process::exit};
use tree_magic_mini::from_filepath;
pub fn file(path: String) -> String {
	let pathdir = Path::new(&path);
	if pathdir.is_dir() {
		rand(path)
	} else if pathdir.is_file() && is_img(pathdir) {
		path
	} else {
		println!("Path does not point to a valid file/directory");
		exit(0);
	}
}

fn dir(path: String) -> Vec<String> {
	let mut vec = vec![];
	for dir in read_dir(path).unwrap() {
		let file = dir.unwrap().path();
		if is_img(&file) {
			vec.push(file.display().to_string());
		}
	}
	vec
}

fn is_img(file: &Path) -> bool {
	matches!(from_filepath(file).unwrap(), "image/jpeg" | "image/png" | "image/avif" | "image/bmp" | "image/webp" | "image/tiff")
}

fn rand(path: String) -> String {
	let valid = dir(path);
	let num = rand::thread_rng().gen_range(0..valid.len());
	valid.into_iter().nth(num).unwrap()
}
