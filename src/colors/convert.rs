use hex::{decode, encode};
use palette::rgb::Rgb;

pub fn hex2rgb(hex: &str) -> Rgb {
	let split1 = hex.strip_prefix('#').unwrap().split_at(2);
	let split2 = split1.1.split_at(2);
	Rgb::from_components((
		decode(split1.0).unwrap().pop().unwrap() as f32 / 255.0,
		decode(split2.0).unwrap().pop().unwrap() as f32 / 255.0,
		decode(split2.1).unwrap().pop().unwrap() as f32 / 255.0,
	))
}

pub fn hex2rgbdisplay(hex: &str) -> String {
	let col = hex2rgb(hex);
	format!("{},{},{}", col.red * 255.0, col.green * 255.0, col.blue * 255.0)
}

pub fn hex2xrgb(hex: &str) -> String {
	let col = hex2rgb(hex);
	format!("{}/{}/{}/ff", col.red * 255.0, col.green * 255.0, col.blue * 255.0)
}

pub fn rgb2hex(rgb: Rgb) -> String {
	let (r, g, b) = rgb.into_components();
	let color_u8: Vec<u8> = vec![(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8];
	let color_hex = encode(color_u8);
	format!("#{}", color_hex)
}

#[allow(dead_code)]
pub fn rgb2yiq(rgb: Rgb) -> Vec<i16> {
	let (r, g, b) = rgb.into_components();
	let (r, g, b) = (r / 255.0, g / 255.0, b / 255.0);
	let y = 0.30 * r + 0.59 * g + 0.11 * b;
	let i = 0.74 * (r - y) - 0.27 * (b - y);
	let q = 0.48 * (r - y) + 0.41 * (b - y);
	vec![q as i16, i as i16, y as i16]
}

#[allow(dead_code)]
pub fn darken_color(rgb: Rgb, amp: f32) -> Rgb {
	let (r, g, b) = rgb.into_components();
	Rgb::from_components((r * (1.0 - amp), g * (1.0 - amp), b * (1.0 - amp)))
}

#[allow(dead_code)]
pub fn blend_color(rgb1: Rgb, rgb2: Rgb) -> Rgb {
	let (r1, g1, b1) = rgb1.into_components();
	let (r2, g2, b2) = rgb2.into_components();
	Rgb::from_components((0.5 * r1 + 0.5 * r2, 0.5 * g1 + 0.5 * g2, 0.5 * b1 + 0.5 * b2))
}

#[allow(dead_code)]
pub fn darken_color_checked(rgb: Rgb, amp: f32) -> Rgb {
	if (rgb.red * 255.0) < 16.0 {
		rgb
	} else {
		darken_color(rgb, amp)
	}
}
