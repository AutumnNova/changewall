use colors::colors;
use export::export;
use image::image;
use reload::reload;
use seq::seq;
use structopt::StructOpt;
mod colors;
mod export;
mod image;
mod reload;
mod seq;
mod colordict;
#[derive(StructOpt)]
struct Cli {
	//path of wallpaper
	path: String,
	#[structopt(default_value = "fill")]
	setting: String,
	//enables a different color style which has 16 unique colors, instead of just the 9
	#[structopt(short = "n", long = "newstyle")]
	style: bool,
	//effects output of console escape seq and any values filled in via template
	#[structopt(short = "a", long = "alpha", default_value = "100")]
	alpha: usize,
}

fn main() {
	let args = Cli::from_args();

	let dict = colors(image(args.path, args.setting), args.style, args.alpha);
	export(&dict);
	reload(seq(&dict));
}
