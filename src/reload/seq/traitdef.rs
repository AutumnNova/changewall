pub trait PushSeq {
	fn push_strs_color(&mut self, index: usize, color: &str);
	fn push_strs_special(&mut self, index: u16, color: &str);
	fn push_strs_special_alpha(&mut self, index: u16, color: &str, alpha: u8);
}

impl PushSeq for String {
	fn push_strs_color(&mut self, index: usize, color: &str) {
		self.push_str("\u{001B}]4;");
		self.push_str(&index.to_string());
		self.push(';');
		self.push_str(color);
		self.push_str("\u{001B}\\");
	}

	fn push_strs_special(&mut self, index: u16, color: &str) {
		self.push_str("\u{001B}]");
		self.push_str(&index.to_string());
		self.push(';');
		self.push_str(color);
		self.push_str("\u{001B}\\");
	}

	fn push_strs_special_alpha(&mut self, index: u16, color: &str, alpha: u8) {
		if alpha != 100 {
			self.push_str("\u{001B}]");
			self.push_str(&index.to_string());
			self.push(';');
			self.push('[');
			self.push_str(&alpha.to_string());
			self.push(']');
			self.push_str(color);
			self.push_str("\u{001B}\\");
		} else {
			self.push_strs_special(index, color);
		}
	}
}
