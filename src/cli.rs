use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static>{
	const VERSION: &'static str = env!("CARGO_PKG_VERSION");
	App::new("List")
		.version(VERSION)
		.author("Michael Mezzina")
		.about("List files in a directory")
		.arg(Arg::with_name("all")
				.short("a")
				.long("all")
				.help("lists all files in directory, e.g. hidden files \".gitignore\"")
			)
		.arg(Arg::with_name("path")
				.multiple(true)	
                .help("Enter one or more paths to list their contents")
                .required(false)
                .index(1))
}