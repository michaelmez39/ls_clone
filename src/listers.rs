use std::fs;
use std::path::PathBuf;
use termion::color;


struct Multicolumn {
	columns: usize,
	width: usize,
	correct: usize
}

impl Multicolumn {
	fn new(terminal_size: (u16, u16), max_width: usize) -> Multicolumn {
		Multicolumn{
			columns : terminal_size.0 as usize / max_width,
			width : terminal_size.0 as usize/ (terminal_size.0 as usize / max_width),
			correct : terminal_size.0 as usize- ((terminal_size.0 as usize/ (terminal_size.0 as usize / max_width)) * ( terminal_size.0 as usize / max_width))
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
// {:-^1$}
// fn get_str(path: DirEntry) -> &str {
// 	path.path().file_name().unwrap().to_str().unwrap()
// }
pub fn multicolumn(path: &PathBuf)
	{
		use Multicolumn;
		// parse the path to get a Vec<DirEntry>
	
		let children_paths: Vec<PathBuf> = parse_path(path).unwrap();
		// Find the longest path name in the directory
		let max = children_paths.iter().map(|string| string.file_name().unwrap().to_str().unwrap().len()).max().unwrap();

		// Making a new Multicolumn figures out the column width, number of columns, and the correction that needs to be printed to completely fill the terminal
		let column_data = Multicolumn::new(termion::terminal_size().unwrap(), max + 2);

		for (i, val) in children_paths.iter().enumerate() {
		 	let padding = column_data.width - val.file_name().unwrap().to_str().unwrap().len();

			match val.metadata().unwrap().file_type().is_dir() {
				true => print!("{}{}", color::Fg(color::Blue), val.file_name().unwrap().to_str().unwrap()),
				false => print!("{}{}", color::Fg(color::Reset), val.file_name().unwrap().to_str().unwrap()),
			}
		 	
			
			// if we are at the last column, we need to print a number of spaces to keep the columns aligned
			if i % column_data.columns == 0 {
				for _ in 0..(padding+column_data.correct) {
					print!(" ");
				}
			} else {
			// if not we just print spaces to make the column the correct size
				for _ in 0..padding {
					print!(" ");
				}
			}

		}
		println!("{}\n", color::Fg(color::Reset));
	}