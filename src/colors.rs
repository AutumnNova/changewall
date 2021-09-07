pub mod colordict;
pub mod convert;
use color_thief::{get_palette, Color, ColorFormat};
use colordict::ColorDict;
use convert::{blend_color, darken_color, darken_color_checked, rgb2hex, rgb2yiq};
use image::{ColorType, GenericImageView, imageops::FilterType, open};
use palette::rgb::Rgb;
use std::{path::Path, process::{Command, exit}};

pub fn colors(file: String, style: bool, alpha: usize) -> ColorDict {
	let uselegacy = true;
	format(adjust(gen_colors(&file, uselegacy), uselegacy), file.to_string(), style, uselegacy, alpha)
}

fn gen_colors(file: &str, uselegacy: bool) -> Vec<Rgb> {
	let mut temp = Vec::new();
	let mut i = 0;

	if uselegacy {
		while i <= 10 {
			let raw_col = imagemagick(file, 16 + i);
			for line in raw_col.lines().skip(1) {
				let tmp = line.replace('(', "").replace(')', "").split(' ').nth(1).unwrap().to_string();
				let mut tmp2 = tmp.split(',');
				let color: Rgb = Rgb::new(
					tmp2.next().unwrap().to_string().parse::<f32>().unwrap() / 255.0,
					tmp2.next().unwrap().to_string().parse::<f32>().unwrap() / 255.0,
					tmp2.next().unwrap().to_string().parse::<f32>().unwrap() / 255.0,
				);
				temp.insert(0, color);
			}

			if temp.len() >= 16 {
				break;
			}

			temp.clear();
			i += 1;
		}
	} else {
		while i <= 10 {
			let raw = colorthief(file, 8 + i);
			for line in raw.into_iter() {
				let color: Rgb = Rgb::new(
					line.r as f32 / 255.0,
					line.g as f32 / 255.0,
					line.b as f32 / 255.0,
				);
				temp.insert(0, color);
			}

			if temp.len() >= 8 {
				temp.sort_by_cached_key(|k| rgb2yiq(*k));
				temp.append(&mut temp.to_vec());
				break;
			}

			temp.clear();
			i += 1;
		}
	}

	if i == 11 {
		println!("Could not generate palette for {} within 10 attemps, Exiting", file);
		exit(0)
	}
	temp
}

fn adjust(colors: Vec<Rgb>, uselegacy: bool) -> Vec<Rgb> {
	let mut temp = Vec::new();
	let mut i = 0;
	if uselegacy {
		for mut rgb in colors {
			match i {
				// vec is inverted so 0=15, 1=14 and so on
				0 => rgb = blend_color(rgb, Rgb::from_components((238.0/255.0, 238.0/255.0, 238.0/255.0))),
				7 => rgb = darken_color(rgb, 0.30),
				8 => rgb = blend_color(rgb, Rgb::from_components((238.0/255.0, 238.0/255.0, 238.0/255.0))),
				15 => rgb = darken_color_checked(rgb, 0.40),
				_ => (),
			}
			temp.push(rgb);
			i += 1;
		}
	} else {
		for rgb in colors {
			match i {
				// vec is inverted so 0=15, 1=14 and so on
				_ => (),
			}
			temp.push(rgb);
			i += 1;
		}
	}
	temp
}

fn imagemagick(file: &str, quant: u8) -> String {
	let output = Command::new("magick")
		.args([file, "-resize", "25%", "-colors", &quant.to_string(), "-unique-colors", "txt:-"])
		.output()
		.expect("failed to gather colors");

	String::from_utf8_lossy(&output.stdout).to_string()
}

fn colorthief(file: &str, quant: u8) -> Vec<Color> {
	let img_path = open(&Path::new(file)).unwrap();
	let dim = img_path.dimensions();
	let (dim0, dim1) = ((dim.0 * 25) / 100, (dim.1 * 25) / 100);
	get_palette(img_path.resize(dim0, dim1, FilterType::Triangle).as_bytes(), find_color(img_path.color()), 1, quant).unwrap()
}

fn find_color(t: ColorType) -> ColorFormat {
	match t {
		ColorType::Rgb8 => ColorFormat::Rgb,
		ColorType::Rgba8 => ColorFormat::Rgba,
		ColorType::Rgb16 => ColorFormat::Rgb,
		ColorType::Rgba16 => ColorFormat::Rgba,
		ColorType::Bgr8 => ColorFormat::Bgr,
		ColorType::Bgra8 => ColorFormat::Bgra,
		ColorType::L8 => panic!(),
		ColorType::La8 => panic!(),
		ColorType::L16 => panic!(),
		ColorType::La16 => panic!(),
		_ => unreachable!(),
	}
}

fn format(colors: Vec<Rgb>, wallpaper: String, style: bool, uselegacy: bool, alpha: usize) -> ColorDict {
	let mut temp = Vec::new();
	if uselegacy && !style {
		let mut i = 15;
		for col in colors.into_iter() {
			if i > 7 || i == 0 {
				temp.insert(0, rgb2hex(col));
			}
			i -= 1;
		}
		temp.append(&mut temp.to_vec());
		temp.remove(9);
		temp.pop().unwrap();
	} else {
		for color in colors.into_iter() {
			temp.insert(0, rgb2hex(color));
		}
	}
	ColorDict { wallpaper, alpha, background: temp.to_vec().into_iter().next().unwrap(), foreground: temp.to_vec().into_iter().nth(15).unwrap(), cursor: temp.to_vec().into_iter().nth(15).unwrap(), colorvec: temp }
}
