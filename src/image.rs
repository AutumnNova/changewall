use rand::Rng;
use std::{ffi::OsStr, fs::read_dir, path::Path, process::exit};

pub fn image(path: String) -> String {
	if Path::new(&path).is_dir() {
		let file = rand(path);
		file
	} else if Path::new(&path).is_file() && is_img(&path) {
		path
	} else {
		println!("Path does not point to a valid file/directory");
		exit(0);
	}
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
	Path::new(filename).extension().and_then(OsStr::to_str)
}

fn dir(path: String) -> Vec<String> {
	let mut vec = vec![];
	for dir in read_dir(path).unwrap() {
		let file = dir.unwrap().path().display().to_string();
		if is_img(&file) {
			vec.push(file);
		}
	}
	vec
}

fn is_img(file: &str) -> bool {
	match get_extension_from_filename(&file).unwrap() {
		"png" => true,
		"jpg" => true,
		"jpeg" => true,
		"jpe" => true,
		"gif" => true,
		_ => false,
	}
}

fn rand(path: String) -> String {
	let num = rand::thread_rng().gen_range(0..path.len());
	dir(path).into_iter().nth(num).unwrap()
}
