use super::colors::colordict::ColorDict;
use home::home_dir;
use std::fs::{create_dir_all, read_to_string, write};

pub fn writecache(dict: &ColorDict) {
	let dict: ColorDict = ColorDict::clonedict(dict);

	let cachedir = format!("{}/.cache/wal/palette/", home_dir().unwrap().display().to_string());
	let _ = create_dir_all(&cachedir);

	let mut tmp = String::new();
	for color in dict.colorvec.into_iter() {
		tmp.push_str(&format!("{}\n", color));
	}
	tmp.push_str(&format!("{}\n", dict.foreground));
	tmp.push_str(&format!("{}\n", dict.background));
	tmp.push_str(&format!("{}\n", dict.cursor));
	write(format!("{}{}", cachedir, dict.wallpaper.replace('/', "%")), tmp).expect("write failed");
}

pub fn readcache(path: &str, alpha: &usize) -> ColorDict {
	let cachedir = format!("{}/.cache/wal/palette/", home_dir().unwrap().display().to_string());
	let data = read_to_string(format!("{}{}", cachedir, &path.replace('/', "%"))).unwrap_or(String::new());
	if data != String::new() {
		let mut ln = data.lines();
		let mut dict = ColorDict::new();
		let mut i = 0;
		while i <= 15 {
			dict.colorvec.push(ln.next().unwrap().to_string());
			i += 1;
		}
		dict.foreground = ln.next().unwrap().to_string();
		dict.background = ln.next().unwrap().to_string();
		dict.cursor = ln.next().unwrap().to_string();
		dict.wallpaper = path.to_string();
		dict.alpha = *alpha;
		dict
	} else {
		ColorDict::new()
	}
}
