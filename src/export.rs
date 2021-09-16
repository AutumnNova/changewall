mod traitdef;
use super::colors::colordict::ColorDict;
use anyhow::Result;
use home::home_dir;
use std::fs::{create_dir_all, metadata, read_dir, read_to_string, write};
use traitdef::Strparse;

pub fn export(dict: &ColorDict) -> Result<()> {
	let templatedir = format!("{}/.config/wal/", home_dir().unwrap().display().to_string());

	let _ = create_dir_all(&templatedir);
	let _ = create_dir_all(&templatedir.replace("/.config/", "/.cache/"));

	for file in read_dir(templatedir)? {
		let file = file?.path();
		let meta = metadata(&file);

		if meta.is_err() || meta?.is_dir() {
			continue;
		}

		if file.to_str().unwrap().contains("reload.toml") {
			continue;
		}

		let mut dat = read_to_string(&file)?
			.replace("{wallpaper}", &dict.wallpaper)
			.replace("{alpha}", &dict.alpha.to_string())
			.replace("{alpha.decimal}", &(dict.alpha as f32 / 100.0).to_string())
			.replace("{background.alpha}", &format!("[{}]{}", dict.alpha, dict.background))
			.replacedef("foreground", &dict.foreground)
			.replacedef("background", &dict.background)
			.replacedef("cursor", &dict.cursor);

		for (i, entry) in dict.colorvec.to_vec().into_iter().enumerate() {
			dat = dat.replacedef(&*format!("color{}", i), &entry);
		}

		write(file.to_str().unwrap().replace("/.config/", "/.cache/"), dat)?;
	}
	Ok(())
}
