pub mod colordict;
#[cfg(feature = "colorthief")]
mod colorthief;

pub mod convert;

use std::path::PathBuf;

#[cfg(feature = "colorthief")]
use colorthief::{adjust, format, gen_colors};

#[cfg(not(feature = "colorthief"))]
mod imagemagick;

use colordict::ColorDict;

#[cfg(not(feature = "colorthief"))]
use imagemagick::{adjust, format, gen_colors};

pub fn colors(file: PathBuf, alpha: u8) -> ColorDict {
	format(adjust(gen_colors(&file)), file, alpha)
}
