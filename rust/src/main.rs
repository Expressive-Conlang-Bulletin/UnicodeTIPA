extern crate serde_json;
extern crate kmp;

use data::read_groups;
use tokenize::TokenSequence;
use CatenateMethod::*;
use ToneMethod::*;

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
    let str3 = String::from("dõˑ˩ ʃidaˑ˥˩ | ɲe˧ t͡sʰʉ˥ muːɲiˑ˥˩ t͡sʰiˑ˥˩ ʔowvis˧ t͡sʰʉj˧ mowrʉ̃ˑ˩ leˑ˩ dud̠͡ʒiː˧˥ | ɲi˥ nuːʒeː˧˥ tʰuʒeː˧˥ t̠͡ʃʰi˧ | fiʃeː˧˧ raːkɛj˥ but̠͡ʃi˥ ʔub˧ | fiʃeː˧˧ muwsʉː˧˥ raːkɛj˥ ɲe˥ ‖");
    for u in str.chars().map(|c| c as u32){
        println!("{u:?}");
    }
    println!("{:#?}", data::read_combineboth());
    println!("{:#?}", tone::replace_tone(tokenize::tokenize_str(str), ToneMethod::Ligature));
    println!("{:#?}", group::replace_groups(tokenize::tokenize_str(str2.clone()), &read_groups()));
    println!("{:#?}", combine::combine_tokens(group::replace_groups(tokenize::tokenize_str(str2.clone()), &read_groups())));
    println!("{:#?}", tokenize::tokenize_all_str(str2.clone()));
    let conf = Config {tone: Ligature, cat: Bracket, unknown: ()};
    let last = TokenSequence(tokenize::tokenize_all_str(str3)).to_string(&conf);
    println!("{last}");
    //seems there's a line break after each input
    //TODO: call tone, combine
}