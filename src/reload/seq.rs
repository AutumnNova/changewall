use super::super::colors::colordict::ColorDict;
pub fn seq(dict: ColorDict, vte: bool) -> String {
	let mut temp: String = String::new();

	for (i, entry) in dict.colorvec.iter().enumerate() {
		temp.push_str(&set_color(i, entry));
	}

	temp.push_str(&set_special(10, &dict.foreground));
	temp.push_str(&set_special_alpha(11, &dict.background, dict.alpha));
	temp.push_str(&set_special(12, &dict.cursor));
	temp.push_str(&set_special(13, &dict.foreground));
	temp.push_str(&set_special(17, &dict.foreground));
	temp.push_str(&set_special(19, &dict.background));
	temp.push_str(&set_color(232, &dict.background));
	temp.push_str(&set_color(256, &dict.foreground));
	if !vte {
		temp.push_str(&set_special_alpha(708, &dict.background, dict.alpha));
	}
	temp
}

fn set_special(index: usize, color: &str) -> String {
	format!("\u{001B}]{};{}\u{001B}\\", index, color)
}

fn set_special_alpha(index: usize, color: &str, alpha: usize) -> String {
	if alpha != 100 {
		format!("\u{001B}]{};[{}]{}\u{001B}\\", index, alpha, color)
	} else {
		format!("\u{001B}]{};{}\u{001B}\\", index, color)
	}
}

fn set_color(index: usize, color: &str) -> String {
	format!("\u{001B}]4;{};{}\u{001B}\\", index, color)
}
