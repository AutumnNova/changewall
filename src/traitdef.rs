use clap::ArgMatches;

#[derive(Clone)]
pub struct AppOpt {
	pub alpha: u8,
	pub nocache: bool,
	pub skip: String,
	pub vte: bool,
	pub writeseq: bool,
}

impl AppOpt {
	pub fn grabargs(args: &ArgMatches) -> Self { Self { alpha: *args.get_one::<u8>("alpha").unwrap().clamp(&0, &100), nocache: args.get_flag("nocache"), skip: args.get_one::<String>("skip").unwrap().to_string() , vte: args.get_flag("vte"), writeseq: args.get_flag("writeseq") } }
}
