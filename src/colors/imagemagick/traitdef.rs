use aho_corasick::AhoCorasick;
use anyhow::Result;
use magick_rust::PixelWand;
use palette::Srgb;
use std::{path::Path, process::Command};

pub trait MagickGen {
	fn imagemagick(&mut self, file: &Path, quant: u8, ac: &AhoCorasick) -> Result<()>;
	fn formatpix(&mut self, pix: Vec<PixelWand>) -> Result<()>;
}

impl MagickGen for Vec<Srgb<u8>> {
	fn imagemagick(&mut self, file: &Path, quant: u8, ac: &AhoCorasick) -> Result<()> {
		const REPLACE: &[&str; 2] = &["", ""];

		let output = Command::new("magick")
			.args([file.to_str().unwrap(), "-resize", "25%", "-colors", &quant.to_string(), "-unique-colors", "txt:-"])
			.output()
			.expect("failed to gather colors");

		for line in ac.replace_all(&String::from_utf8_lossy(&output.stdout), REPLACE).lines().skip(1) {
			let tmp = line.split(' ').nth(1).unwrap().to_string();
			let mut tmp2 = tmp.split(',');
			let color: Srgb<u8> = Srgb::new(
				tmp2.next().unwrap().parse::<u8>()?,
				tmp2.next().unwrap().parse::<u8>()?,
				tmp2.next().unwrap().parse::<u8>()?,
			);
			self.insert(0, color);
		}
		Ok(())
	}

	fn formatpix(&mut self, pix: Vec<PixelWand>) -> Result<() >{
		for pix in pix {
			let tmp = pix.get_color_as_normalized_string()?;
			let mut tmp2 = tmp.split(',');
			let color: Srgb<u8> = Srgb::new(
				(tmp2.next().unwrap().parse::<f64>()? * 255.0f64).round() as u8,
				(tmp2.next().unwrap().parse::<f64>()? * 255.0f64).round() as u8,
				(tmp2.next().unwrap().parse::<f64>()? * 255.0f64).round() as u8,
			);
			self.insert(0, color);
		};
		Ok(())
	}
}
