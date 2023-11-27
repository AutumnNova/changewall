use std::{ops::Div, str::FromStr};
use anyhow::Result;
use hex::encode;
use palette::Srgb;

pub fn hex2rgbdisplay(hex: &str) -> Result<String> {
	let col = Srgb::from_str(hex)?;
	let mut temp = String::with_capacity(11);
	temp.push_str(&col.red.to_string());
	temp.push(',');
	temp.push_str(&col.green.to_string());
	temp.push(',');
	temp.push_str(&col.blue.to_string());
	Ok(temp)
}

pub fn hex2xrgb(hex: &str) -> Result<String> {
	let col = Srgb::from_str(hex)?;
	let mut temp = String::with_capacity(14);
	temp.push_str(&col.red.to_string());
	temp.push('/');
	temp.push_str(&col.green.to_string());
	temp.push('/');
	temp.push_str(&col.blue.to_string());
	temp.push_str("/ff");
	Ok(temp)
}

pub fn rgb2hex(rgb: Srgb<u8>) -> String {
	let (r, g, b) = rgb.into_components();
	let color_u8: Vec<u8> = vec![r, g, b];
	let color_hex = encode(color_u8);
	let mut output = String::with_capacity(7);
	output.push('#');
	output.push_str(&color_hex);
	output
}

#[cfg(feature = "colorthief")]
pub fn rgb2yiq(rgb: Rgb) -> Vec<i64> {
	let (red, green, blue) = rgb.into_components();
	let (red, green, blue) = (red / 255.0, green / 255.0, blue / 255.0);
	let y = 0.30 * red + 0.59 * green + 0.11 * blue;
	let i = 0.74 * (red - y) - 0.27 * (blue - y);
	let q = 0.48 * (red - y) + 0.41 * (blue - y);
	vec![(q * 100.0) as i64, (i * 100.0) as i64, (y * 100.0) as i64]
}

pub fn darken_color(rgb: Srgb<u8>, amp: f32) -> Srgb<u8> {
	let (r, g, b) = rgb.into_components();
	Srgb::from_components(((r as f32 * (1.0 - amp)) as u8, (g as f32 * (1.0 - amp)) as u8, (b as f32 * (1.0 - amp)) as u8))
}


pub fn blend_color(rgb1: Srgb<u8>, rgb2: Srgb<u8>) -> Srgb<u8> {
	let (r1, g1, b1) = rgb1.into_components();
	let (r2, g2, b2) = rgb2.into_components();
	Srgb::from_components((r1.div(2) + r2.div(2), g1.div(2) + g2.div(2), b1.div(2) + b2.div(2)))
}

pub fn darken_color_checked(rgb: Srgb<u8>, amp: f32) -> Srgb<u8> {
	if rgb.red < 16 {
		rgb
	} else {
		darken_color(rgb, amp)
	}
}
