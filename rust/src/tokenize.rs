use crate::serde_json::{Value, Map};
use crate::data::*;
use crate::group::replace_groups;
use Token::*;

pub const TONE1: &str = "1";
pub const TONE2: &str = "2";
pub const TONE3: &str = "3";
pub const TONE4: &str = "4";
pub const TONE5: &str = "5";

#[derive(PartialEq, Debug, Clone)]
pub struct TokenSequence(pub Vec<Token>);

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
	Letter(String),
	Mark(String),
	CombineLeft(String),
	CombineBoth(String),
	Unknown(u32),
	Tone(&'static str),
	//LongTone(String),
	AppliedMacro(String, TokenSequence)
}

pub fn tokenize(t: Token
	, letter_map: &Map<String, Value>
	, mark_map: &Map<String, Value>
	, combl_map: &Map<String, Value>
	, comblr_map: &Map<String, Value>
) -> Token {
	match t {
		Unknown(32) => Mark(String::from("\\, ")),
		Unknown(10) => Mark(String::from("\\\\ ")),
		Unknown(9) => Mark(String::from("\\quad ")),
		Unknown(126) => Mark(String::from("\\textasciitilde")),
		// in principle, these two should not appear except small a~z. I'm lazy to support them.
		Unknown(l)
			if 48 <= l && l <= 57 || 64 <= l && l <= 90 || 97 <= l && l <= 122 =>
			Letter(char::try_from(l).unwrap().to_string()),
		Unknown(m) 
			if 91 <= m && m <= 96 || 123 <= m && m <= 126 =>
			Mark(char::try_from(m).unwrap().to_string()),
		Unknown(741) => Tone(TONE5),
		Unknown(742) => Tone(TONE4),
		Unknown(743) => Tone(TONE3),
		Unknown(744) => Tone(TONE2),
		Unknown(745) => Tone(TONE1),
		Unknown(u) => {
			let key = u.to_string();
			match letter_map.get(&key) {
				Some(s) => {
					Letter(s.as_str().unwrap().to_owned())
				},
				None => {
					match mark_map.get(&key) {
						Some(s) => Mark(s.as_str().unwrap().to_owned()),
						None => match combl_map.get(&key) {
							Some(s) => CombineLeft(s.as_str().unwrap().to_owned()),
							None => match comblr_map.get(&key) {
								Some(s) => CombineBoth(s.as_str().unwrap().to_owned()),
								None => t
							}
						}
					}
				}
			}
		},
		other => other
	}
}

// pub fn tokenize_str(str: String) -> Vec<Token> {
// 	let l = &read_letters();
// 	let m = &read_marks();
// 	let cl = &read_combineleft();
// 	let cb = &read_combineboth();
// 	str.chars()
// 		.map(|c| tokenize(Unknown(c as u32), l, m, cl, cb))
// 		.collect()
// }

pub fn tokenize_withgrp_str(str: String) -> Vec<Token> {
	let l = &read_letters();
	let m = &read_marks();
	let cl = &read_combineleft();
	let cb = &read_combineboth();
	let g = &read_groups();
	let unknowns = replace_groups(str.chars().map(|c| Unknown(c as u32)).collect(), g);
	unknowns.iter().map(|t|
		tokenize(t.to_owned(), l, m, cl, cb)
	).collect()
}