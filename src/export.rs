use crate::colors::ColorDict;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::{read_dir, read_to_string, write};

pub fn export(dict: &ColorDict) {
	lazy_static! {
		static ref RE: Regex = Regex::new("templates/([a-zA-Z.-]+)").unwrap();
	};

	for file in read_dir("/home/autumn/.config/wal/templates/").unwrap() {
		let path = file.unwrap().path().display().to_string();
		let dat = read_to_string(&path).unwrap();

		// Run the replace operation in memory
		let new_data = dat
			.replace(&*"{wallpaper}", &*dict.wallpaper)
			.replace(&*"{foreground}", &*dict.foreground)
			.replace(&*"{background}", &*dict.background)
			.replace(&*"{cursor}", &*dict.cursor)
			.replace(&*"{color0}", &*dict.color0)
			.replace(&*"{color0.strip}", &*dict.color0.strip_prefix('#').unwrap())
			.replace(&*"{color1}", &*dict.color8)
			.replace(&*"{color2}", &*dict.color9)
			.replace(&*"{color3}", &*dict.color10)
			.replace(&*"{color4}", &*dict.color11)
			.replace(&*"{color5}", &*dict.color12)
			.replace(&*"{color6}", &*dict.color13)
			.replace(&*"{color7}", &*dict.color14)
			.replace(&*"{color8}", &*dict.color15)
			.replace(&*"{color9}", &*dict.color8)
			.replace(&*"{color10}", &*dict.color9)
			.replace(&*"{color11}", &*dict.color10)
			.replace(&*"{color12}", &*dict.color11)
			.replace(&*"{color13}", &*dict.color12)
			.replace(&*"{color14}", &*dict.color13)
			.replace(&*"{color15}", &*dict.color14);

		let mut newpath = "".to_string();

		for dir in RE.captures(&path) {
			newpath.push_str(&format!("/home/autumn/.cache/wal/{}", &dir[1]));
		}

		write(newpath, new_data).expect("write failed");
	}
}
