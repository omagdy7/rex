mod nfa;
mod regex;
use nfa::*;
use regex::*;

fn test(regex: &str, input: &str) -> bool {
    let regex = Regex::new(regex);
    println!("{:#?}", regex);
    let nfa = NFA::from(regex);
    nfa.matches(input)
}

fn main() {
    dbg!(test("aa*", "aa   "));
}
