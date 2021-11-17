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
	let mut buf = ryu::Buffer::new();
	let col = hex2rgb(hex);
	let mut temp = String::with_capacity(11);
	temp.push_str(buf.format_finite(col.red * 255.0));
	temp.push(',');
	temp.push_str(buf.format_finite(col.green * 255.0));
	temp.push(',');
	temp.push_str(buf.format_finite(col.blue * 255.0));
	temp
}

pub fn hex2xrgb(hex: &str) -> String {
	let mut buf = ryu::Buffer::new();
	let col = hex2rgb(hex);
	let mut temp = String::with_capacity(14);
	temp.push_str(buf.format_finite(col.red * 255.0));
	temp.push('/');
	temp.push_str(buf.format_finite(col.green * 255.0));
	temp.push('/');
	temp.push_str(buf.format_finite(col.blue * 255.0));
	temp.push_str("/ff");
	temp
}

pub fn rgb2hex(rgb: Rgb) -> String {
	let (r, g, b) = rgb.into_components();
	let color_u8: Vec<u8> = vec![(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8];
	let color_hex = encode(color_u8);
	let mut output = String::with_capacity(7);
	output.push('#');
	output.push_str(&color_hex);
	output
}

#[allow(dead_code)]
pub fn rgb2yiq(rgb: Rgb) -> Vec<i64> {
	let (red, green, blue) = rgb.into_components();
	let (red, green, blue) = (red / 255.0, green / 255.0, blue / 255.0);
	let y = 0.30 * red + 0.59 * green + 0.11 * blue;
	let i = 0.74 * (red - y) - 0.27 * (blue - y);
	let q = 0.48 * (red - y) + 0.41 * (blue - y);
	vec![(q * 100.0) as i64, (i * 100.0) as i64, (y * 100.0) as i64]
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
