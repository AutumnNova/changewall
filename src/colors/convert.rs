use hex::{decode, encode};

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

pub fn rgb2hex(mut rgb: Vec<u8>) -> String {
	format!("#{}", encode(vec![rgb.pop().unwrap(), rgb.pop().unwrap(), rgb.pop().unwrap()]))
}

pub fn darken_color(mut rgb: Vec<u8>, amp: f64) -> Vec<u8> {
	let r = rgb.pop().unwrap() as f64 * (1f64 - amp);
	let g = rgb.pop().unwrap() as f64 * (1f64 - amp);
	let b = rgb.pop().unwrap() as f64 * (1f64 - amp);
	let mut vec: Vec<u8> = Vec::new();
	vec.push(b as u8);
	vec.push(g as u8);
	vec.push(r as u8);
	vec
}

pub fn blend_color(mut rgb1: Vec<u8>, mut rgb2: Vec<u8>) -> Vec<u8> {
	let r = 0.5f64 * rgb1.pop().unwrap() as f64 + 0.5f64 * rgb2.pop().unwrap() as f64;
	let g = 0.5f64 * rgb1.pop().unwrap() as f64 + 0.5f64 * rgb2.pop().unwrap() as f64;
	let b = 0.5f64 * rgb1.pop().unwrap() as f64 + 0.5f64 * rgb2.pop().unwrap() as f64;
	let mut vec: Vec<u8> = Vec::new();
	vec.push(b as u8);
	vec.push(g as u8);
	vec.push(r as u8);
	vec
}

pub fn darken_color_checked(mut rgb: Vec<u8>, amp: f64) -> Vec<u8> {
	let r = rgb.pop().unwrap() as f64;
	rgb.push(r as u8);
	if r < 16f64 {
		rgb
	} else {
		darken_color(rgb, amp)
	}
}
