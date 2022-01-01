use palette::rgb::Rgb;
use std::{path::Path, process::Command};

pub trait MagickGen {
	fn imagemagick(&mut self, file: &Path, quant: u8);
}

impl MagickGen for Vec<Rgb> {
	fn imagemagick(&mut self, file: &Path, quant: u8) {
		let output = Command::new("magick")
			.args([file.to_str().unwrap(), "-resize", "25%", "-colors", &quant.to_string(), "-unique-colors", "txt:-"])
			.output()
			.expect("failed to gather colors");
	
		for line in String::from_utf8_lossy(&output.stdout).to_string().lines().skip(1) {
			let tmp = line.replace('(', "").replace(')', "").split(' ').nth(1).unwrap().to_string();
			let mut tmp2 = tmp.split(',');
			let color: Rgb = Rgb::new(
				tmp2.next().unwrap().parse::<f32>().unwrap() / 255.0,
				tmp2.next().unwrap().parse::<f32>().unwrap() / 255.0,
				tmp2.next().unwrap().parse::<f32>().unwrap() / 255.0,
			);
			self.insert(0, color);
		}
	}
}
