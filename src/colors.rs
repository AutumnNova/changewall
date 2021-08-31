pub mod colordict;
pub mod convert;
use colordict::ColorDict;
use convert::{blend_color, darken_color, darken_color_checked, hex2rgb, rgb2hex};
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

fn imagemagick(file: &str, quant: i32) -> Vec<u8> {
	let output = Command::new("magick")
		.args([&file, "-resize", "25%", "-colors", &quant.to_string(), "-unique-colors", "txt:-"])
		.output()
		.expect("failed to gather colors");

	output.stdout
}

fn format(mut colors: Vec<String>, wallpaper: String, style: bool, alpha: usize) -> ColorDict {
	let mut temp = Vec::new();
	let mut i = 15;
	if style {
		temp = vec![colors.pop().unwrap(), colors.pop().unwrap(), colors.pop().unwrap(), colors.pop().unwrap(), colors.pop().unwrap(), colors.pop().unwrap(), colors.pop().unwrap(), colors.pop().unwrap(), colors.pop().unwrap(), colors.pop().unwrap(), colors.pop().unwrap(), colors.pop().unwrap(), colors.pop().unwrap(), colors.pop().unwrap(), colors.pop().unwrap(), colors.pop().unwrap()]
	}
	else {
		for col in colors.into_iter() {
			if i == 0 {
				temp.insert(0, col);
			} else if i > 7 {
				temp.insert(0, col);
			}
			i -= 1;
		}
		temp.append(&mut temp.to_vec());
		temp.remove(9);
	}
	ColorDict { wallpaper, alpha, background: temp.to_vec().into_iter().next().unwrap(), foreground: temp.to_vec().into_iter().nth(15).unwrap(), cursor: temp.to_vec().into_iter().nth(15).unwrap(), colorvec: temp }
}
