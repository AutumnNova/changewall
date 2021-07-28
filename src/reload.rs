use lazy_static::lazy_static;
use nix::sys::signal::{kill, Signal::{SIGKILL, SIGUSR1}};
use notify_rust::{Notification, Urgency::Normal};
use procfs::process::all_processes;
use regex::Regex;
use std::{fs::{read_dir, write}, process::Command, thread::sleep, time::Duration};

pub fn reload(seq: String) {
	pts(seq);
	polybar();
	xrdb();
	dunst();
}

fn dunst() {
	for prc in all_processes().unwrap() {
		if prc.stat.comm == "dunst" || prc.stat.comm == "/usr/bin/dunst" {
			kill(prc.stat.pid, SIGKILL).expect("SIGTERM failed")
		}
	}
	sleep(Duration::from_millis(1));

	Notification::new()
		.summary("pywal")
		.body("Reloaded wal configurations!")
		.urgency(Normal)
		.show()
		.unwrap();
}

fn pts(seq: String) {
	lazy_static! {
		static ref RE: Regex = Regex::new("(/dev/pts/)[0-9]+").unwrap();
	};

	for dir in read_dir("/dev/pts/").unwrap() {
		let file = dir.unwrap().path().display().to_string();
		if RE.is_match(&file) {
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
	Command::new("xrdb")
		.args(["-merge", "-quiet", "/home/autumn/.cache/wal/colors.Xresources"])
		.spawn()
		.expect("xrdb merge failed");
}
