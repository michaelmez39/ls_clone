use std::fs;
use std::path::PathBuf;
use termion::color;
use std::fmt::Display;


pub struct Multicolumn {
	inner_directory: Vec<PathBuf>,
	columns: usize,
	width: usize,
	correct: usize
}


impl<'a> Multicolumn {
	pub fn new(terminal_size: (u16, u16), path: &'a PathBuf) -> Multicolumn {
		let children_paths: Vec<PathBuf> = parse_path(path).unwrap();
		let max = children_paths.iter().map(|string| string.file_name().unwrap().to_str().unwrap().len()).max().unwrap() + 2;
		Multicolumn{
			inner_directory: children_paths,
			columns : terminal_size.0 as usize / max,
			width : terminal_size.0 as usize/ (terminal_size.0 as usize / max),
			correct : terminal_size.0 as usize- ((terminal_size.0 as usize/ (terminal_size.0 as usize / max)) * ( terminal_size.0 as usize / max))
		}
	}

}

// This isn't used right now because it isn't needed yet
// for displaying file information and such this could be used later
// pub fn unicolumn(vals: Vec<fs::DirEntry>) 
// 	{
// 	for val in vals {
// 		match val.file_type().unwrap().is_dir() {
// 			true => println!("{}{}{}", color::Fg(color::Blue), val.path().to_str().expect("something really fucked up"), color::Fg(color::Reset)),
// 			false => println!("{}", val.path().to_str().expect("something really fucked up")),
// 	}
// 	}
// 	}

impl<'a> Display for Multicolumn {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>{
		for (i, val) in self.inner_directory.iter().enumerate() {
		 	let padding = self.width - val.file_name().unwrap().to_str().unwrap().len();

			match val.metadata().expect("File error: incorrect metadata").file_type().is_dir() {
				true => write!(f, "{}{}", color::Fg(color::Blue), val.file_name().unwrap().to_str().unwrap())?,
				false => write!(f, "{}{}", color::Fg(color::Reset), val.file_name().unwrap().to_str().unwrap())?,
			}

			if i % self.columns == 0 {
					for _ in 0..(padding+self.correct) {
						write!(f, " ")?;
					}
				} else {
					for _ in 0..padding {
						write!(f, " ")?;
					}
			}
		}
	Ok(())
	}
}

pub fn parse_path(path: &PathBuf) -> Option<Vec<PathBuf>>{
	Some(fs::read_dir(path)
			.expect("invalid path entered")
			.map(|val| {val
							.unwrap()
							.path()
							// .file_name().unwrap()
							// .to_str().unwrap()
							// .to_string()
			}).collect::<Vec<PathBuf>>())
}