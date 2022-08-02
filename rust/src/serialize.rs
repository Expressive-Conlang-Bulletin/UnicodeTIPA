use crate::regex::Regex;
use crate::{Config, CatenateMethod::*};
use crate::tokenize::{Token, Token::*, TokenSequence};

impl Token {
	pub fn to_string(self, conf: &Config) -> String {
		match self {
			Letter(l) => {l},
			Mark(m) => {m},
			// Should eliminate unknown before this according to config.
			Unknown(_) => {
				String::new()
			},
			AppliedMacro(c, ts) => {
				let mut s = String::from(c);
				s.push('{');
				s.push_str(&(ts.to_string(conf)));
				s.push('}');
				s
			},
			CombineLeft(_) => {
				unreachable!("`CombineLeft` should not appear here, as combination should be done.")
			},
			CombineBoth(_) => {
				unreachable!("`CombineBoth` should not appear here, as combination should be done.")
			},
			Tone(_) => {
				unreachable!("`Tone` should not appear here, as conversion to `Mark` should be done.")
			}
		}
	}
}

impl TokenSequence {
	pub fn to_string(self, conf: &Config) -> String {
		match &conf.cat {
			Bracket => {
				let mut s = String::new();
				for t in self.0 {
					s.push('{');
					s.push_str(&t.to_string(conf));
					s.push('}')
				};
				s
			},
			Space => {
				let mut s = String::new();
				for t in self.0 {
					s.push_str(&t.to_string(conf));
					s.push(' ')
				};
				s.pop();
				s
			},
			LessSpace => {
				to_string_less_space(self.0, conf)
			}
		}
	}
}

fn to_string_less_space(tokens: Vec<Token>, conf: &Config) -> String {
	// eprintln!("{tokens:?}");
	let mut res = String::new();
	let mut is_ctrl_seq = false;
	for t in tokens {
		let token_str = t.to_string(conf);
		if is_ctrl_seq && is_start_from_csname(&token_str) {
			res.push(' ')
		}
		is_ctrl_seq = is_waiting_for_csname(&token_str);
		res.push_str(&token_str)
	};
	res
}

fn is_waiting_for_csname(s: &String) -> bool {
	// pattern should be determined by TeX category codes(which may change), but let it go.
	let re = Regex::new("^.*\\\\[^\\\\\\{\\s},;:]*$").unwrap();
	let res = re.is_match(s);
	// eprintln!("{s}\n waiting csname? {res}");
	res
}

fn is_start_from_csname(s: &String) -> bool {
	// pattern should be determined by TeX category codes(which may change), but let it go.
	// not support chinese char
	let re = Regex::new("^\\p{L}.*$").unwrap();
    let res = re.is_match(s);
	// eprintln!("{s}\n start from csname? {res}");
	res
}