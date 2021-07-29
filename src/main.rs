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
#[derive(StructOpt)]
struct Cli {
	//path of wallpaper
	path: String,
	#[structopt(default_value = "fill")]
	setting: String,
}

fn main() {
	let args = Cli::from_args();

	let dict = colors(image(args.path, args.setting));
	let seq = seq(&dict);
	export(&dict);
	reload(seq);
}
