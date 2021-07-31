use crate::colors::{hex2rgbdisplay, hex2xrgb, ColorDict};
use home::home_dir;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::{create_dir_all, read_dir, read_to_string, write};

pub fn export(dict: &ColorDict) {
	lazy_static! {
		static ref RE: Regex = Regex::new("templates/([a-zA-Z.-]+)").unwrap();
	};

	let templatedir = format!("{}/.config/wal/templates/", home_dir().unwrap().display().to_string());
	let _f = create_dir_all(&templatedir);
	let cachedir = format!("{}/.cache/wal", home_dir().unwrap().display().to_string());
	let _f = create_dir_all(&cachedir);

	for file in read_dir(templatedir).unwrap() {
		let path = file.unwrap().path().display().to_string();
		let dat = read_to_string(&path).unwrap();

		// {background.alpha} = [100]#03080D
		// Run the replace operation in memory
		let new_data = dat
			.replace(&*"{wallpaper}", &*dict.wallpaper)
			.replace(&*"{foreground}", &*dict.foreground)
			.replace(&*"{foreground.strip}", &*dict.foreground.strip_prefix('#').unwrap())
			.replace(&*"{foreground.rgb}", &*hex2rgbdisplay(&dict.foreground))
			.replace(&*"{foreground.xrgba}", &*hex2xrgb(&dict.foreground))
			.replace(&*"{background}", &*dict.background)
			.replace(&*"{background.strip}", &*dict.background.strip_prefix('#').unwrap())
			.replace(&*"{background.rgb}", &*hex2rgbdisplay(&dict.background))
			.replace(&*"{background.xrgba}", &*hex2xrgb(&dict.background))
			.replace(&*"{cursor}", &*dict.cursor)
			.replace(&*"{cursor.strip}", &*dict.cursor.strip_prefix('#').unwrap())
			.replace(&*"{cursor.rgb}", &*hex2rgbdisplay(&dict.cursor))
			.replace(&*"{cursor.xrgba}", &*hex2xrgb(&dict.cursor))
			.replace(&*"{color0}", &*dict.color0)
			.replace(&*"{color0.strip}", &*dict.color0.strip_prefix('#').unwrap())
			.replace(&*"{color0.rgb}", &*hex2rgbdisplay(&dict.color0))
			.replace(&*"{color0.xrgba}", &*hex2xrgb(&dict.color0))
			.replace(&*"{color1}", &*dict.color1)
			.replace(&*"{color1.strip}", &*dict.color1.strip_prefix('#').unwrap())
			.replace(&*"{color1.rgb}", &*hex2rgbdisplay(&dict.color1))
			.replace(&*"{color1.xrgba}", &*hex2xrgb(&dict.color1))
			.replace(&*"{color2}", &*dict.color2)
			.replace(&*"{color2.strip}", &*dict.color2.strip_prefix('#').unwrap())
			.replace(&*"{color2.rgb}", &*hex2rgbdisplay(&dict.color2))
			.replace(&*"{color2.xrgba}", &*hex2xrgb(&dict.color2))
			.replace(&*"{color3}", &*dict.color3)
			.replace(&*"{color3.strip}", &*dict.color3.strip_prefix('#').unwrap())
			.replace(&*"{color3.rgb}", &*hex2rgbdisplay(&dict.color3))
			.replace(&*"{color3.xrgba}", &*hex2xrgb(&dict.color3))
			.replace(&*"{color4}", &*dict.color4)
			.replace(&*"{color4.strip}", &*dict.color4.strip_prefix('#').unwrap())
			.replace(&*"{color4.rgb}", &*hex2rgbdisplay(&dict.color4))
			.replace(&*"{color4.xrgba}", &*hex2xrgb(&dict.color4))
			.replace(&*"{color5}", &*dict.color5)
			.replace(&*"{color5.strip}", &*dict.color5.strip_prefix('#').unwrap())
			.replace(&*"{color5.rgb}", &*hex2rgbdisplay(&dict.color5))
			.replace(&*"{color5.xrgba}", &*hex2xrgb(&dict.color5))
			.replace(&*"{color6}", &*dict.color6)
			.replace(&*"{color6.strip}", &*dict.color6.strip_prefix('#').unwrap())
			.replace(&*"{color6.rgb}", &*hex2rgbdisplay(&dict.color6))
			.replace(&*"{color6.xrgba}", &*hex2xrgb(&dict.color6))
			.replace(&*"{color7}", &*dict.color7)
			.replace(&*"{color7.strip}", &*dict.color7.strip_prefix('#').unwrap())
			.replace(&*"{color7.rgb}", &*hex2rgbdisplay(&dict.color7))
			.replace(&*"{color7.xrgba}", &*hex2xrgb(&dict.color7))
			.replace(&*"{color8}", &*dict.color8)
			.replace(&*"{color8.strip}", &*dict.color8.strip_prefix('#').unwrap())
			.replace(&*"{color8.rgb}", &*hex2rgbdisplay(&dict.color8))
			.replace(&*"{color8.xrgba}", &*hex2xrgb(&dict.color8))
			.replace(&*"{color9}", &*dict.color9)
			.replace(&*"{color9.strip}", &*dict.color9.strip_prefix('#').unwrap())
			.replace(&*"{color9.rgb}", &*hex2rgbdisplay(&dict.color9))
			.replace(&*"{color9.xrgba}", &*hex2xrgb(&dict.color9))
			.replace(&*"{color10}", &*dict.color10)
			.replace(&*"{color10.strip}", &*dict.color10.strip_prefix('#').unwrap())
			.replace(&*"{color10.rgb}", &*hex2rgbdisplay(&dict.color10))
			.replace(&*"{color10.xrgba}", &*hex2xrgb(&dict.color10))
			.replace(&*"{color11}", &*dict.color11)
			.replace(&*"{color11.strip}", &*dict.color11.strip_prefix('#').unwrap())
			.replace(&*"{color11.rgb}", &*hex2rgbdisplay(&dict.color11))
			.replace(&*"{color11.xrgba}", &*hex2xrgb(&dict.color11))
			.replace(&*"{color12}", &*dict.color12)
			.replace(&*"{color12.strip}", &*dict.color12.strip_prefix('#').unwrap())
			.replace(&*"{color12.rgb}", &*hex2rgbdisplay(&dict.color12))
			.replace(&*"{color12.xrgba}", &*hex2xrgb(&dict.color12))
			.replace(&*"{color13}", &*dict.color13)
			.replace(&*"{color13.strip}", &*dict.color13.strip_prefix('#').unwrap())
			.replace(&*"{color13.rgb}", &*hex2rgbdisplay(&dict.color13))
			.replace(&*"{color13.xrgba}", &*hex2xrgb(&dict.color13))
			.replace(&*"{color14}", &*dict.color14)
			.replace(&*"{color14.strip}", &*dict.color14.strip_prefix('#').unwrap())
			.replace(&*"{color14.rgb}", &*hex2rgbdisplay(&dict.color14))
			.replace(&*"{color14.xrgba}", &*hex2xrgb(&dict.color14))
			.replace(&*"{color15}", &*dict.color15)
			.replace(&*"{color15.strip}", &*dict.color15.strip_prefix('#').unwrap())
			.replace(&*"{color15.rgb}", &*hex2rgbdisplay(&dict.color15))
			.replace(&*"{color15.xrgba}", &*hex2xrgb(&dict.color15));

		let mut newpath = "".to_string();

		for dir in RE.captures(&path) {
			newpath.push_str(&format!("{}/{}", &cachedir, &dir[1]));
		}

		write(newpath, new_data).expect("write failed");
	}
}
