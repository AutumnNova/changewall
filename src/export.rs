use super::colors::{colordict::ColorDict, convert::{hex2rgbdisplay, hex2xrgb}};
use home::home_dir;
use std::fs::{create_dir_all, read_dir, read_to_string, write};

pub fn export(dict: &ColorDict) {
	let mut colorvec = dict.colorvec.to_vec();
	let templatedir = format!("{}/.config/wal/", home_dir().unwrap().display().to_string());

	let _ = create_dir_all(&templatedir);
	let _ = create_dir_all(&templatedir.replace("/.config/", "/.cache/"));

	for file in read_dir(templatedir).unwrap() {
		if file.as_ref().unwrap().path().is_dir() {
			continue;
		}
		let path = file.unwrap().path().display().to_string();
		let dat = read_to_string(&path).unwrap();

		// Run the replace operation in memory
		let mut new_data = dat
			.replace("{wallpaper}", &dict.wallpaper)
			.replace("{alpha}", &dict.alpha.to_string())
			.replace("{alpha.decimal}", &(dict.alpha / 100).to_string())
			.replace("{background.alpha}", &format!("[{}]{}", dict.alpha, dict.background));
		new_data = parameters(new_data, "foreground".to_string(), &dict.foreground);
		new_data = parameters(new_data, "background".to_string(), &dict.background);
		new_data = parameters(new_data, "cursor".to_string(), &dict.cursor);

		let mut i = 0;
		for entry in &mut colorvec {
			new_data = parameters(new_data, format!("color{}", i), entry);
			i += 1;
		}

		write(path.replace("/.config/", "/.cache/"), new_data).expect("write failed");
	}
}

fn parameters(data: String, from: String, to: &str) -> String {
	data.replace(&format!("{{{}}}", from), to)
		.replace(&format!("{{{}.strip}}", from), to.strip_prefix('#').unwrap())
		.replace(&format!("{{{}.rgb}}", from), &hex2rgbdisplay(to))
		.replace(&format!("{{{}.xrgba}}", from), &hex2xrgb(to))
}
