mod cache;
mod colors;
mod export;
mod file;
mod preview;
mod reload;
use anyhow::Result;
use cache::{readcache, writecache};
use clap::{crate_authors, App, Arg, Parser, ValueHint};
use colors::colors;
use export::export;
use file::file;
use preview::preview;
use reload::reload;

fn build_cli() -> clap::App<'static> {
	App::new("changewal").author(crate_authors!("\n"))
	.args(&[
		Arg::new("path").required_unless_present("zsh").about("Path to wallpaper or directory").value_hint(ValueHint::AnyPath),
		Arg::new("alpha").short('a').long("alpha").default_value("100").about("Effects output of console escape seq and any values filled in via template"),
		Arg::new("skip").short('s').long("skip").default_value("").hide_default_value(true).about("List of things to skip reloading. Valid options are: (t)erminal, (d)unst, (w)allpaper, (h)ooks, (a)ll"),
		Arg::new("zsh").long("zsh").about("Print zsh completions to stdout and quit."),
		Arg::new("style").short('n').long("newstyle").takes_value(false).about("EXPERIMENTAL: enables a different color style which has 16 unique colors, instead of just 9"),
		Arg::new("vte").short('v').long("vte").takes_value(false).about("Skip setting esc seq 708, may fix artifacting in some terms"),
		Arg::new("preview").short('p').long("preview").takes_value(false).about("Preview current color theme"),
		Arg::new("nocache").long("nocache").takes_value(false).about("Disable read/write of cache file"),
		Arg::new("writeseq").long("writeseq").takes_value(false).about("Write file containing escape sequence to ~/.cache/wal/seq"),
	])
}

#[derive(Parser)]
struct Opts {
	///path to wallpaper
	path: String,
	///EXPERIMENTAL: enables a different color style which has 16 unique colors, instead of just 9
	#[clap(short = 'n', long = "newstyle")]
	style: bool,
	///effects output of console escape seq and any values filled in via template
	#[clap(short, long, default_value = "100")]
	alpha: usize,
	///List of things to skip reloading. Valid options are: (t)erminal, (d)unst, (w)allpaper, (h)ooks, (a)ll
	#[clap(short, long, default_value = "")]
	skip: String,
	///Skip setting esc seq 708, may fix artifacting in some terms
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

fn main() -> Result<()> {
	let arg = build_cli().get_matches();

	let img = file(arg.value_of_t::<String>("path")?)?;

	let dict = {
		let style = arg.is_present("style");
		let alpha = arg.value_of_t::<usize>("alpha")?.min(100).max(0);
		if arg.is_present("nocache") {
			colors(img, style, alpha)
		} else {
			readcache(&img, &alpha).unwrap_or_else(|_| {
				let cache = colors(img, style, alpha);
				writecache(&cache);
				cache
			})
		}
	};

	export(&dict)?;
	let skip = arg.value_of_t("skip")?;
	let vte = arg.is_present("vte");
	let writeseq = arg.is_present("writeseq");
	reload(dict, skip, vte, writeseq)?;
	let argpreview = arg.is_present("preview");
	if argpreview {
		preview()
	}
	Ok(())
}
