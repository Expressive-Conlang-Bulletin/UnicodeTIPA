use crate::kmp::kmp_find;
use crate::serde_json::{Map, Value};
use crate::tokenize::{Token, Token::*};

fn parse_group_key(str: &String) -> Vec<Token> {
	str.split(',').map(|s| Unknown(s.parse::<u32>().unwrap())).collect()
}

fn parse_group_val(str: &str) -> Token {
	//Now support and assume letters only.
	Letter(str.to_owned())
}

pub fn replace_groups(mut tokens: Vec<Token>, dict: &Map<String, Value>) -> Vec<Token> {
	let tokensref = &mut tokens;
	for (s, v) in dict {
		let key = parse_group_key(s);

		let find = kmp_find(key.as_slice(), tokensref.as_slice());
		match find {
			None => {},
			Some(res) => {
				let len = key.len();
				let val = parse_group_val(v.as_str().unwrap());
				// tokensref.drain(res..res+len).last();
				// tokensref.insert(res, val)
				tokensref.splice(res..res+len, [val]).last();
			}
		};
	};
	tokens
}