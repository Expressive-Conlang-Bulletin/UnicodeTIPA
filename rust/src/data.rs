use std::fs::File;
use std::io::{BufReader, Write, stderr};
use crate::serde_json::{Value, Map, from_reader};

const READ_DATA_FAIL: &[u8] = b"Failed to read unicode-tipa mapping data from JSON files.\r\nMake sure you have those JSON files in the current directory when calling the binary.\r\n";

pub fn read_letters() -> Map<String, Value>{
	let file_open = File::open("data/letters.json");
	let file = match file_open {
		Ok(f) => f,
		e => {
			stderr().write_all(READ_DATA_FAIL).unwrap();
			e.unwrap()
		}
	};
    let reader = BufReader::new(file);
	let json = from_reader(reader).unwrap();
	json
}

pub fn read_marks() -> Map<String, Value>{
	let file_open = File::open("data/marks.json");
	let file = match file_open {
		Ok(f) => f,
		e => {
			stderr().write_all(READ_DATA_FAIL).unwrap();
			e.unwrap()
		}
	};
    let reader = BufReader::new(file);
	let json = from_reader(reader).unwrap();
	json
}

pub fn read_groups() -> Map<String, Value>{
	let file_open = File::open("data/groups.json");
	let file = match file_open {
		Ok(f) => f,
		e => {
			stderr().write_all(READ_DATA_FAIL).unwrap();
			e.unwrap()
		}
	};
    let reader = BufReader::new(file);
	let json = from_reader(reader).unwrap();
	json
}

pub fn read_combineleft() -> Map<String, Value>{
	let file_open = File::open("data/combineleft.json");
	let file = match file_open {
		Ok(f) => f,
		e => {
			stderr().write_all(READ_DATA_FAIL).unwrap();
			e.unwrap()
		}
	};
    let reader = BufReader::new(file);
	let json = from_reader(reader).unwrap();
	json
}

pub fn read_combineboth() -> Map<String, Value>{
	let file_open = File::open("data/combineboth.json");
	let file = match file_open {
		Ok(f) => f,
		e => {
			stderr().write_all(READ_DATA_FAIL).unwrap();
			e.unwrap()
		}
	};
    let reader = BufReader::new(file);
	let json = from_reader(reader).unwrap();
	json
}

// combinators.json
// groups.json
// marks.json
// letters.json