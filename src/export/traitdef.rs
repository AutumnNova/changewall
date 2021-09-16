use super::super::colors::convert::{hex2rgbdisplay, hex2xrgb};

pub trait Strparse {
	fn replacedef<'a>(&'a self, from: &str, to: &str) -> String;
}

impl Strparse for String {
	fn replacedef(&self, from: &str, to: &str) -> String {
		self.replace(&format!("{{{}}}", from), to)
			.replace(&format!("{{{}.strip}}", from), to.strip_prefix('#').unwrap())
			.replace(&format!("{{{}.rgb}}", from), &hex2rgbdisplay(to))
			.replace(&format!("{{{}.xrgba}}", from), &hex2xrgb(to))
	}
}
