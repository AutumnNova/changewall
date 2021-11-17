use super::colordict::ColorDict;
use super::convert::{rgb2hex, rgb2yiq};
use color_thief::{get_palette, Color, ColorFormat};
use image::{imageops::FilterType, open, ColorType, GenericImageView};
use palette::rgb::Rgb;
use std::{path::{Path, PathBuf}, process::exit};

pub fn gen_colors(file: &Path) -> Vec<Rgb> {
	let mut temp = Vec::new();
	let mut i = 0;
	while i <= 10 {
		let raw = colorthief(file, 8 + i);
		for line in raw.into_iter() {
			let color: Rgb =
				Rgb::new(line.r as f32 / 255.0, line.g as f32 / 255.0, line.b as f32 / 255.0);
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

	if i == 11 {
		println!("Could not generate palette for {} within 10 attemps, Exiting", file.to_str().unwrap());
		exit(0)
	}
	temp
}

pub fn adjust(colors: Vec<Rgb>) -> Vec<Rgb> {
	let mut temp = Vec::new();
	for rgb in colors {
		temp.push(rgb);
	}
	temp
}

pub fn format(colors: Vec<Rgb>, wallpaper: PathBuf, _style: bool, alpha: u8) -> ColorDict {
	let mut temp = Vec::new();
	for color in colors.into_iter() {
		temp.insert(0, rgb2hex(color));
	}
	ColorDict { wallpaper, alpha, background: temp.to_vec().into_iter().next().unwrap(), foreground: temp.to_vec().into_iter().nth(15).unwrap(), cursor: temp.to_vec().into_iter().nth(15).unwrap(), colorvec: temp }
}

fn colorthief(file: &Path, quant: u8) -> Vec<Color> {
	let img_path = open(file).unwrap();
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
