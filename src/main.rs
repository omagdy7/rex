mod nfa;
mod regex;
use nfa::*;
use regex::*;

fn test(regex: &str, input: &str) -> bool {
    let token = Regex::new(String::from(regex));
    let mut nfa = NFA::new();
    nfa.regex_to_nfa(token);
    nfa.matches(String::from(input))
}

fn main() {
    // println!("{}", test("a.b..", "a.bxb"));
    println!("{}", test(".b", "ab"))
}
