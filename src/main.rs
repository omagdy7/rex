mod nfa;
mod regex;
use nfa::*;
use regex::*;

fn main() {
    let input = "abcdefglmno";
    let token = Regex::new(String::from(input));
    println!("{input}\n{:#?}", token);

    let mut nfa = NFA::new();
    nfa.regex_to_nfa(token);
    nfa.add_state();

    println!("NFA: {:#?}", nfa);
    let inp = "abcdefglmno";
    let output = nfa.simulate(String::from(inp));
    println!("{inp} was = {output}")
}
