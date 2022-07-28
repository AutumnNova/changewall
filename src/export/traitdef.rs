use super::super::colors::convert::{hex2rgbdisplay, hex2xrgb};
use anyhow::Result;

pub trait Adddefs {
	fn push_variants(&mut self, to: &str) -> Result<()>;
}

impl Adddefs for Vec<String>{
	fn push_variants(&mut self, to: &str) -> Result<()> {
		self.push(to.to_string());
		self.push(to.strip_prefix('#').unwrap().to_string());
		self.push(hex2rgbdisplay(to)?);
		self.push(hex2xrgb(to)?);
		Ok(())
	}
}
