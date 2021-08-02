use hex::{decode, encode};
use std::process::Command;

pub struct ColorDict {
	pub wallpaper: String,
	pub alpha: usize,
	pub background: String,
	pub foreground: String,
	pub cursor: String,
	pub color0: String,
	pub color1: String,
	pub color2: String,
	pub color3: String,
	pub color4: String,
	pub color5: String,
	pub color6: String,
	pub color7: String,
	pub color8: String,
	pub color9: String,
	pub color10: String,
	pub color11: String,
	pub color12: String,
	pub color13: String,
	pub color14: String,
	pub color15: String,
}

fn gen_colors(file: &str) -> Vec<String> {

	let mut temp = Vec::new();
	let mut i = 0;

	while i <= 20 {
		let raw_col = imagemagick(file, 16 + i);
		temp.clear();

		let asd = String::from_utf8_lossy(&raw_col).to_string();
		for line in asd.lines().skip(1) {
			temp.insert(0, line.split("  srgb(").next().unwrap().rsplit(")  ").next().unwrap().to_string());
		}

		if temp.len() >= 16 {
			break;
		}
		i += 1;
	}
	temp
}

fn adjust(colors: Vec<String>) -> Vec<String> {
	let mut temp = Vec::new();
	let mut i = 0;

	for hex in &colors {
		let mut rgb = hex2rgb(&hex);
		match i {
			// vec is inverted so 0=15, 1=14 and so on
			0 => rgb = blend_color(rgb, vec![238, 238, 238]),
			7 => rgb = darken_color(rgb, 0.30),
			8 => rgb = blend_color(rgb, vec![238, 238, 238]),
			15 => rgb = darken_color_checked(rgb, 0.40),
			_ => (),
		}
		let hex = rgb2hex(rgb);
		temp.push(hex);
		i += 1;
	}
	temp
}

fn hex2rgb(hex: &str) -> Vec<u8> {
	let split1 = hex.strip_prefix('#').unwrap().split_at(2);
	let split2 = split1.1.split_at(2);
	vec![decode(split2.1).unwrap().pop().unwrap(), decode(split2.0).unwrap().pop().unwrap(), decode(split1.0).unwrap().pop().unwrap()]
}

pub fn hex2rgbdisplay(hex: &str) -> String {
	let mut vec = hex2rgb(hex);
	format!("{},{},{}", vec.pop().unwrap(), vec.pop().unwrap(), vec.pop().unwrap())
}

pub fn hex2xrgb(hex: &str) -> String {
	let mut rgb = hex2rgb(hex);
	format!("{}/{}/{}/ff", &rgb.pop().unwrap(), &rgb.pop().unwrap(), &rgb.pop().unwrap())
}

fn rgb2hex(mut rgb: Vec<u8>) -> String {
	format!("#{}", encode(vec![rgb.pop().unwrap(), rgb.pop().unwrap(), rgb.pop().unwrap()]))
}

fn darken_color(mut rgb: Vec<u8>, amp: f64) -> Vec<u8> {
	let r = rgb.pop().unwrap() as f64 * (1f64 - amp);
	let g = rgb.pop().unwrap() as f64 * (1f64 - amp);
	let b = rgb.pop().unwrap() as f64 * (1f64 - amp);
	let mut vec: Vec<u8> = Vec::new();
	vec.push(b as u8);
	vec.push(g as u8);
	vec.push(r as u8);
	vec
}

fn blend_color(mut rgb1: Vec<u8>, mut rgb2: Vec<u8>) -> Vec<u8> {
	let r = 0.5f64 * rgb1.pop().unwrap() as f64 + 0.5f64 * rgb2.pop().unwrap() as f64;
	let g = 0.5f64 * rgb1.pop().unwrap() as f64 + 0.5f64 * rgb2.pop().unwrap() as f64;
	let b = 0.5f64 * rgb1.pop().unwrap() as f64 + 0.5f64 * rgb2.pop().unwrap() as f64;
	let mut vec: Vec<u8> = Vec::new();
	vec.push(b as u8);
	vec.push(g as u8);
	vec.push(r as u8);
	vec
}

fn darken_color_checked(mut rgb: Vec<u8>, amp: f64) -> Vec<u8> {
	let r = rgb.pop().unwrap() as f64;
	rgb.push(r as u8);
	if r < 16f64 {
		rgb
	} else {
		darken_color(rgb, amp)
	}
}

fn imagemagick(file: &str, quant: i32) -> Vec<u8> {
	let output = Command::new("magick")
		.args([&file, "-resize", "25%", "-colors", &quant.to_string(), "-unique-colors", "txt:-"])
		.output()
		.expect("failed to gather colors");

	output.stdout
}

fn format(mut colors: Vec<String>, file: String, style: bool, alpha: usize) -> ColorDict {
	let mut dict: ColorDict = ColorDict {
		wallpaper: file,
		alpha,
		background: "".to_string(),
		foreground: "".to_string(),
		cursor: "".to_string(),
		color0: colors.pop().unwrap().to_string(),
		color1: colors.pop().unwrap().to_string(),
		color2: colors.pop().unwrap().to_string(),
		color3: colors.pop().unwrap().to_string(),
		color4: colors.pop().unwrap().to_string(),
		color5: colors.pop().unwrap().to_string(),
		color6: colors.pop().unwrap().to_string(),
		color7: colors.pop().unwrap().to_string(),
		color8: colors.pop().unwrap().to_string(),
		color9: colors.pop().unwrap().to_string(),
		color10: colors.pop().unwrap().to_string(),
		color11: colors.pop().unwrap().to_string(),
		color12: colors.pop().unwrap().to_string(),
		color13: colors.pop().unwrap().to_string(),
		color14: colors.pop().unwrap().to_string(),
		color15: colors.pop().unwrap().to_string(),
	};

	dict.background.clone_from(&dict.color0);
	dict.foreground.clone_from(&dict.color15);
	dict.cursor.clone_from(&dict.color15);
	if !style {
		dict.color1.clone_from(&dict.color8);
		dict.color2.clone_from(&dict.color9);
		dict.color3.clone_from(&dict.color10);
		dict.color4.clone_from(&dict.color11);
		dict.color5.clone_from(&dict.color12);
		dict.color6.clone_from(&dict.color13);
		dict.color7.clone_from(&dict.color14);
		dict.color8.clone_from(&dict.color15);
		dict.color9.clone_from(&dict.color1);
		dict.color10.clone_from(&dict.color2);
		dict.color11.clone_from(&dict.color3);
		dict.color12.clone_from(&dict.color4);
		dict.color13.clone_from(&dict.color5);
		dict.color14.clone_from(&dict.color6);
		dict.color15.clone_from(&dict.color7);
	}
	dict
}

pub fn colors(file: String, style: bool, alpha: usize) -> ColorDict {
	format(adjust(gen_colors(&file)), file.to_string(), style, alpha)
}
