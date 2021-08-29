mod cache;
mod colors;
mod export;
mod image;
mod preview;
mod reload;
use cache::{readcache, writecache};
use colors::colors;
use export::export;
use image::image;
use preview::preview;
use reload::reload;
use structopt::StructOpt;
#[derive(StructOpt)]
struct Cli {
	///path of wallpaper
	path: String,
	///value to be passed to feh, valid options are center, fill, scale and tile
	#[structopt(default_value = "fill")]
	setting: String,
	///EXPERIMENTAL: enables a different color style which has 16 unique colors, instead of just the 9
	#[structopt(short = "n", long = "newstyle")]
	style: bool,
	///effects output of console escape seq and any values filled in via template
	#[structopt(short, long, default_value = "100")]
	alpha: usize,
	///List of things to skip reloading. Valid options are: (t)erminal, (x)rdb, (p)olybar, (d)unst, (i)3, (s)way, (w)allpaper, (a)ll
	#[structopt(short, long, default_value = "")]
	skip: String,
	///Skip setting esc seq 708, may fix artifacting in vte terms
	#[structopt(short, long)]
	vte: bool,
	//Preview current color theme
	#[structopt(short, long)]
	preview: bool,
}

fn main() {
	let args = Cli::from_args();

	let img = image(args.path);
	let mut dict = readcache(&img, &args.alpha);
	if dict.background.len() == 0 {
		dict = colors(img, args.style, args.alpha);
		writecache(&dict);
	}
	export(&dict);
	reload(dict, args.skip, args.vte, args.setting);
	if args.preview {
		preview()
	}
}
