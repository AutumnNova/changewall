use colors::colors;
use export::export;
use image::image;
use reload::reload;
use seq::seq;
use std::{process::exit, thread::sleep, time::Duration};
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

	let path = image(args.path, args.setting);

	if path == "".to_string() {
		println!("Path does not point to a valid file/directory");
		exit(0);
	}

	let dict = colors(&path);
	let seq = seq(&dict);
	export(&dict);

	sleep(Duration::from_millis(1));

	reload(seq);
}
