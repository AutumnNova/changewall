mod traitdef;
use super::super::colors::colordict::ColorDict;
use traitdef::PushSeq;
pub fn seq(dict: ColorDict, vte: bool) -> String {
	let mut prealloc_len: usize = 364;
	if !vte {
		prealloc_len += 15;
	}

	let mut temp = String::with_capacity(prealloc_len);

	for (i, entry) in dict.colorvec.iter().enumerate() {
		temp.push_strs_color(i, entry);
	}

	temp.push_strs_special(10, &dict.foreground);
	temp.push_strs_special_alpha(11, &dict.background, dict.alpha);
	temp.push_strs_special(12, &dict.cursor);
	temp.push_strs_special(13, &dict.foreground);
	temp.push_strs_special(17, &dict.foreground);
	temp.push_strs_special(19, &dict.background);
	temp.push_strs_color(232, &dict.background);
	temp.push_strs_color(256, &dict.foreground);
	if !vte {
		temp.push_strs_special_alpha(708, &dict.background, dict.alpha);
	}
	temp
}
