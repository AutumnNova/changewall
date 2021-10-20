use super::super::colors::colordict::ColorDict;
pub fn seq(dict: ColorDict, vte: bool) -> String {
	let mut prealloc_len: usize = 364;
	if !vte {
		prealloc_len += 15;
	}

	let mut temp = String::with_capacity(prealloc_len);

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
	let mut temp = String::with_capacity(12 + index.to_string().len());
	temp.push_str("\u{001B}]");
	temp.push_str(&index.to_string());
	temp.push(';');
	temp.push_str(color);
	temp.push_str("\u{001B}\\");
	temp
}

fn set_special_alpha(index: usize, color: &str, alpha: usize) -> String {
	if alpha != 100 {
		let mut temp = String::with_capacity(14 + index.to_string().len() + alpha.to_string().len());
		temp.push_str("\u{001B}]");
		temp.push_str(&index.to_string());
		temp.push(';');
		temp.push('[');
		temp.push_str(&alpha.to_string());
		temp.push(']');
		temp.push_str(color);
		temp.push_str("\u{001B}\\");
		temp
	} else {
		set_special(index, color)
	}
}

fn set_color(index: usize, color: &str) -> String {
	let mut temp = String::with_capacity(13 + index.to_string().len());
	temp.push_str("\u{001B}]4;");
	temp.push_str(&index.to_string());
	temp.push(';');
	temp.push_str(color);
	temp.push_str("\u{001B}\\");
	temp
}
