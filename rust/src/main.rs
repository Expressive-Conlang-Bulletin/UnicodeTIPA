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
mod copy;

const DEFAULT_SLEEP_DURATION: u64 = 100;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short='C', long, arg_enum, default_value_t = LessSpace)]
    cat: CatenateMethod,
    #[clap(short, long, arg_enum, default_value_t = Ligature)]
    tone: ToneMethod,
    #[clap(short, long, help="Copy to clipboard instead of print to standard output.")]
    copy: bool,
    #[clap(short, default_value_t = DEFAULT_SLEEP_DURATION, help="If stuck or fail when copy, try to increase this.")]
    sleep: u64,
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
    // println!("{tone_replaced:?}");
    let combined = combine_tokens(tone_replaced);
    let res = TokenSequence(combined).to_string(&conf);
    if cli.copy {
        copy::copy_to_clipboard(res, &cli.sleep)
    } else {
        println!("{res}");
    }
}
//they should be methods of TokenSeq?