use super::super::colors::convert::{hex2rgbdisplay, hex2xrgb};

pub trait Adddefs {
	fn push_variants(&mut self, to: String);
}

impl Adddefs for Vec<String> {
	fn push_variants(&mut self, to: String) {
		self.push(to.clone());
		self.push(to.strip_prefix('#').unwrap().to_string());
		self.push(hex2rgbdisplay(&to));
		self.push(hex2xrgb(&to));
	}
}
