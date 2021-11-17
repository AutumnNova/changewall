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
	pub fn grabargs(args: &ArgMatches) -> Self { Self { alpha: args.value_of_t::<u8>("alpha").unwrap().min(100).max(0), nocache: args.is_present("nocache"), skip: args.value_of_t::<String>("skip").unwrap() , vte: args.is_present("vte"), writeseq: args.is_present("writeseq") } }
}
