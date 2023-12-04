mod nfa;
mod regex;
use nfa::*;
use regex::*;

fn test(regex: &str, input: &str) -> bool {
    let regex = Regex::new(String::from(regex));
    let nfa = NFA::from(regex);
    nfa.matches(String::from(input))
}

fn main() {
    println!("{}", dbg!(test("(a|b)a", "aa")));
    println!("{}", dbg!(test("(a|b)a", "ba")));
    println!("{}", dbg!(test("(a|b)a", "bb")));
}
