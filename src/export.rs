mod traitdef;
use super::colors::colordict::ColorDict;
use aho_corasick::{AhoCorasickBuilder, MatchKind};
use anyhow::Result;
use home::home_dir;
use std::fs::{create_dir_all, read_dir, read_to_string, write};
use traitdef::Adddefs;

pub fn export(dict: &ColorDict) -> Result<()> {
	const PATTERN: &[&str; 80] = &["{wallpaper}", "{alpha}", "{alpha.decimal}", "{background.alpha}", "{foreground}", "{foreground.strip}", "{foreground.rgb}", "{foreground.xrgb}", "{background}", "{background.strip}", "{background.rgb}", "{background.xrgb}", "{cursor}", "{cursor.strip}", "{cursor.rgb}", "{cursor.xrgb}", "{color0}", "{color0.strip}", "{color0.rgb}", "{color0.xrgb}", "{color1}", "{color1.strip}", "{color1.rgb}", "{color1.xrgb}", "{color2}", "{color2.strip}", "{color2.rgb}", "{color2.xrgb}", "{color3}", "{color3.strip}", "{color3.rgb}", "{color3.xrgb}", "{color4}", "{color4.strip}", "{color4.rgb}", "{color4.xrgb}", "{color5}", "{color5.strip}", "{color5.rgb}", "{color5.xrgb}", "{color6}", "{color6.strip}", "{color6.rgb}", "{color6.xrgb}", "{color7}", "{color7.strip}", "{color7.rgb}", "{color7.xrgb}", "{color8}", "{color8.strip}", "{color8.rgb}", "{color8.xrgb}", "{color9}", "{color9.strip}", "{color9.rgb}", "{color9.xrgb}", "{color10}", "{color10.strip}", "{color10.rgb}", "{color10.xrgb}", "{color11}", "{color11.strip}", "{color11.rgb}", "{color11.xrgb}", "{color12}", "{color12.strip}", "{color12.rgb}", "{color12.xrgb}", "{color13}", "{color13.strip}", "{color13.rgb}", "{color13.xrgb}", "{color14}", "{color14.strip}", "{color14.rgb}", "{color14.xrgb}", "{color15}", "{color15.strip}", "{color15.rgb}", "{color15.xrgb}"];

	let templatedir = home_dir().unwrap().join(".config/wal");
	let cachedir = home_dir().unwrap().join(".cache/wal");

	create_dir_all(&templatedir)?;
	create_dir_all(cachedir)?;

	let mut value_vec = Vec::<String>::with_capacity(80);
	value_vec.push(dict.wallpaper.to_string_lossy().to_string());
	value_vec.push(dict.alpha.to_string());
	value_vec.push(ryu::Buffer::new().format_finite(dict.alpha as f32 / 100.0).to_string());
	value_vec.push(format!("[{}]{}", dict.alpha, dict.colorvec[0]));
	value_vec.push_variants(&dict.colorvec[15])?;
	value_vec.push_variants(&dict.colorvec[0])?;
	value_vec.push_variants(&dict.colorvec[15])?;

	for entry in &dict.colorvec {
		value_vec.push_variants(entry)?;
	}

	let ac = AhoCorasickBuilder::new()
		.match_kind(MatchKind::LeftmostFirst)
		.auto_configure(PATTERN)
		.build(PATTERN);

	for file in read_dir(templatedir)? {
		let file = file?.path();
		if file.is_dir() || file.file_name().unwrap() == "reload.toml" {
			continue;
		}

		let dat = read_to_string(&file)?;

		write(file.to_str().unwrap().replace("/.config/", "/.cache/"), ac.replace_all(&dat, &value_vec))?;
	}
	Ok(())
}
