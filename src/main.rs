#![feature(uniform_paths)]

extern crate clap;
extern crate termion;

use clap::{App, Arg};

mod list_modifiers;
use list_modifiers::{unicolumn, multicolumn};

use std::fs;
use std::env;


fn main() {
	const VERSION: &'static str = env!("CARGO_PKG_VERSION");
	let matches = App::new("List")
						.version(VERSION)
						.author("Michael Mezzina")
						.about("List files in a directory")
						.arg(Arg::with_name("all")
								.short("a")
								.long("all")
								.help("lists all files in directory, e.g. hidden files \".hidden\"")
							)
						.arg(Arg::with_name("PATH")
                               .help("Sets the path to list")
                               .required(false)
                               .index(1))
	;
	//termion::terminal_size()

	let args: Vec<String> = env::args().collect();
	// println!("{}", args[0]);
	// next step: parse argument by absolute or relative path

    match args.len() {
    	1 => {
    		multicolumn(fs::read_dir(env::current_dir().unwrap()).unwrap().map(|x| x.unwrap()).collect());
    	},
    	2 => {
    		multicolumn(fs::read_dir(&args[1]).unwrap().map(|x| x.unwrap()).collect());
    	},
    	_ => println!("Something went wrong, expected 0 or 1 arguments, instead got {} arguments", (args.len() - 1))
    }
	println!();
}
