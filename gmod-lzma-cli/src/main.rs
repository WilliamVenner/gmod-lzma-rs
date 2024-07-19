use std::{fs::File, io::Write, path::PathBuf};

struct Options {
	operation: Operation,
	input: PathBuf,
	output: PathBuf,
}
impl Options {
	fn get() -> Option<Self> {
		let mut args = std::env::args_os().skip(1).peekable();
		let args_len = args.len();

		Some(Self {
			operation: match (args_len, args.next()) {
				(3 | 5, Some(arg))
					if arg.eq_ignore_ascii_case("-c") || arg.eq_ignore_ascii_case("--compress") =>
				{
					Operation::Compress {
						level: if args_len == 5 {
							let arg = args.next().unwrap();
							if !arg.eq_ignore_ascii_case("-l")
								&& !arg.eq_ignore_ascii_case("--level")
							{
								return None;
							}

							let level = args.next().unwrap().to_str()?.parse().ok()?;
							if level > 9 {
								return None;
							}

							level
						} else {
							9
						},
					}
				}

				(3, Some(arg))
					if arg.eq_ignore_ascii_case("-d")
						|| arg.eq_ignore_ascii_case("--decompress") =>
				{
					Operation::Decompress
				}

				_ => return None,
			},

			input: PathBuf::from(args.next().unwrap()),
			output: PathBuf::from(args.next().unwrap()),
		})
	}
}

enum Operation {
	Compress { level: u8 },
	Decompress,
}

fn main() {
	let Options {
		operation,
		input,
		output,
	} = match Options::get() {
		Some(options) => options,
		None => {
			let exe = std::env::current_exe().ok();
			eprintln!(
				"Usage: {exe} (-c | --compress) [-l | --level <level>] <input> <output>\n       {exe} (-d | --decompress) <input> <output>",
				exe = exe.as_deref()
					.and_then(|exe| exe.file_stem().and_then(|exe| exe.to_str()))
					.unwrap_or("gmod-lzma")
			);
			std::process::exit(1);
		}
	};

	let input = match std::fs::read(&input) {
		Ok(input) => input,
		Err(err) => {
			eprintln!("Failed to read input file: {}", err);
			std::process::exit(1);
		}
	};

	let mut output = match File::create(output) {
		Ok(output) => output,
		Err(err) => {
			eprintln!("Failed to create output file: {}", err);
			std::process::exit(1);
		}
	};

	let input = match operation {
		Operation::Compress { level } => match gmod_lzma::compress(&input, level as _) {
			Ok(compressed) => compressed,
			Err(err) => {
				eprintln!("Failed to compress input data: LZMA error {}", err);
				std::process::exit(1);
			}
		},

		Operation::Decompress => match gmod_lzma::decompress(&input) {
			Ok(decompressed) => decompressed,
			Err(err) => {
				eprintln!("Failed to decompress input data: LZMA error {}", err);
				std::process::exit(1);
			}
		},
	};

	if let Err(err) = output.write_all(&input) {
		eprintln!("Failed to write to output file: {}", err);
		std::process::exit(1);
	}
}
