use crate::{ToneMethod::*, tokenize::{Token, Token::*}};

// May be good to restart `LongTone' as `Tone not stringfied' and stringfy it in a future stage.

pub fn replace_tone(tokens: Vec<Token>, conf: &crate::Config) -> Vec<Token> {
	let mut vec = Vec::new();
	let v = &mut vec;
	match conf.tone {
		Ligature => {
			let mut mem: Vec<&str> = Vec::new();
			for token in tokens {
				match token {
					Tone(t) => {mem.push(t)},
					t => {
						match mem.len() {
							0 => {},
							1 => {
								let mut long_tone = String::from("\\tone{");
								for c in &mem {
									long_tone.push_str(c);
								};
								let uniq = mem.get(0).unwrap();
								long_tone.push_str(uniq);
								long_tone.push('}');
								mem.clear();
								v.push(Mark(long_tone));
							},
							_ => {
								let mut long_tone = String::from("\\tone{");
								for c in &mem {
									long_tone.push_str(c);
								};
								mem.clear();
								long_tone.push('}');
								v.push(Mark(long_tone));
							}
						};
						v.push(t)
					}
				};
			};
			//maybe oneday reduce the repeated codes
			match mem.len() {
				0 => {},
				1 => {
					let mut long_tone = String::from("\\tone{");
					for c in &mem {
						long_tone.push_str(c);
					};
					let uniq = mem.get(0).unwrap();
					long_tone.push_str(uniq);
					long_tone.push('}');
					mem.clear();
					v.push(Mark(long_tone));
				},
				_ => {
					let mut long_tone = String::from("\\tone{");
					for c in &mem {
						long_tone.push_str(c);
					};
					mem.clear();
					long_tone.push('}');
					v.push(Mark(long_tone));
				}
			};
		},
		Catenate => {
			let mut mem: Vec<&str> = Vec::new();
			for token in tokens {
				match token {
					Tone(t) => {mem.push(t)},
					t => {
						let mut long_tone = String::from("\\tone{");
						for c in &mem {
							long_tone.push_str(c);
							long_tone.push_str(c);
						};
						long_tone.push('}');
						mem.clear();
						v.push(Mark(long_tone));
						v.push(t)
					}
				};
			};
			
		},
		Keep => {
			for token in tokens {
				let replaced = match token {
					Tone("1") => {Mark(String::from("\\tone{11}"))},
					Tone("2") => {Mark(String::from("\\tone{22}"))},
					Tone("3") => {Mark(String::from("\\tone{33}"))},
					Tone("4") => {Mark(String::from("\\tone{44}"))},
					Tone("5") => {Mark(String::from("\\tone{55}"))},
					t => t
				};
				v.push(replaced);
			}
		}
	};
	vec
}