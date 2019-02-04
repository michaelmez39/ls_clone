#![feature(uniform_paths)]

mod listers;
mod cli;

use std::env;
use termion::style;

fn main() {
	//termion::terminal_size()
	let matches = cli::build_cli().get_matches_safe().unwrap_or_else(|e| e.exit());
	let paths = match matches.values_of("path") {
		None => vec!(env::current_dir().expect("there is not a current directory")),
		Some(i) => i.map(|path| std::path::PathBuf::from(path)).collect::<Vec<std::path::PathBuf>>()
	};
	
	match paths.len() {
		1 => {
			print!("{}", style::Bold);
			listers::multicolumn(&paths[0])
			},
		_ => {
				paths.iter().for_each(|path| {
					print!("{}", style::Reset);
					println!("{}:", path.display());
					print!("{}", style::Bold);
					listers::multicolumn(path)
				});
			},
	}
}
