mod hooks;
mod seq;
use super::colors::colordict::ColorDict;
use home::home_dir;
use hooks::Reload;
use nix::{sys::signal::{kill, Signal::{SIGKILL, SIGUSR1}}, unistd::Pid};
use notify_rust::{Notification, Urgency::Normal};
use procfs::process::all_processes;
use seq::seq;
use std::{fs::{read_dir, read_to_string, write}, path::Path, process::{Command, Stdio}, thread::sleep, time::Duration};
use toml::from_str;
use tree_magic_mini::from_filepath;
use wall::xlib::{ImageFormat, Xlib};

pub fn reload(dict: ColorDict, skip: String, vte: bool, writeseq: bool) {
	let path = format!("{}/.config/wal/reload.toml", home_dir().unwrap().display().to_string());
	let string = read_to_string(path).unwrap_or_default();
	if !string.is_empty() {
		let reload_hook: Reload = from_str(&string).unwrap();

		for mut item in reload_hook.items.unwrap() {
			if !which(&item.hook) {
				item.args.insert(0, item.hook);
				droppedcmd(&item.args)
			}
		}	
	}

	if skip == "a" {
		return;
	}

	let mut proc = String::new();

	for prc in all_processes().unwrap() {
		match &*prc.stat.comm {
			"dunst" => proc.push('d'),
			"polybar" => proc.push('p'),
			_ => (),
		}
	}

	reload_progs(dict, skip, proc, vte, writeseq)
}

fn reload_progs(dict: ColorDict, skip: String, proc: String, vte: bool, writeseq: bool) {
	if !skip.contains('w') {
		wallpaper(&dict.wallpaper);
	}
	if !skip.contains('t') {
		pts(dict, vte, writeseq);
	}
	if proc.contains('p') && !skip.contains('p') {
		polybar();
	}
	if proc.contains('d') && !skip.contains('d') {
		dunst();
	}
}

fn dunst() {
	for prc in all_processes().unwrap() {
		if prc.stat.comm == "dunst" || prc.stat.comm == "/usr/bin/dunst" {
			kill(Pid::from_raw(prc.stat.pid), SIGKILL).expect("SIGTERM failed")
		}
	}
	sleep(Duration::from_millis(1));

	Notification::new()
		.summary("wal")
		.body("Reloaded wal configurations!")
		.urgency(Normal)
		.show()
		.unwrap();
}

fn pts(dict: ColorDict, vte: bool, writeseq: bool) {
	let seq = seq(dict, vte);
	for dir in read_dir("/dev/pts/").unwrap() {
		let file = dir.unwrap().path().display().to_string();
		if !file.contains("ptmx") {
			write(file, &seq).expect("write to /dev/pts failed.");
		}
	}
	if writeseq {
		write(format!("{}/.cache/wal/seq", home_dir().unwrap().display().to_string()), seq).expect("write failed");
	}
}

fn polybar() {
	for prc in all_processes().unwrap() {
		if prc.stat.comm == "polybar" {
			kill(Pid::from_raw(prc.stat.pid), SIGUSR1).expect("SIGUSR1 failed")
		}
	}
}

fn wallpaper(path: &str) {
	let path = Path::new(&path);
	Xlib::set(&Xlib::new().unwrap(), path, mime2format(path)).unwrap();
}

fn which(cmd: &str) -> bool {
	let which = Command::new("which").arg(cmd).output().unwrap();
	which.stderr.is_empty()
}

fn droppedcmd(command: &[String]) {
	let _ = Command::new(&command[0])
		.args(&command[1..])
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn();
}

fn mime2format(path: &Path) -> Option<ImageFormat> {
	match from_filepath(path).unwrap() {
		"image/jpeg" => Some(ImageFormat::Jpeg),
		"image/png" => Some(ImageFormat::Png),
		"image/avif" => Some(ImageFormat::Avif),
		"image/bmp" => Some(ImageFormat::Bmp),
		"image/webp" => Some(ImageFormat::WebP),
		"image/tiff" => Some(ImageFormat::Tiff),
		_ => None,
	}
}
