pub mod colordict;
#[cfg(feature = "colorthief")]
mod colorthief;

pub mod convert;

use std::path::PathBuf;

#[cfg(feature = "colorthief")]
use colorthief::{adjust, format, gen_colors};

#[cfg(feature = "imagemagick")]
mod imagemagick;

use colordict::ColorDict;

#[cfg(feature = "imagemagick")]
use imagemagick::{adjust, format, gen_colors};

pub fn colors(file: PathBuf, style: bool, alpha: usize) -> ColorDict {
	format(adjust(gen_colors(&file)), file, style, alpha)
}
