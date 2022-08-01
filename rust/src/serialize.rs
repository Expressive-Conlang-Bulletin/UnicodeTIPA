use std::string::ToString;
use crate::{Config, CatenateMethod};
use crate::tokenize::{Token, Token::*, TokenSequence};

impl Token {
	pub fn to_string(self, conf: &Config) -> String {
		match self {
			Letter(l) => {l},
			Mark(m) => {m},
			// Should eliminate unknown before this according to config.
			Unknown(u) => {
				String::new()
			},
			AppliedMacro(c, ts) => {
				let mut s = String::from(c);
				s.push_str(&(ts.to_string(conf)));
				s
			},
			CombineLeft(c) => {
				unreachable!("`CombineLeft` should not appear here, as combination should be done.")
			},
			CombineBoth(c) => {
				unreachable!("`CombineBoth` should not appear here, as combination should be done.")
			},
			Tone(t) => {
				unreachable!("`Tone` should not appear here, as conversion to `Mark` should be done.")
			}
		}
	}
}

impl TokenSequence {
	pub fn to_string(self, conf: &Config) -> String {
		match &conf.cat {
			Bracket => {
				let mut s = String::from("{");
				for t in self.0 {
					s.push_str(&t.to_string(conf));
					s.push_str("}{")
				};
				s.push_str("}");
				s
			},
			Space => {
				unimplemented!()
			}
		}
	}
}