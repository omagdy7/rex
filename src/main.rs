mod nfa;
mod regex;
use nfa::*;
use regex::*;

fn test(regex: &str, input: &str) -> bool {
    let token = Regex::new(String::from(regex));
    dbg!(&token);
    let mut nfa = NFA::new();
    nfa.regex_to_nfa(token);
    let mut x: Vec<(&usize, &State)> = nfa.states.iter().map(|(k, v)| (k, v)).collect();
    x.sort();
    dbg!(x);
    // nfa.match_re(String::from(input))
    true
}

fn main() {
    // println!("{}", test("a.b..", "a.bxb"));
    println!("{}", test(".*b", "aaaaaabbb"))
}
