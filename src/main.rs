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
use clap::{Arg, Command, ValueHint, value_parser, ArgAction};
use colors::{colordict::ColorDict, colors};
use export::export;
use file::file;
use preview::preview;
use reload::reload;
use std::path::PathBuf;
#[cfg(feature = "timechange")]
use timechange::timebased;
use traitdef::AppOpt;

fn build_cli() -> clap::Command {
	Command::new("changewal").author("Autumn")
	.args(&[
		#[cfg(feature = "timechange")]
		Arg::new("path").help("Path to wallpaper or directory").value_hint(ValueHint::AnyPath),
		#[cfg(not(feature = "timechange"))]
		Arg::new("path").help("Path to wallpaper or directory").value_hint(ValueHint::AnyPath).required(true),
		Arg::new("alpha").short('a').long("alpha").default_value("100").help("Effects output of console escape seq and any values filled in via template").value_parser(value_parser!(u8)),
		Arg::new("skip").short('s').long("skip").default_value("").hide_default_value(true).help("List of things to skip reloading. Valid options are: (t)erminal, (d)unst, (w)allpaper, (h)ooks, (a)ll"),
		Arg::new("vte").short('v').long("vte").help("Skip setting esc seq 708, may fix artifacting in some terms").action(ArgAction::SetTrue),
		Arg::new("preview").short('p').long("preview").help("Preview current color theme").action(ArgAction::SetTrue),
		Arg::new("nocache").long("nocache").help("Disable read/write of cache file").action(ArgAction::SetTrue),
		Arg::new("writeseq").long("writeseq").help("Write file containing escape sequence to ~/.cache/wal/seq").action(ArgAction::SetTrue),
		#[cfg(feature = "timechange")]
		Arg::new("time").long("time").short('t').requires("daybg").requires("nightbg").help("Set wallpaper based on sunset/sunrise").action(ArgAction::SetTrue),
		#[cfg(feature = "timechange")]
		Arg::new("daybg").long("daybg").help("Wallpaper to be used during the day"),
		#[cfg(feature = "timechange")]
		Arg::new("nightbg").long("nightbg").help("Wallpaper to be used during the night"),
		#[cfg(feature = "timechange")]
		Arg::new("lat").long("lat").help("Latitude used for sunrise/sunset calculations").value_parser(value_parser!(f64)),
		#[cfg(feature = "timechange")]
		Arg::new("long").long("long").help("Latitude used for sunrise/sunset calculations").value_parser(value_parser!(f64)),
	])
}

fn main() -> Result<()> {
	let args = build_cli().get_matches();

	let appopt = AppOpt::grabargs(&args);
	
	#[cfg(feature = "timechange")]
	if args.get_flag("time") {
		let daybg = args.get_one::<String>("daybg").expect("required");
		let nightbg = args.get_one::<String>("nightbg").expect("required");
		let lat = *args.get_one::<f64>("lat").expect("required");
		let long = *args.get_one::<f64>("long").expect("required");
		timebased(lat, long, daybg, nightbg, appopt)?;
	} else {
		let path = args.get_one::<String>("path").expect("required");
		stdoperation(path, &appopt)?;
	}

	#[cfg(not(feature = "timechange"))]
	let path = args.get_one::<String>("path").unwrap();

	#[cfg(not(feature = "timechange"))]
	stdoperation(path, &appopt)?;

	if args.get_flag("preview") {
		preview();
	}
	Ok(())
}

fn generate(file: PathBuf, nocache: bool, alpha: u8) -> Result<ColorDict> {
	if nocache {
		Ok(colors(file, alpha)?)
	} else {
		Ok(readcache(&file, alpha).unwrap_or_else(|_| {
			let cache = colors(file, alpha).unwrap();
			writecache(&cache).unwrap();
			cache
		}))
	}
}

fn stdoperation(path: &String, appopt: &AppOpt) -> Result<()> {
	let dict = generate(file(path)?, appopt.nocache, appopt.alpha)?;
	export(&dict)?;
	reload(&dict, &appopt.skip, appopt.vte, appopt.writeseq)?;
	Ok(())
}
