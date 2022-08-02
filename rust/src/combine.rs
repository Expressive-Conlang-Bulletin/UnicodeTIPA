use crate::tokenize::{Token, Token::*, TokenSequence};
use CombineStatus::*;

#[derive(Debug)]
enum CombineStatus {
	Relax,
	WaitingCombinator,
	CombiningRight
}

pub fn combine_tokens(tokens: Vec<Token>) -> Vec<Token> {
	let mut relax_stack: Vec<Token> = Vec::new();
	let mut combine_stack: Vec<Token> = Vec::new();
	// We need None here in fact, but let it go.
	let mut current_combinator: Token = Unknown(0);
	let mut status: CombineStatus = Relax;
	for token in tokens {
		match token {
			Letter(_)|AppliedMacro(_, _) => {
				match status {
					Relax => {
						combine_stack.push(token);
						status = WaitingCombinator
					},
					WaitingCombinator => {
						relax_stack.append(&mut combine_stack);
						combine_stack.push(token)
					},
					CombiningRight => {
						combine_stack.push(token);
						let mut seq = Vec::new();
						match current_combinator.clone() {
							CombineLeft(c)|CombineBoth(c) => {
								seq.append(&mut combine_stack);
								combine_stack.push(AppliedMacro(c, TokenSequence(seq)));
								current_combinator = Unknown(0)
							},
							_ => unreachable!("`Status` is `CombiningRight`, but `current_combinator` is not a combinator token.")
						};
						status = WaitingCombinator
					}
				}
			},
			Tone(_) => {unreachable!("Found `Tone` tokens after tone handling. Seems tokenize incomplete.")},
			CombineBoth(_) => {
				match status {
					Relax => {
						unreachable!("Found `CombineBoth` not behind any letters. Cannot Combine.")
					},
					WaitingCombinator => {
						current_combinator = token;
						status = CombiningRight
					},
					CombiningRight => {
						unreachable!("Found `CombineBoth` not behind any letters, but just behind another combinator. Cannot Combine.")
					}
				}
			},
			CombineLeft(c) => {
				match status {
					Relax => {
						unreachable!("Found `CombineLeft` not behind any letters. Cannot Combine.")
					},
					WaitingCombinator => {
						let mut seq = Vec::new();
						match current_combinator.clone() {
							CombineLeft(_)|CombineBoth(_) => {
								unreachable!("Another combinator found when `current_combinator` is a valid combinator token.")
							},
							_ => {
								seq.append(&mut combine_stack);
								let applied = AppliedMacro(c, TokenSequence(seq));
								combine_stack.push(applied);
								current_combinator = Unknown(0)
							}
						};
						status = WaitingCombinator
					},
					CombiningRight => {
						unreachable!("Found `CombineBoth` not behind any letters. Cannot Combine.")
					}
				}
			},
			Mark(_) => {
				match status {
					Relax => {
						relax_stack.push(token)
					},
					WaitingCombinator|CombiningRight => {
						combine_stack.push(token)
					}
				}
			},
			Unknown(u) => {
				eprintln!("Not implemented. Skip unknown tokwn {u}")
			}
		};
	};
	relax_stack.append(&mut combine_stack);
	relax_stack
}