mod cache;
mod colors;
mod export;
mod file;
mod preview;
mod reload;
#[cfg(feature = "timechange")]
mod timechange;
mod traitdef;
use anyhow::Result;
use cache::{readcache, writecache};
use clap::{crate_authors, App, Arg, ValueHint};
use colors::{colordict::ColorDict, colors};
use export::export;
use file::file;
use preview::preview;
use reload::reload;
use std::path::PathBuf;
#[cfg(feature = "timechange")]
use timechange::timebased;
use traitdef::AppOpt;

fn build_cli() -> clap::App<'static> {
	App::new("changewal").author(crate_authors!("\n"))
	.args(&[
		#[cfg(feature = "timechange")]
		Arg::new("path").help("Path to wallpaper or directory").value_hint(ValueHint::AnyPath).required_unless_present("time"),
		#[cfg(not(feature = "timechange"))]
		Arg::new("path").help("Path to wallpaper or directory").value_hint(ValueHint::AnyPath),
		Arg::new("alpha").short('a').long("alpha").default_value("100").help("Effects output of console escape seq and any values filled in via template"),
		Arg::new("skip").short('s').long("skip").default_value("").hide_default_value(true).help("List of things to skip reloading. Valid options are: (t)erminal, (d)unst, (w)allpaper, (h)ooks, (a)ll"),
		Arg::new("vte").short('v').long("vte").takes_value(false).help("Skip setting esc seq 708, may fix artifacting in some terms"),
		Arg::new("preview").short('p').long("preview").takes_value(false).help("Preview current color theme"),
		Arg::new("nocache").long("nocache").takes_value(false).help("Disable read/write of cache file"),
		Arg::new("writeseq").long("writeseq").takes_value(false).help("Write file containing escape sequence to ~/.cache/wal/seq"),
		#[cfg(feature = "timechange")]
		Arg::new("time").long("time").short('t').takes_value(false).requires("daybg").requires("nightbg").help("Set wallpaper based on sunset/sunrise"),
		#[cfg(feature = "timechange")]
		Arg::new("daybg").long("daybg").takes_value(true).help("Wallpaper to be used during the day"),
		#[cfg(feature = "timechange")]
		Arg::new("nightbg").long("nightbg").takes_value(true).help("Wallpaper to be used during the night"),
		#[cfg(feature = "timechange")]
		Arg::new("lat").long("lat").takes_value(true).help("Latitude used for sunrise/sunset calculations"),
		#[cfg(feature = "timechange")]
		Arg::new("long").long("long").takes_value(true).help("Latitude used for sunrise/sunset calculations"),
	])
}

fn main() -> Result<()> {
	let args = build_cli().get_matches();

	let appopt = AppOpt::grabargs(&args);

	#[cfg(feature = "timechange")]
	if args.is_present("time") {
		let daybg = args.value_of_t::<String>("daybg")?;
		let nightbg = args.value_of_t::<String>("nightbg")?;
		let lat = args.value_of_t::<f64>("lat")?;
		let long = args.value_of_t::<f64>("long")?;
		timebased(lat, long, daybg, nightbg, appopt)?;
	} else {
		let path = args.value_of_t::<String>("path")?;
		stdoperation(path, appopt)?;
	}

	#[cfg(not(feature = "timechange"))]
	let path = args.value_of_t::<String>("path")?;

	#[cfg(not(feature = "timechange"))]
	stdoperation(path, appopt)?;

	if args.is_present("preview") {
		preview()
	}
	Ok(())
}

fn generate(file: PathBuf, nocache: bool, alpha: u8) -> Result<ColorDict> {
	if nocache {
		Ok(colors(file, alpha))
	} else {
		Ok(readcache(&file, &alpha).unwrap_or_else(|_| {
			let cache = colors(file, alpha);
			writecache(&cache);
			cache
		}))
	}
}

fn stdoperation(path: String, appopt: AppOpt) -> Result<()> {
	let dict = generate(file(path)?, appopt.nocache, appopt.alpha)?;
	export(&dict)?;
	reload(dict, (appopt.skip).to_string(), appopt.vte, appopt.writeseq)?;
	Ok(())
}
