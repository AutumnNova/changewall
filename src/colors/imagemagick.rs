use super::colordict::ColorDict;
use super::convert::{blend_color, darken_color, darken_color_checked, rgb2hex};
use palette::rgb::Rgb;
use std::{path::Path, process::{exit, Command}};

pub fn gen_colors(file: &Path) -> Vec<Rgb> {
	let mut temp = Vec::new();
	let mut i = 0;
	while i <= 10 {
		let raw_col = imagemagick(file, 16 + i);
		for line in raw_col.lines().skip(1) {
			let tmp = line.replace('(', "").replace(')', "").split(' ').nth(1).unwrap().to_string();
			let mut tmp2 = tmp.split(',');
			let color: Rgb = Rgb::new(
				tmp2.next().unwrap().parse::<f32>().unwrap() / 255.0,
				tmp2.next().unwrap().parse::<f32>().unwrap() / 255.0,
				tmp2.next().unwrap().parse::<f32>().unwrap() / 255.0,
			);
			temp.insert(0, color);
		}

		if temp.len() >= 16 {
			break;
		}

		temp.clear();
		i += 1;
	}

	if i == 11 {
		println!("Could not generate palette for {} within 10 attemps, Exiting", file.to_str().unwrap());
		exit(0)
	}
	temp
}

pub fn adjust(colors: Vec<Rgb>) -> Vec<Rgb> {
	let mut temp = Vec::new();
	for (i, mut rgb) in colors.into_iter().enumerate() {
		match i {
			// vec is inverted so 0=15, 1=14 and so on
			0 => rgb = blend_color(rgb, Rgb::from_components((238.0/255.0, 238.0/255.0, 238.0/255.0))),
			7 => rgb = darken_color(rgb, 0.30),
			8 => rgb = blend_color(rgb, Rgb::from_components((238.0/255.0, 238.0/255.0, 238.0/255.0))),
			15 => rgb = darken_color_checked(rgb, 0.40),
			_ => (),
		}
		temp.push(rgb);
	}
	temp
}

pub fn format(colors: Vec<Rgb>, wallpaper: &Path, style: bool, alpha: usize) -> ColorDict {
	let mut temp = Vec::new();
	if !style {
		for (i, col) in colors.into_iter().enumerate() {
			if i < 8 || i == 15 {
				temp.insert(0, rgb2hex(col));
			}
		}
		temp.append(&mut temp.to_vec());
		temp.remove(9);
		temp.pop().unwrap();
	}
	ColorDict { wallpaper: wallpaper.to_path_buf(), alpha, background: temp.to_vec().into_iter().next().unwrap(), foreground: temp.to_vec().into_iter().nth(15).unwrap(), cursor: temp.to_vec().into_iter().nth(15).unwrap(), colorvec: temp }
}

fn imagemagick(file: &Path, quant: u8) -> String {
	let output = Command::new("magick")
		.args([file.to_str().unwrap(), "-resize", "25%", "-colors", &quant.to_string(), "-unique-colors", "txt:-"])
		.output()
		.expect("failed to gather colors");

	String::from_utf8_lossy(&output.stdout).to_string()
}
