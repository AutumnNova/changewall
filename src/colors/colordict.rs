#[derive(Clone)]
pub struct ColorDict {
	pub wallpaper: String,
	pub alpha: usize,
	pub background: String,
	pub foreground: String,
	pub cursor: String,
	pub colorvec: Vec<String>,
}

impl ColorDict {
	#[inline]
	pub const fn new() -> Self {
		ColorDict {
			wallpaper: String::new(),
			alpha: 100,
			background: String::new(),
			foreground: String::new(),
			cursor: String::new(),
			colorvec: Vec::new(),
		}
	}

	#[inline]
	pub fn clonedict(dict: &ColorDict) -> ColorDict {
		let mut temp: ColorDict = ColorDict::new();
		temp.clone_from(dict);
		temp
	}
}
