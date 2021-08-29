pub mod colordict;
use colordict::ColorDict;
use hex::{decode, encode};
use std::process::Command;

pub fn colors(file: String, style: bool, alpha: usize) -> ColorDict {
	format(adjust(gen_colors(&file)), file.to_string(), style, alpha)
}

fn gen_colors(file: &str) -> Vec<String> {
	let mut temp = Vec::new();
	let mut i = 0;

	while i <= 20 {
		let raw_col = imagemagick(file, 16 + i);

		let output = String::from_utf8_lossy(&raw_col).to_string();
		for line in output.lines().skip(1) {
			temp.insert(0, line.split("  ").nth(1).unwrap().to_string());
		}

		if temp.len() >= 16 {
			break;
		}
		temp.clear();
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

pub fn hex2rgb(hex: &str) -> Vec<u8> {
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
	let mut dict: ColorDict = ColorDict::new();
	dict.wallpaper = file;
	dict.alpha = alpha;
	if !style {
		dict.colorvec = vechack(colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string());
	} else {
		dict.colorvec = vec![colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string(), colors.pop().unwrap().to_string()]
	}
	dict.background.clone_from(&dict.colorvec.clone().into_iter().nth(0).unwrap());
	dict.foreground.clone_from(&dict.colorvec.clone().into_iter().nth(15).unwrap());
	dict.cursor.clone_from(&dict.foreground);
	dict
}
#[allow(unused_assignments)]
fn vechack(color0: String, mut color1: String, mut color2: String, mut color3:String, mut color4:String, mut color5:String, mut color6:String, mut color7:String, mut color8:String, mut color9:String, mut color10:String, mut color11:String, mut color12:String, mut color13:String, mut color14:String, mut color15: String) -> Vec<String> {
	color1 = color8.clone();
	color2 = color9.clone();
	color3 = color10.clone();
	color4 = color11.clone();
	color5 = color12.clone();
	color6 = color13.clone();
	color7 = color14.clone();
	color8 = color15.clone();
	color9 = color1.clone();
	color10 = color2.clone();
	color11 = color3.clone();
	color12 = color4.clone();
	color13 = color5.clone();
	color14 = color6.clone();
	color15 = color7.clone();
	vec![color0, color1, color2, color3, color4, color5, color6, color7, color8, color9, color10, color11, color12, color13, color14, color15]}

mod tests {
	#[allow(unused_imports)]
	use super::{
		blend_color, darken_color, darken_color_checked, hex2rgb, hex2rgbdisplay, hex2xrgb, rgb2hex,
	};

	#[test]
	fn test_rgb2hex() {
		assert_eq!(rgb2hex(vec![255, 255, 255]), "#ffffff");
		assert_eq!(rgb2hex(vec![127, 127, 127]), "#7f7f7f");
		assert_eq!(rgb2hex(vec![0, 0, 0]), "#000000");
	}

	#[test]
	fn test_hex2rgb() {
		assert_eq!(hex2rgb("#FFFFFF"), vec![255, 255, 255]);
		assert_eq!(hex2rgb("#7F7F7F"), vec![127, 127, 127]);
		assert_eq!(hex2rgb("#000000"), vec![0, 0, 0]);
	}

	#[test]
	fn test_hex2xrgb() {
		assert_eq!(hex2xrgb("#FFFFFF"), "255/255/255/ff");
		assert_eq!(hex2xrgb("#7F7F7F"), "127/127/127/ff");
		assert_eq!(hex2xrgb("#000000"), "0/0/0/ff");
	}

	#[test]
	fn test_hex2rgbdisplay() {
		assert_eq!(hex2rgbdisplay("#FFFFFF"), "255,255,255");
		assert_eq!(hex2rgbdisplay("#7F7F7F"), "127,127,127");
		assert_eq!(hex2rgbdisplay("#000000"), "0,0,0");
	}

	#[test]
	fn test_darkencolor() {
		assert_eq!(darken_color(vec![255, 255, 255], 0.0), [255, 255, 255]);
		assert_eq!(darken_color(vec![255, 255, 255], 0.5), [127, 127, 127]);
		assert_eq!(darken_color(vec![127, 127, 127], 1.0), [0, 0, 0]);
	}

	#[test]
	fn test_darkencolorchecked() {
		assert_eq!(darken_color_checked(vec![255, 255, 255], 0.0), [255, 255, 255]);
		assert_eq!(darken_color_checked(vec![127, 127, 127], 1.0), [0, 0, 0]);
		assert_eq!(darken_color_checked(vec![10, 10, 10], 0.5), [10, 10, 10]);
	}

	#[test]
	fn test_blendcolor() {
		assert_eq!(blend_color(vec![255, 255, 255], vec![255, 255, 255]), [255, 255, 255]);
		assert_eq!(blend_color(vec![255, 255, 255], vec![0, 0, 0]), [127, 127, 127]);
		assert_eq!(blend_color(vec![0, 0, 0], vec![255, 255, 255]), [127, 127, 127]);
	}
}
