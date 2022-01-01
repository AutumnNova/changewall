mod traitdef;
use super::colordict::to_array;
use super::colordict::ColorDict;
use super::convert::{blend_color, darken_color, darken_color_checked, rgb2hex};
use palette::rgb::Rgb;
use std::{path::{Path, PathBuf}, process::exit};
use traitdef::MagickGen;

pub fn gen_colors(file: &Path) -> Vec<Rgb> {
	let mut temp = Vec::with_capacity(16);
	let mut i = 0;
	while i <= 10 {
		temp.imagemagick(file, 16 + i);

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
	let mut temp = Vec::with_capacity(16);
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

pub fn format(colors: Vec<Rgb>, wallpaper: PathBuf, alpha: u8) -> ColorDict {
	let mut colorvec = Vec::with_capacity(16);

	for (i, col) in colors.into_iter().enumerate() {
		if i < 8 || i == 15 {
			colorvec.insert(0, rgb2hex(col));
		}
	}
	colorvec.append(&mut colorvec.to_vec());
	colorvec.remove(9);
	colorvec.pop().unwrap();
	ColorDict::new(wallpaper, alpha, to_array(colorvec))
}

/*fn graphicsmagick(file: &Path, quant: u8) -> Result<Vec<Rgb>> {
	let mut temp = Vec::with_capacity(16);
	graphicsmagick::initialize();
	let x =  MagickWand::new().read_image(&file.to_path_buf().to_string_lossy())?.get_image_width();
	let y =  MagickWand::new().read_image(&file.to_path_buf().to_string_lossy())?.get_image_height();

		let mut mw = MagickWand::new();
	let mw = mw.read_image(&file.to_path_buf().to_string_lossy())?.resize_image((x as f64 * 0.01) as u64, (y as f64 * 0.01) as u64, FilterTypes::MitchellFilter, 0.5)?.quantize_image(quant.into(), graphicsmagick::types::ColorspaceType::RGBColorspace, 1, 1, 1)?;

	let mut i = 0;
	while i != 16 {
		let pixel = mw.get_image_colormap_color(i)?;
		let r = pixel.get_red();
		let g = pixel.get_green();
		let b = pixel.get_blue();

		let color: Rgb = Rgb::new(r as f32, g as f32, b as f32);
		temp.insert(0, color);


		i += 1;
	}

	Ok(temp)
}*/
