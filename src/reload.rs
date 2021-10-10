mod hooks;
mod seq;
use super::colors::colordict::ColorDict;
use anyhow::{Context, Result};
use home::home_dir;
use hooks::Reload;
use nix::{sys::signal::{kill, Signal::SIGKILL}, unistd::Pid};
use notify_rust::{Notification, Urgency::Normal};
use procfs::process::all_processes;
use seq::seq;
use std::{fs::{read_dir, read_to_string, write}, path::Path, process::{Command, Stdio}, thread::sleep, time::Duration};
use toml::from_str;

pub fn reload(dict: ColorDict, skip: String, vte: bool, writeseq: bool) -> Result<()> {
	if skip == "a" {
		return Ok(());
	}

	if !skip.contains('h') {
		reload_hooks()?
	}

	let mut pid: i32 = -999;

	for prc in all_processes()? {
		if prc.stat.comm == "dunst" {
			pid = prc.stat.pid;
			break;
		};
	}

	reload_progs(dict, skip, vte, writeseq, pid)?;
	Ok(())
}

fn reload_hooks() -> Result<()> {
	let path = format!("{}/.config/wal/reload.toml", home_dir().unwrap().display().to_string());
	let string = read_to_string(path).unwrap_or_default();
	if !string.is_empty() {
		let reload_hook: Reload = from_str(&string)?;

		for mut item in reload_hook.items.unwrap() {
			item.args.insert(0, item.hook);
			droppedcmd(&item.args)
		}
	}
	Ok(())
}

fn reload_progs(dict: ColorDict, skip: String, vte: bool, writeseq: bool, pid: i32) -> Result<()> {
	if !skip.contains('w') {
		wallpaper(&dict.wallpaper);
	}
	if !skip.contains('t') {
		pts(dict, vte, writeseq)?;
	}
	if pid != -999 && !skip.contains('d') {
		notif_daemon(pid)?;
	}
	Ok(())
}

fn notif_daemon(pid: i32) -> Result<()> {
	kill(Pid::from_raw(pid), SIGKILL).with_context(|| "Failed to send SIGKILL to notification daemon")?;

	sleep(Duration::from_millis(1));

	Notification::new()
		.summary("wal")
		.body("Reloaded wal configurations!")
		.urgency(Normal)
		.id(1390764)
		.show()
		.unwrap();
	Ok(())
}

fn pts(dict: ColorDict, vte: bool, writeseq: bool) -> Result<()> {
	let seq = seq(dict, vte);
	for dir in read_dir("/dev/pts/").with_context(|| "Failed to read /dev/pts/")? {
		let file = dir?.path();
		if !file.to_str().unwrap().contains("ptmx") {
			write(file, &seq).with_context(|| "Failed to write to /dev/pts/[0..]")?;
		}
	}
	if writeseq {
		write(format!("{}/.cache/wal/seq", home_dir().unwrap().display().to_string()), seq).with_context(|| "Failed to write seq file")?;
	}
	Ok(())
}

fn wallpaper(path: &Path){
	droppedcmd(&["feh".to_string(), "--no-fehbg".to_string(), "--bg-fill".to_string(), path.display().to_string()])
}

fn droppedcmd(command: &[String]) {
	let _ = Command::new(&command[0])
		.args(&command[1..])
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn();
}
