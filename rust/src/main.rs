extern crate serde_json;
extern crate kmp;
extern crate regex;
extern crate clap;

use tokenize::{TokenSequence, tokenize_withgrp_str};
use tone::replace_tone;
use combine::combine_tokens;
use CatenateMethod::*;
use ToneMethod::*;

use clap::{Parser, ValueEnum};

mod data;
mod tokenize;
mod tone;
mod group;
mod combine;
mod serialize;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, arg_enum, default_value_t = LessSpace)]
    cat: CatenateMethod,
    #[clap(short, long, arg_enum, default_value_t = Ligature)]
    tone: ToneMethod,
    input: String
}

pub struct Config {
    tone: ToneMethod,
    cat: CatenateMethod
    //, unknown: UnknownHandle
}

pub enum UnknownHandle {
    NotImplemented
}

#[derive(Clone, ValueEnum)]
pub enum CatenateMethod {
    Bracket,
    Space,
    LessSpace
}

#[derive(Clone, ValueEnum)]
pub enum ToneMethod {
    Ligature,
    Catenate,
    Keep
}

fn main() {
    let cli = Cli::parse();
    let input = cli.input;
    let conf = Config {tone: cli.tone, cat: cli.cat};
    let tokens = tokenize_withgrp_str(input);
    let tone_replaced = replace_tone(tokens, &conf);
    let combined = combine_tokens(tone_replaced);
    let res = TokenSequence(combined).to_string(&conf);
    println!("{res}");
}
//they should be methods of TokenSeq?