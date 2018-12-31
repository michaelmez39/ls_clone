use std::fs;
use termion::color;

struct Multicolumn {
	terminal_x: usize,
	columns: usize,
	width: usize
}

impl Multicolumn {
	fn new(terminal_size: (u16, u16), max_width: usize) -> Multicolumn {
		Multicolumn{
			terminal_x : terminal_size.0 as usize,
			columns : terminal_size.0 as usize / max_width,
			width : terminal_size.0 as usize/ (terminal_size.0 as usize / max_width),
		}
	}

}

pub fn unicolumn(vals: Vec<fs::DirEntry>) 
	{
		for val in vals {
			match val.file_type().unwrap().is_dir() {
				true => println!("{}{}{}", color::Fg(color::Blue), val.path().to_str().expect("something really fucked up"), color::Fg(color::Reset)),
				false => println!("{}", val.path().to_str().expect("something really fucked up")),
			}
		}
	}

// {:-^1$}
pub fn multicolumn(vals: Vec<fs::DirEntry>)
	{
		use Multicolumn;
		let mut max = 0;
		for val in &vals {
			let m = val.path().to_str().unwrap().len();
			if  m > max {
				max = m
			}
		}
		let column_data = Multicolumn::new(termion::terminal_size().unwrap(), (max + 2));
		let correct = column_data.terminal_x - (column_data.width * column_data.columns);
		println!("---- {}, {}, {} ----", column_data.width, column_data.columns, correct);
		for (i, val) in vals.iter().enumerate() {
			//println!(" ----- {}, {}, {}", columns, column_width, val.path().to_str().unwrap().len());
		 	let padding = column_data.width - val.path().to_str().unwrap().len();
		 	// print!("{}", val.path().to_str().unwrap());
		 	match val.file_type().unwrap().is_dir() {
				true => print!("{}{}", color::Fg(color::Blue), val.path().to_str().unwrap()),
				false => print!("{}{}",  color::Fg(color::Reset), val.path().to_str().unwrap()),
			}
			

			if i % column_data.columns == 0 {
				for _ in 0..(padding+correct) {
					print!(" ");
				}
			} else {
				for _ in 0..padding {
					print!(" ");
				}
			}
			// match val.file_type().unwrap().is_dir() {
			// 	true => println!("{}{: <1$}{}", color::Fg(color::Blue), f, (width - f.len()), color::Fg(color::Reset)),
			// 	false => println!("{: <1$}", f, (width - f.len())),
			// }
		}
	}