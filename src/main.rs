mod cache;
mod colors;
mod export;
mod file;
mod preview;
mod reload;
use cache::{readcache, writecache};
use clap::Clap;
use colors::colors;
use export::export;
use file::file;
use preview::preview;
use reload::reload;
#[derive(Clap)]
struct Opts {
	///path of wallpaper
	path: String,
	///EXPERIMENTAL: enables a different color style which has 16 unique colors, instead of just the 9
	#[clap(short = 'n', long = "newstyle")]
	style: bool,
	///effects output of console escape seq and any values filled in via template
	#[clap(short, long, default_value = "100")]
	alpha: usize,
	///List of things to skip reloading. Valid options are: (t)erminal, (p)olybar, (d)unst, (w)allpaper, (a)ll
	#[clap(short, long, default_value = "")]
	skip: String,
	///Skip setting esc seq 708, may fix artifacting in vte terms
	#[clap(short, long)]
	vte: bool,
	///Preview current color theme
	#[clap(short, long)]
	preview: bool,
	///Disable read/write of cache file
	#[clap(long)]
	nocache: bool,
	///Write file containing escape sequence to ~/.cache/wal/seq
	#[clap(long)]
	writeseq: bool,
}

fn main() {
	let args = Opts::parse();

	let img = file(args.path.clone());

	let dict = {
		if args.nocache {
			colors(img, args.style, args.alpha)
		} else {
			readcache(&img, &args.alpha).unwrap_or_else(|_| colors(img, args.style, args.alpha))
		}
	};

	if !args.nocache {
		writecache(&dict);
	}

	export(&dict);
	reload(dict, args.skip, args.vte, args.writeseq);
	if args.preview {
		preview()
	}
}
