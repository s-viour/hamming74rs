extern crate hamming74;


use std::env;
use std::io::{self, Write};

use hamming74::{H74Encoder, H74Decoder};


fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();
	let stdin = io::stdin();
	let mut stdout = io::stdout();

	if args.len() < 2 {
		println!("not enough arguments supplied");
		return Ok(());
	}

	match args[1].as_str() {
		"encode" => {
			let h = H74Encoder::new(stdin);
			for byte in h {
				stdout.write(&[byte])?;
			}
		},
		"decode" => {
			let h = H74Decoder::new(stdin);
			for byte in h {
				stdout.write(&[byte])?;
			}
		},
		_ => {
			println!("command not understood. options are [encode|decode]");
		}
	}

    Ok(())
}

