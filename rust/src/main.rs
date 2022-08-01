//extern crate serde;

use crate::data::read_groups;
extern crate serde_json;
extern crate kmp;

mod data;
mod tokenize;
mod tone;
mod group;
mod combine;
mod serialize;

pub struct Config {
    tone: ToneMethod,
    cat: CatenateMethod,
    unknown: ()
}

pub enum CatenateMethod {
    Bracket,
    Space
}

pub enum ToneMethod {
    Ligature,
    Catenate,
    Keep
}

mod tests{
    
}

fn main() {
	let str = String::from("ã [aˑ˩˩]");
    let str2 = String::from("1l̴3");
    for u in str.chars().map(|c| c as u32){
        println!("{u:?}");
    }
    println!("{:#?}", data::read_combineboth());
    println!("{:#?}", tone::replace_tone(tokenize::tokenize_str(str), ToneMethod::Ligature));
    println!("{:#?}", group::replace_groups(tokenize::tokenize_str(str2.clone()), &read_groups()));
    println!("{:#?}", combine::combine_tokens(group::replace_groups(tokenize::tokenize_str(str2.clone()), &read_groups())));
    println!("{:#?}", tokenize::tokenize_all_str(str2));
}