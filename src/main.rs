use structopt::StructOpt;
use std::{process::Command, thread, time, fs};
use notify_rust::{Notification, Urgency};

#[derive(StructOpt)]
struct Cli {
	//path of wallpaper
	path: String,
	#[structopt(default_value = "fill")]
	setting: String
}

fn main() {
	let args = Cli::from_args();

	Command::new("wal")
		.args(["-tnqi", &args.path])
		.spawn()
		.expect("wal failed");

	Command::new("feh")
		.args(["--no-fehbg", &format!("--bg-{}", args.setting), &fs::read_to_string("/home/autumn/.cache/wal/wal").expect("failed to read file")])
		.spawn()
		.expect("feh failed");

	dunst();	
}

fn dunst() {
	for prc in procfs::process::all_processes().unwrap() {
		if  prc.stat.comm == "dunst" {
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