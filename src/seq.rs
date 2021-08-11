use super::colordict::ColorDict;
pub fn seq(colors: &ColorDict, vte: bool) -> String {
	let mut dict: ColorDict = ColorDict::clonedict(colors);
	let mut temp: String = String::new();
	let mut i = 0;

	for entry in &mut dict.colorvec {
		temp.push_str(&set_color(i, entry));
		i += 1;
	}

	temp.push_str(&set_special(10, &colors.foreground));
	temp.push_str(&set_special_alpha(11, &colors.background, colors.alpha));
	temp.push_str(&set_special(12, &colors.cursor));
	temp.push_str(&set_special(13, &colors.foreground));
	temp.push_str(&set_special(17, &colors.foreground));
	temp.push_str(&set_special(19, &colors.background));
	temp.push_str(&set_color(232, &colors.background));
	temp.push_str(&set_color(256, &colors.foreground));
	if !vte {
		temp.push_str(&set_special_alpha(708, &colors.background, colors.alpha));
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
