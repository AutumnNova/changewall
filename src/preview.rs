pub fn preview() {
	let mut tmp = String::new();
	let mut i = 0;
	while i <= 15 {
		if i <= 7 {
			tmp.push_str(&format!("\u{001B}[4{i}m    \u{001B}\u{001B}"));
		} else if i == 8 {
			tmp.push_str(&format!("\n\u{001B}[48;5;{i}m    \u{001B}\u{001B}"));
		} else {
			tmp.push_str(&format!("\u{001B}[48;5;{i}m    \u{001B}\u{001B}"));
		}
		i += 1;
	}
	println!("{tmp}\n");
}
