use crate::colors::ColorDict;

pub fn seq(colors: &ColorDict) -> String {
	let mut temp: String = "".to_string();
	temp.push_str(&set_color(0, &colors.color0));
	temp.push_str(&set_color(1, &colors.color1));
	temp.push_str(&set_color(2, &colors.color2));
	temp.push_str(&set_color(3, &colors.color3));
	temp.push_str(&set_color(4, &colors.color4));
	temp.push_str(&set_color(5, &colors.color5));
	temp.push_str(&set_color(6, &colors.color6));
	temp.push_str(&set_color(7, &colors.color7));
	temp.push_str(&set_color(8, &colors.color8));
	temp.push_str(&set_color(9, &colors.color9));
	temp.push_str(&set_color(10, &colors.color10));
	temp.push_str(&set_color(11, &colors.color11));
	temp.push_str(&set_color(12, &colors.color12));
	temp.push_str(&set_color(13, &colors.color13));
	temp.push_str(&set_color(14, &colors.color14));
	temp.push_str(&set_color(15, &colors.color15));
	temp.push_str(&set_special(10, &colors.foreground));
	temp.push_str(&set_special(11, &colors.background));
	temp.push_str(&set_special(12, &colors.cursor));
	temp.push_str(&set_special(13, &colors.foreground));
	temp.push_str(&set_special(17, &colors.foreground));
	temp.push_str(&set_special(19, &colors.background));
	temp.push_str(&set_color(232, &colors.background));
	temp.push_str(&set_color(256, &colors.foreground));
	temp.push_str(&set_special(708, &colors.background));
	temp
}

fn set_special(index: usize, color: &str) -> String {
	format!("]{};{}\\", index, color)
}

fn set_color(index: usize, color: &str) -> String {
	format!("]4;{};{}\\", index, color)
}
