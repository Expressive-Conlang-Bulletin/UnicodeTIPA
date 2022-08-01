use std::fs::File;
use std::io::BufReader;
use crate::serde_json::{Value, Map, from_reader};

pub fn read_letters() -> Map<String, Value>{
	let file = File::open("data/letters.json").unwrap();
    let reader = BufReader::new(file);
	let json = from_reader(reader).unwrap();
	json
}

pub fn read_marks() -> Map<String, Value>{
	let file = File::open("data/marks.json").unwrap();
    let reader = BufReader::new(file);
	let json = from_reader(reader).unwrap();
	json
}

pub fn read_groups() -> Map<String, Value>{
	let file = File::open("data/groups.json").unwrap();
    let reader = BufReader::new(file);
	let json = from_reader(reader).unwrap();
	json
}

pub fn read_combineleft() -> Map<String, Value>{
	let file = File::open("data/combineleft.json").unwrap();
    let reader = BufReader::new(file);
	let json = from_reader(reader).unwrap();
	json
}

pub fn read_combineboth() -> Map<String, Value>{
	let file = File::open("data/combineboth.json").unwrap();
    let reader = BufReader::new(file);
	let json = from_reader(reader).unwrap();
	json
}

// combinators.json
// groups.json
// marks.json
// letters.json