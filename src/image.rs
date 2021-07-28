use rand::Rng;
use std::{ffi::OsStr, fs::{metadata, read_dir}, path::Path, process::Command};

fn get_extension_from_filename(filename: &str) -> Option<&str> {
	Path::new(filename).extension().and_then(OsStr::to_str)
}

pub fn image(path: String, setting: String) -> String {
	if metadata(&path).expect("error getting path meta").is_dir() {
		let path = dir(path);
		let file = rand(path);
		feh(&file, setting);
		file
	} else if is_img(&path) {
		feh(&path, setting);
		path
	} else {
		"".to_string()
	}
}

fn dir(path: String) -> Vec<String> {
	let files = read_dir(path).unwrap();
	let mut vec = vec![];
	for dir in files {
		let file = dir.unwrap().path().display().to_string();
		if is_img(&file) {
			vec.push(file);
		}
	}
	vec
}

fn is_img(file: &String) -> bool {
	match get_extension_from_filename(&file).unwrap() {
		"png" => true,
		"jpg" => true,
		"jpeg" => true,
		"jpe" => true,
		"gif" => true,
		_ => false,
	}
}

fn rand(mut path: Vec<String>) -> String {
	let num = rand::thread_rng().gen_range(0..path.len());
	let mut i = 1;
	while num > i {
		path.pop();
		i += 1;
	}

	path.pop().unwrap()
}

fn feh(path: &String, setting: String) {
	Command::new("feh")
		.args(["--no-fehbg", &format!("--bg-{}", setting), &path])
		.spawn()
		.expect("feh failed");
}
