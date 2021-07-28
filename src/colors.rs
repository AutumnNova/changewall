use lazy_static::lazy_static;
use regex::Regex;
use std::process::Command;

#[derive(Debug)]
pub struct ColorDict {
	pub wallpaper: String,
	pub alpha: i32,
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
	pub color15: String
}

fn get(file:&String) -> String {
let colors = gen_colors(file.to_string());
colors
}

fn gen_colors(file:String) -> String {
	let raw_col = imagemagick(file, 16);

	lazy_static!{
		static ref RE:Regex = Regex::new("(#[A-F0-9]{6})").unwrap();
	};

	let mut temp:String = "".to_string();

	for color in RE.captures_iter(&String::from_utf8_lossy(&raw_col).to_string()) {
		temp.push_str(&color[0]);
		temp.push('&');
	}
	temp
}

fn adjust(colors:String) -> String{

	let mut  temp = "".to_string();
	let mut i = 0;

	for hex in colors.strip_suffix('&').unwrap().split('&') {
		let mut  rgb = hex2rgb(hex);
		if i == 0 {
			rgb = darken_color_checked(rgb, 0.40)
		} else if i == 7 {
			rgb = blend_color(rgb, vec![238, 238, 238])
		} else if i == 8 {
			rgb = darken_color(rgb, 0.30)
		} else if i == 15 {
			rgb = blend_color(rgb, vec![238, 238, 238])
		}
		let hex = rgb2hex(rgb);
		temp.push_str(&hex);
		temp.push('&');
		i += 1;
	}

colors
}

fn hex2rgb(hex:&str) -> Vec<u8> {

	lazy_static!{
		static ref RE:Regex = Regex::new("#([A-F0-9]{2})([A-F0-9]{2})([A-F0-9]{2})").unwrap();
	};

	let mut vec:Vec<u8> = Vec::new();

	for color in RE.captures(&hex) {
		let mut r = hex::decode(&color[1]).unwrap();
		let mut g = hex::decode(&color[2]).unwrap();
		let mut b = hex::decode(&color[3]).unwrap();
		vec.push(b.pop().unwrap());
		vec.push(g.pop().unwrap());
		vec.push(r.pop().unwrap());
	}	
vec
}

fn rgb2hex(mut rgb:Vec<u8>) -> String {
	let r = rgb.pop().unwrap();
	let g = rgb.pop().unwrap();
	let b = rgb.pop().unwrap();
	let hex = format!("#{}", hex::encode(vec![r, g, b]));
	hex
}

fn darken_color(mut rgb:Vec<u8>, amp:f64) -> Vec<u8> {
	let r = rgb.pop().unwrap() as f64 * (1f64 - amp);
	let g = rgb.pop().unwrap() as f64 * (1f64 - amp);
	let b = rgb.pop().unwrap() as f64 * (1f64 - amp);
	let mut vec:Vec<u8> = Vec::new();
	vec.push(b as u8);
	vec.push(g as u8);
	vec.push(r as u8);
	vec
}

fn blend_color(mut rgb1:Vec<u8>, mut rgb2:Vec<u8>) -> Vec<u8> {
	let r = 0.5f64 * rgb1.pop().unwrap() as f64 + 0.5f64 * rgb2.pop().unwrap() as f64;
	let g = 0.5f64 * rgb1.pop().unwrap() as f64 + 0.5f64 * rgb2.pop().unwrap() as f64;
	let b = 0.5f64 * rgb1.pop().unwrap() as f64 + 0.5f64 * rgb2.pop().unwrap() as f64;
	let mut vec:Vec<u8> = Vec::new();
	vec.push(b as u8);
	vec.push(g as u8);
	vec.push(r as u8);
	vec
}

fn darken_color_checked(mut rgb:Vec<u8>, amp:f64) -> Vec<u8> {
	let r = rgb.pop().unwrap() as f64;
	if r < 16f64 {
		rgb.push(r as u8);
		rgb
	}
	else {
		let r = r * (1f64 - amp);
		let g = rgb.pop().unwrap() as f64 * (1f64 - amp);
		let b = rgb.pop().unwrap() as f64 * (1f64 - amp);
	
		let mut vec:Vec<u8> = Vec::new();
		vec.push(b as u8);
		vec.push(g as u8);
		vec.push(r as u8);
		vec
		
	}
}


fn imagemagick(file:String, quant:i32) -> Vec<u8> {

	let output = Command::new("magick")
	.args([file, "-resize".to_string(), "25%".to_string(), "-colors".to_string(), quant.to_string(), "-unique-colors".to_string(), "txt:-".to_string()])
	.output()
	.expect("failed to gather colors");

	output.stdout
}

fn format(colors:String, file:String) -> ColorDict {
	let mut col = colors.split('&');
	let mut dict: ColorDict = ColorDict {
		wallpaper: file,
		alpha: 100,
		background: "".to_string(),
		foreground: "".to_string(),
		cursor: "".to_string(),
		color0: col.next().unwrap().to_string(),
		color1: col.next().unwrap().to_string(),
		color2: col.next().unwrap().to_string(),
		color3: col.next().unwrap().to_string(),
		color4: col.next().unwrap().to_string(),
		color5: col.next().unwrap().to_string(),
		color6: col.next().unwrap().to_string(),
		color7: col.next().unwrap().to_string(),
		color8: col.next().unwrap().to_string(),
		color9: col.next().unwrap().to_string(),
		color10: col.next().unwrap().to_string(),
		color11: col.next().unwrap().to_string(),
		color12: col.next().unwrap().to_string(),
		color13: col.next().unwrap().to_string(),
		color14: col.next().unwrap().to_string(),
		color15: col.next().unwrap().to_string()
	};
	dict.background.clone_from(&dict.color0);
	dict.foreground.clone_from(&dict.color15);
	dict.cursor.clone_from(&dict.color15);
	dict
}

pub fn colors(file:&String) -> ColorDict {
	let mut colors = get(&file);
	colors = adjust(colors);
	
	let dict = format(colors, file.to_string());
	dict
}