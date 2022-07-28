mod traitdef;
use super::colordict::to_array;
use super::colordict::ColorDict;
use super::convert::{blend_color, darken_color, darken_color_checked, rgb2hex};
use aho_corasick::{AhoCorasickBuilder, MatchKind};
use anyhow::Result;
use palette::rgb::Rgb;
use std::{path::{Path, PathBuf}, process::exit};
use traitdef::MagickGen;

pub fn gen_colors(file: &Path) -> Result<Vec<Rgb>> {
	const PATTERN: &[&str; 2] = &["(", ")"];

	let mut temp = Vec::with_capacity(16);
	let mut i = 0;

	let ac = AhoCorasickBuilder::new()
		.match_kind(MatchKind::LeftmostFirst)
		.auto_configure(PATTERN)
		.build(PATTERN);

	while i <= 10 {
		temp.imagemagick(file, 16 + i, &ac)?;

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
	Ok(temp)
}

pub fn adjust(colors: Vec<Rgb>) -> Vec<Rgb> {
	let mut temp = Vec::with_capacity(16);
	for (i, mut rgb) in colors.into_iter().enumerate() {
		match i {
			// vec is inverted so 0=15, 1=14 and so on
			0 | 8 => rgb = blend_color(rgb, Rgb::from_components((238.0/255.0, 238.0/255.0, 238.0/255.0))),
			7 => rgb = darken_color(rgb, 0.30),
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
	colorvec.append(&mut colorvec.clone());
	colorvec.remove(9);
	colorvec.pop().unwrap();
	ColorDict::new(wallpaper, alpha, to_array(colorvec))
}
