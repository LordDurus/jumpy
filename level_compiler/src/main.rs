mod binary_writer;
mod compile;
mod entity_parse_state;
mod layer_parse_state;
mod message_registry;
mod runtime;
mod source;
mod text_parse;
mod trigger_parse_state;

use std::{env, fs, io::Write, path::Path};

use crate::{compile::compile_and_serialize, text_parse::load_level_from_str};

fn main() {
	let mut args = env::args().skip(1);

	let input_path = match args.next() {
		Some(p) => p,
		None => {
			eprintln!("usage: level_compiler <input.level> [output.lvlb]");
			std::process::exit(1);
		}
	};

	let output_path = match args.next() {
		Some(p) => p,
		None => replace_extension(&input_path, "lvlb"),
	};

	if input_path == output_path {
		eprintln!("input and output paths must be different");
		std::process::exit(1);
	}

	let path = Path::new(&input_path);
	if !path.exists() {
		eprintln!("File {} Not Found.", input_path);
		std::process::exit(1);
	}

	let text = match fs::read_to_string(&input_path) {
		Ok(t) => t,
		Err(e) => {
			eprintln!("failed to read {}: {}", input_path, e);
			std::process::exit(2);
		}
	};

	let level_source = match load_level_from_str(&text) {
		Ok(l) => l,
		Err(e) => {
			eprintln!("parse error in {}: {}", input_path, e);
			std::process::exit(3);
		}
	};

	let bytes = match compile_and_serialize(&level_source) {
		Ok(b) => b,
		Err(e) => {
			eprintln!("compile error in {}: {}", input_path, e);
			std::process::exit(4);
		}
	};

	let write_result = fs::File::create(&output_path).and_then(|mut f| {
		let result = f.write_all(&bytes);
		return result;
	});

	match write_result {
		Ok(_) => {
			println!("wrote {}", output_path);
		}
		Err(e) => {
			eprintln!("failed to write {}: {}", output_path, e);
			std::process::exit(5);
		}
	}
}

fn replace_extension(path: &str, new_ext: &str) -> String {
	let p = Path::new(path);

	let stem = match p.file_stem() {
		Some(s) => s.to_string_lossy().to_string(),
		None => path.to_string(),
	};

	let parent = match p.parent() {
		Some(pp) => pp.to_path_buf(),
		None => std::path::PathBuf::from("."),
	};

	let mut new_name = String::new();
	new_name.push_str(&stem);
	new_name.push('.');
	new_name.push_str(new_ext);

	let combined = parent.join(new_name);
	let result = combined.to_string_lossy().to_string();
	return result;
}
