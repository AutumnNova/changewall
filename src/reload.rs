use home::home_dir;
use nix::sys::signal::{kill, Signal::{SIGKILL, SIGUSR1}};
use notify_rust::{Notification, Urgency::Normal};
use procfs::process::all_processes;
use std::{fs::{read_dir, write}, process::{Command, Stdio}, thread::sleep, time::Duration};

pub fn reload(seq: String, skip: String) {
	if skip == "a" {
		return;
	}
	let mut proc = String::new();

	for prc in all_processes().unwrap() {
		match &*prc.stat.comm {
			"dunst" => proc.push_str("d"),
			"polybar" => proc.push_str("p"),
			"i3" => proc.push_str("i"),
			"sway" => proc.push_str("s"),
			_ => (),
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
	write(format!("{}/.cache/wal/seq", home_dir().unwrap().display().to_string()), seq).expect("write failed");
}

fn polybar() {
	for prc in all_processes().unwrap() {
		if prc.stat.comm == "polybar" {
			kill(prc.stat.pid, SIGUSR1).expect("SIGUSR1 failed")
		}
	}
}

fn xrdb() {
	droppedcmd("xrdb", "-merge", &format!("{}/.cache/wal/colors.Xresources", home_dir().unwrap().display().to_string()));
}

fn i3() {
	droppedcmd("swaymsg", "i3-msg", "");
}

fn sway() {
	droppedcmd("swaymsg", "reload", "");
}

fn droppedcmd(cmd: &str, arg: &str, arg2: &str) {
	let _ = Command::new(cmd)
		.args([arg, arg2])
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn();
}
