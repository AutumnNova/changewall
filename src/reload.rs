use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, process::Command, thread, time};
use notify_rust::{Notification, Urgency};


pub fn reload(seq:String) {
	pts(seq);
	polybar();
	xrdb();
	dunst();
}

fn dunst() {
	for prc in procfs::process::all_processes().unwrap() {
		if  prc.stat.comm == "dunst" || prc.stat.comm == "/usr/bin/dunst" {
			nix::sys::signal::kill(prc.stat.pid, nix::sys::signal::Signal::SIGKILL)
			.expect("SIGTERM failed")
		}
	}
	thread::sleep(time::Duration::from_millis(1));

	Notification::new()
		.summary("pywal")
		.body("Reloaded wal configurations!")
		.urgency(Urgency::Normal)
		.show()
		.unwrap();
}

fn pts(seq:String) {

	lazy_static!{
		static ref RE:Regex = Regex::new("(/dev/pts/)[0-9]+").unwrap();
	};

	for dir in fs::read_dir("/dev/pts/").unwrap() {
		let file = dir.unwrap().path().display().to_string();
		if RE.is_match(&file) {
			fs::write(file, &seq)
				.expect("write to /dev/pts failed.");
		}
	}
}

fn polybar() {
	for prc in procfs::process::all_processes().unwrap() {
		if  prc.stat.comm == "polybar" {
			nix::sys::signal::kill(prc.stat.pid, nix::sys::signal::Signal::SIGUSR1)
			.expect("SIGUSR1 failed")
		}
	}
}

fn xrdb() {
	Command::new("xrdb")
		.args(["-merge", "-quiet", "/home/autumn/.cache/wal/colors.Xresources"])
		.spawn()
		.expect("xrdb merge failed");
}