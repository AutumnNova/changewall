mod traitdef;
use super::colordict::to_array;
use super::colordict::ColorDict;
use super::convert::{blend_color, darken_color, darken_color_checked, rgb2hex};
//use aho_corasick::{AhoCorasickBuilder, MatchKind};
use anyhow::Result;
use magick_rust::MagickWand;
use magick_rust::ToMagick;
use magick_rust::bindings;
use palette::Srgb;
use std::{path::{Path, PathBuf}, process::exit};
use traitdef::MagickGen;

pub fn gen_colors(file: &Path) -> Result<Vec<Srgb<u8>>> {
	//const PATTERN: &[&str; 2] = &["(", ")"];

	let mut temp = Vec::with_capacity(16);
	let mut i = 0;

	//let ac = AhoCorasickBuilder::new()
	//	.match_kind(MatchKind::LeftmostFirst)
	//	.auto_configure(PATTERN)
	//	.build(PATTERN);

	let wand = MagickWand::new();
	wand.read_image(file.to_str().unwrap())?;
	wand.resize_image((wand.get_image_width() as f32 * 0.25) as usize , (wand.get_image_height() as f32 * 0.25) as usize, bindings::FilterType_LanczosFilter);
	let blob = wand.write_image_blob("MIFF").unwrap();

	while i <= 10 {
		//temp.imagemagick(file, 16 + i, &ac)?;
		wand.quantize_image(16 + i, wand.get_colorspace(), 3, bindings::DitherMethod_RiemersmaDitherMethod, false.to_magick())?;
		wand.unique_image_colors()?;
		if wand.get_image_colors() >= 16 {
		//if temp.len() >= 16 {
			break;
		}
		wand.read_image_blob(blob.clone())?;
		//temp.clear();
		i += 1;
	}
	temp.formatpix(wand.get_image_histogram().unwrap())?;

	if i == 11 {
		println!("Could not generate palette for {} within 10 attemps, Exiting", file.to_str().unwrap());
		exit(0)
	}
	Ok(temp)
}

pub fn adjust(colors: Vec<Srgb<u8>>) -> Vec<Srgb<u8>> {
	let mut temp = Vec::with_capacity(16);
	for (i, mut rgb) in colors.into_iter().enumerate() {
		match i {
			// vec is inverted so 0=15, 1=14 and so on
			0 | 8 => rgb = blend_color(rgb, Srgb::from_components((238, 238, 238))),
			7 => rgb = darken_color(rgb, 0.30),
			15 => rgb = darken_color_checked(rgb, 0.40),
			_ => (),
		}
		temp.push(rgb);
	}
	temp
}

pub fn format(colors: Vec<Srgb<u8>>, wallpaper: PathBuf, alpha: u8) -> ColorDict {
	let mut colorvec = Vec::with_capacity(16);
	for (i, col) in colors.into_iter().enumerate() {
		if i < 8 || i == 15 {
			colorvec.insert(0, rgb2hex(col));
		}
	}
	colorvec.append(&mut colorvec.clone());
	colorvec.remove(9);
	colorvec.pop().unwrap();
	ColorDict::new(wallpaper, alpha, to_array(colorvec))
}
