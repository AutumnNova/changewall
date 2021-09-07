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

pub fn rgb2yiq(mut rgb: Vec<u8>) -> Vec<i16> {
	let (r, g, b) = (rgb.pop().unwrap() as f64, rgb.pop().unwrap() as f64, rgb.pop().unwrap() as f64);
	let y = 0.30 * r + 0.59 * g + 0.11 * b;
	let i = 0.74 * (r - y) - 0.27 * (b - y);
	let q = 0.48 * (r - y) + 0.41 * (b - y);
	vec![q as i16, i as i16, y as i16]
}

pub fn hex2yiq(hex: &str) -> Vec<i16> {
	rgb2yiq(hex2rgb(hex))
}

pub fn rgb2hsl(mut rgb: Vec<u8>) -> Vec<f64> {
	let (r, g, b) = (rgb.pop().unwrap() as f64 / 255.0, rgb.pop().unwrap() as f64 / 255.0, rgb.pop().unwrap() as f64 / 255.0);

	let (max, min, sep, coeff) = {
		let (max, min, sep, coeff) = if r > g {
			(r, g, g - b, 0.0)
		} else {
			(g, r, b - r, 2.0)
		};
		if b > max {
			(b, min, r - g, 4.0)
		} else {
			let min_val = if b < min { b } else { min };
			(max, min_val, sep, coeff)
		}
	};

	let sum = max + min;

	let l = sum / 2.0;

	if max == min {
		return vec![l, 0.0, 0.0];
	}

	let delta = max - min;

	let s = {
		if sum > 1.0 {
			delta / (2.0 - sum)
		} else {
			delta / sum
		}
	};

	let h = ((sep / delta) + coeff) * 60.0;

	vec![((l * 100.0) * 10.0).round() / 10.0, ((s * 100.0) * 10.0).round() / 10.0, (h * 1.0).round() / 1.0]
}

pub fn hsl2rgb(mut hsl: Vec<f64>) -> Vec<u8> {
	let (h, s, l) = (hsl.pop().unwrap() / 60.0, hsl.pop().unwrap() / 100.0, hsl.pop().unwrap() / 100.0);

	if s == 0.0 {
		return vec![l as u8, l as u8, l as u8];
	}

	let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
	let x = c * (1.0 - (h % 2.0 - 1.0).abs());
	let m = l - c / 2.0;

	let (r, g, b) = {
		if h >= 0.0 && h < 1.0 {
			(c, x, 0.0)
		} else if h >= 1.0 && h < 2.0 {
			(x, c, 0.0)
		} else if h >= 2.0 && h < 3.0 {
			(0.0, c, x)
		} else if h >= 3.0 && h < 4.0 {
			(0.0, x, c)
		} else if h >= 4.0 && h < 5.0 {
			(x, 0.0, c)
		} else {
			(c, 0.0, x)
		}
	};
	vec![((b + m) * 255.0).round() as u8, ((g + m) * 255.0).round() as u8, ((r + m) * 255.0).round() as u8]
}

pub fn darken_color(mut rgb: Vec<u8>, amp: f64) -> Vec<u8> {
	let (r, g, b) = (rgb.pop().unwrap() as f64 * (1f64 - amp), rgb.pop().unwrap() as f64 * (1f64 - amp), rgb.pop().unwrap() as f64 * (1f64 - amp));
	vec![b as u8, g as u8, r as u8]
}

pub fn blend_color(mut rgb1: Vec<u8>, mut rgb2: Vec<u8>) -> Vec<u8> {
	let (r, g, b) = (0.5f64 * rgb1.pop().unwrap() as f64 + 0.5f64 * rgb2.pop().unwrap() as f64, 0.5f64 * rgb1.pop().unwrap() as f64 + 0.5f64 * rgb2.pop().unwrap() as f64, 0.5f64 * rgb1.pop().unwrap() as f64 + 0.5f64 * rgb2.pop().unwrap() as f64);
	vec![b as u8, g as u8, r as u8]
}

pub fn saturate_color(rgb: Vec<u8>, amp: f64) -> Vec<u8> {
	let mut hsl = rgb2hsl(rgb);
	let (h, mut s, l) = (hsl.pop().unwrap(), hsl.pop().unwrap(), hsl.pop().unwrap());
	s = amp;
	hsl2rgb(vec![l, s, h])
}

pub fn lighten_color(rgb: Vec<u8>, amp: f64) -> Vec<u8> {
	let mut hsl = rgb2hsl(rgb);
	let (h, s, mut l) = (hsl.pop().unwrap(), hsl.pop().unwrap(), hsl.pop().unwrap());
	l = amp;
	hsl2rgb(vec![l, s, h])
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
