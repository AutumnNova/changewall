use colors::colors;
use export::export;
use seq::seq;
use structopt::StructOpt;
use std::{thread, time};
mod colors;
mod image;
mod reload;
mod seq;
mod export;
#[derive(StructOpt)]
struct Cli {
	//path of wallpaper
	path: String,
	#[structopt(default_value = "fill")]
	setting: String
}

fn main() {
	let args = Cli::from_args();

	let path = image::image(args.path, args.setting);

	if path == "".to_string() {
		println!("Path does not point to a valid file/directory");
		std::process::exit(0);
	}

	let dict = colors(&path);
	let seq = seq(&dict);
	export(&dict);

	thread::sleep(time::Duration::from_millis(1));

	reload::reload(seq);
}