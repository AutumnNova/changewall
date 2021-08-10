use home::home_dir;
use nix::sys::signal::{kill, Signal::{SIGKILL, SIGUSR1}};
use notify_rust::{Notification, Urgency::Normal};
use procfs::process::all_processes;
use std::{fs::{read_dir, write}, process::Command, thread::sleep, time::Duration};

pub fn reload(seq: String, skip: String) {

	if skip == "a" {
		return
	}
	let mut proc = String::new();

	for prc in all_processes().unwrap() {
		match &*prc.stat.comm {
			"dunst" => proc.push_str("d"),
			"polybar" => proc.push_str("p"),
			"i3" => proc.push_str("i"),
			"sway" => proc.push_str("s"),
			_ => ()
		}
	}

	if skip == "" {
		reload_checked(seq, proc)
	} else {
		reload_checked_skips(seq, skip, proc)
	}
}


fn reload_checked(seq: String, proc: String) {
		pts(seq);
		xrdb();
	if proc.contains('p') {
		polybar();
	}
	if proc.contains('d') {
		dunst();
	}
	if proc.contains('i') {
		i3();
	}
	if proc.contains('s') {
		sway();
	}
}


fn reload_checked_skips(seq: String, skip: String, proc: String) {
	if !skip.contains('t') {
		pts(seq);
	}
	if !skip.contains('x') {
		xrdb();
	}
	if proc.contains('p') || !skip.contains('p') {
		polybar();
	}
	if proc.contains('d') || !skip.contains('d') {
		dunst();
	}
	if proc.contains('i') || !skip.contains('i') {
		i3();
	}
	if proc.contains('s') || !skip.contains('s') {
		sway();
	}
}

fn dunst() {
	for prc in all_processes().unwrap() {
		if prc.stat.comm == "dunst" || prc.stat.comm == "/usr/bin/dunst" {
			kill(prc.stat.pid, SIGKILL).expect("SIGTERM failed")
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

fn pts(seq: String) {
	for dir in read_dir("/dev/pts/").unwrap() {
		let file = dir.unwrap().path().display().to_string();
		if !file.contains("ptmx") {
			write(file, &seq).expect("write to /dev/pts failed.");
		}
	}
}

fn polybar() {
	for prc in all_processes().unwrap() {
		if prc.stat.comm == "polybar" {
			kill(prc.stat.pid, SIGUSR1).expect("SIGUSR1 failed")
		}
	}
}

fn xrdb() {
	let _ = Command::new("xrdb")
		.args(["-merge", "-quiet", &format!("{}/.cache/wal/colors.Xresources", home_dir().unwrap().display().to_string())])
		.spawn();
}

fn i3() {
	let _ = Command::new("i3-msg")
		.arg("reload")
		.output();
}

fn sway() {
	let _ = Command::new("swaymsg")
		.arg("reload")
		.spawn();
}
