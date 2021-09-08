use super::colors::{colordict::ColorDict, convert::{hex2rgbdisplay, hex2xrgb}};
use home::home_dir;
use std::fs::{create_dir_all, read_dir, read_to_string, write};

pub fn export(dict: &ColorDict) {
	let templatedir = format!("{}/.config/wal/", home_dir().unwrap().display().to_string());

	create_dir_all(&templatedir).unwrap();
	create_dir_all(&templatedir.replace("/.config/", "/.cache/")).unwrap();

	for file in read_dir(templatedir).unwrap() {
		let file = file.unwrap().path();
		if file.is_dir() {
			continue;
		}
		let path = file.display().to_string();
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

		for (i, entry) in dict.colorvec.to_vec().into_iter().enumerate() {
			new_data = parameters(new_data, format!("color{}", i), &entry);
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
