use crate::colordict::ColorDict;
use crate::colors::{hex2rgbdisplay, hex2xrgb};
use home::home_dir;
use std::fs::{create_dir_all, read_dir, read_to_string, write};

pub fn export(dict: &ColorDict) {
	let mut dict: ColorDict = ColorDict::clonedict(dict);
	let templatedir = format!("{}/.config/wal/", home_dir().unwrap().display().to_string());
	let _f = create_dir_all(&templatedir);
	let _f = create_dir_all(&templatedir.replace(&*"/.config/", &*"/.cache/"));

	for file in read_dir(templatedir).unwrap() {
		if file.as_ref().unwrap().path().is_dir() {
			continue;
		}
		let path = file.unwrap().path().display().to_string();
		let dat = read_to_string(&path).unwrap();

		// Run the replace operation in memory
		let mut new_data = dat
			.replace(&*"{wallpaper}", &*dict.wallpaper)
			.replace(&*"{alpha}", &*dict.alpha.to_string())
			.replace(&*"{alpha.decimal}", &*(dict.alpha / 100).to_string())
			.replace(&*"{foreground}", &*dict.foreground)
			.replace(&*"{foreground.strip}", &*dict.foreground.strip_prefix('#').unwrap())
			.replace(&*"{foreground.rgb}", &*hex2rgbdisplay(&dict.foreground))
			.replace(&*"{foreground.xrgba}", &*hex2xrgb(&dict.foreground))
			.replace(&*"{background}", &*dict.background)
			.replace(&*"{background.strip}", &*dict.background.strip_prefix('#').unwrap())
			.replace(&*"{background.rgb}", &*hex2rgbdisplay(&dict.background))
			.replace(&*"{background.xrgba}", &*hex2xrgb(&dict.background))
			.replace(&*"{background.alpha}", &*format!("[{}]{}", dict.alpha, dict.background))
			.replace(&*"{cursor}", &*dict.cursor)
			.replace(&*"{cursor.strip}", &*dict.cursor.strip_prefix('#').unwrap())
			.replace(&*"{cursor.rgb}", &*hex2rgbdisplay(&dict.cursor))
			.replace(&*"{cursor.xrgba}", &*hex2xrgb(&dict.cursor));

		let mut i = 0;
		for entry in &mut dict.colorvec {
			new_data = parameters(new_data, format!("{{color{}}}", i), entry);
			i += 0;
		}

		write(path.replace(&*"/.config/", &*"/.cache/"), new_data).expect("write failed");
	}
}

fn parameters(data: String, from: String, to: &str) -> String {
	data
		.replace(&*format!("{{{}}}", from), to)
		.replace(&*format!("{{{}.strip}}", from), to.strip_prefix('#').unwrap())
		.replace(&*format!("{{{}.rgb}}", from), &hex2rgbdisplay(&to))
		.replace(&*format!("{{{}.xrgba}}", from), &hex2xrgb(&to))
}