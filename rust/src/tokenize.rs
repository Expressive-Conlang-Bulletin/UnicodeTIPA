use crate::serde_json::{Value, Map};
use crate::data::*;
use Token::*;

pub const TONE1: &str = "1";
pub const TONE2: &str = "2";
pub const TONE3: &str = "3";
pub const TONE4: &str = "4";
pub const TONE5: &str = "5";

#[derive(PartialEq, Debug)]
pub enum Token {
	Letter(String),
	Mark(String),
	CombineLeft(String),
	CombineBoth(String),
	Unknown(u32),
	Tone(&'static str)
	// ,LongTone(String)
}

pub fn tokenize(u: u32
	, letter_map: &Map<String, Value>
	, mark_map: &Map<String, Value>
	, combl_map: &Map<String, Value>
	, comblr_map: &Map<String, Value>
) -> Token {
	match u {
		32 => Mark(String::from("\\,")),
		10 => Mark(String::from("\\\\")),
		9 => Mark(String::from("\\quad")),
		741 => Tone(TONE5),
		742 => Tone(TONE4),
		743 => Tone(TONE3),
		744 => Tone(TONE2),
		745 => Tone(TONE1),
		_ => {
			let key = char::try_from(u).unwrap().to_string();
			match letter_map.get(&key) {
				Some(s) => Letter(s.as_str().unwrap().to_owned()),
				None => match mark_map.get(&key) {
					Some(s) => Mark(s.as_str().unwrap().to_owned()),
					None => match combl_map.get(&key) {
						Some(s) => CombineLeft(s.as_str().unwrap().to_owned()),
						None => match comblr_map.get(&key) {
							Some(s) => CombineBoth(s.as_str().unwrap().to_owned()),
							None => Unknown(u)
						}
					}
				}
			}
		}
	}
}

pub fn tokenize_str(str: String) -> Vec<Token> {
	let l = &read_letters();
	let m = &read_marks();
	let cl = &read_combineleft();
	let cb = &read_combineboth();
	str.chars()
		.map(|c| tokenize(c as u32, l, m, cl, cb))
		.collect()
}