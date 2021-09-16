mod hooks;
mod seq;
use super::colors::colordict::ColorDict;
use anyhow::{Context, Result};
use home::home_dir;
use hooks::Reload;
use nix::{sys::signal::{kill, Signal::{SIGKILL, SIGUSR1}}, unistd::Pid};
use notify_rust::{Notification, Urgency::Normal};
use procfs::process::all_processes;
use seq::seq;
use std::{fs::{read_dir, read_to_string, write}, process::{Command, Stdio}, thread::sleep, time::Duration};
use toml::from_str;

pub fn reload(dict: ColorDict, skip: String, vte: bool, writeseq: bool) -> Result<()> {
	let path = format!("{}/.config/wal/reload.toml", home_dir().unwrap().display().to_string());
	let string = read_to_string(path).unwrap_or_default();
	if !string.is_empty() {
		let reload_hook: Reload = from_str(&string)?;

		for mut item in reload_hook.items.unwrap() {
			item.args.insert(0, item.hook);
			droppedcmd(&item.args)
		}
	}

	if skip == "a" {
		return Ok(());
	}

	let mut proc = String::new();
	let mut pid = vec![];

	for prc in all_processes()? {
		match &*prc.stat.comm {
			"dunst" => {proc.push('d'); pid.push(prc.stat.pid)},
			"polybar" => {proc.push('p'); pid.push(prc.stat.pid)},
			_ => (),
		};
		
	}

	reload_progs(dict, skip, proc, vte, writeseq, &pid)?;
	Ok(())
}

fn reload_progs(dict: ColorDict, skip: String, proc: String, vte: bool, writeseq: bool, pid: &[i32]) -> Result<()> {
	if !skip.contains('w') {
		wallpaper(&dict.wallpaper);
	}
	if !skip.contains('t') {
		pts(dict, vte, writeseq)?;
	}
	if proc.contains('p') && !skip.contains('p') {
		polybar(*pid.get(proc.find('p').unwrap()).unwrap())?;
	}
	if proc.contains('d') && !skip.contains('d') {
		dunst(*pid.get(proc.find('d').unwrap()).unwrap())?;
	}
	Ok(())
}

fn dunst(pid: i32) -> Result<()> {
	kill(Pid::from_raw(pid), SIGKILL).with_context(|| "Failed to send SIGKILL to dunst")?;

	sleep(Duration::from_millis(1));

	Notification::new()
		.summary("wal")
		.body("Reloaded wal configurations!")
		.urgency(Normal)
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

fn polybar(pid: i32) -> Result<()> {
	kill(Pid::from_raw(pid), SIGUSR1).with_context(|| "Failed to send SIGUSR1 to polybar")
}

fn wallpaper(path: &str){
	droppedcmd(&["feh".to_string(), "--no-fehbg".to_string(), "--bg-fill".to_string(), path.to_string()])
}

fn droppedcmd(command: &[String]) {
	let _ = Command::new(&command[0])
		.args(&command[1..])
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn();
}
