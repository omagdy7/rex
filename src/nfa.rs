use crate::regex::RegexToken;
use std::{
    clone,
    collections::{HashMap, HashSet, VecDeque},
    iter::Peekable,
    str::Chars,
    usize,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trans {
    Symbol(char),
    Epsilon,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    id: usize,
    transitions: Vec<(usize, Trans)>,
}

impl State {
    fn new(id: usize) -> Self {
        Self {
            id,
            transitions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NFA {
    pub states: HashMap<usize, State>,
    transitions: HashMap<usize, Vec<usize>>,
    initial_state: usize,
    accepting_states: Vec<usize>,
}

impl NFA {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            transitions: HashMap::new(),
            initial_state: 0,
            accepting_states: Vec::new(),
        }
    }

    pub fn add_state(&mut self) -> usize {
        let new_state = State::new(self.states.len());
        let id = new_state.id;
        self.states.insert(id, new_state);
        id
    }

    pub fn regex_to_nfa_helper(&mut self, input: RegexToken) -> (usize, usize) {
        match input {
            RegexToken::Symbol(ch) => {
                let first = self.add_state();
                let last = self.add_state();
                self.add_transition(first, last, Trans::Symbol(ch));
                (first, last)
            }
            RegexToken::Concat((left, right)) => {
                let first = self.add_state();
                let (l_first, l_last) = self.regex_to_nfa_helper(*left);
                let (r_first, r_last) = self.regex_to_nfa_helper(*right);
                self.add_transition(first, l_first, Trans::Epsilon);
                self.add_transition(l_last, r_first, Trans::Epsilon);
                (first, r_last)
            }
            RegexToken::Union((left, right)) => {
                let first = self.add_state();
                let (l_first, l_last) = self.regex_to_nfa_helper(*left);
                let (r_first, r_last) = self.regex_to_nfa_helper(*right);
                self.add_transition(first, l_first, Trans::Epsilon);
                self.add_transition(first, r_first, Trans::Epsilon);
                let last = self.add_state();
                self.add_transition(l_last, last, Trans::Epsilon);
                self.add_transition(r_last, last, Trans::Epsilon);
                (first, last)
            }
            RegexToken::Plus(_) => todo!(),
            RegexToken::Star(tok) => {
                let first = self.add_state();
                let (t_first, t_last) = self.regex_to_nfa_helper(*tok);
                self.add_transition(first, t_first, Trans::Epsilon);
                self.add_transition(t_last, t_first, Trans::Epsilon);
                let last = self.add_state();
                self.add_transition(first, last, Trans::Epsilon);
                self.add_transition(t_last, last, Trans::Epsilon);
                (first, last)
            }
            RegexToken::Dot => {
                let first = self.add_state();
                (first, first)
            }
            RegexToken::None => {
                let state = self.add_state();
                (state, state)
            }
        }
    }
    pub fn regex_to_nfa(&mut self, input: RegexToken) {
        let (_, last) = self.regex_to_nfa_helper(input);
        self.accepting_states.push(last);
        let dead_state = self.add_state();
        self.add_transition(last, dead_state, Trans::Symbol('#'));
    }

    pub fn add_transition(&mut self, from: usize, to: usize, trans: Trans) {
        self.transitions
            .entry(from)
            .and_modify(|val| val.push(to))
            .or_insert_with(|| vec![to]);
        self.states
            .get_mut(&from)
            .unwrap()
            .transitions
            .push((to, trans));
    }

    fn dfs(&self, state: &State, mut chars: Chars, next_states: &mut HashSet<usize>) {
        println!("{state:?}\n{chars:?}");
        for (id, trans) in &state.transitions {
            match trans {
                Trans::Symbol(ch) => {
                    if ch == &'#' {
                        dbg!(ch);
                        chars.next();
                        self.dfs(self.states.get(id).unwrap(), chars.clone(), next_states);
                        next_states.remove(&(id - 1));
                        println!("Removing {}", id - 1);
                        next_states.insert(*id);
                        println!("ch: {ch}: Inserting {id}");
                    }
                    let mut cloned = chars.clone().peekable();
                    let nxt_peek = cloned.peek();
                    match nxt_peek {
                        Some(nxt) if nxt == ch || nxt == &'.' => {
                            // println!("nxt: {nxt}");
                            chars.next();
                            self.dfs(self.states.get(id).unwrap(), chars.clone(), next_states);
                            next_states.insert(*id);
                            println!("ch: {ch} Inserting {id}");
                        }
                        None => {}
                        _ => {}
                    }
                }
                Trans::Epsilon => {
                    self.dfs(self.states.get(id).unwrap(), chars.clone(), next_states);
                    next_states.insert(*id);
                    println!("ch: eps Inserting {id}");
                }
            }
        }
    }

    pub fn match_re(&mut self, input: String) -> bool {
        let chars = input.chars();
        let mut next_states = HashSet::new();
        let _last_state = self.dfs(
            self.states.get(&self.initial_state).unwrap(),
            chars,
            &mut next_states,
        );

        dbg!(&next_states, &self.accepting_states);

        for nxt in next_states {
            if self.accepting_states.iter().any(|&x| x == nxt) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex::Regex;

    fn test(regex: &str, input: &str) -> bool {
        let token = Regex::new(String::from(regex));
        // dbg!(&token);
        let mut nfa = NFA::new();
        nfa.regex_to_nfa(token);
        let mut x: Vec<(&usize, &State)> = nfa.states.iter().map(|(k, v)| (k, v)).collect();
        x.sort();
        dbg!(x);
        nfa.match_re(String::from(input))
    }

    #[test]
    fn test_concat_succ() {
        assert!(test("abc", "abc"));
        assert!(test("", ""));
        assert!(test("This should match", "This should match"));
    }

    #[test]
    fn test_concat_fail() {
        assert!(!test("abc", "abd"));
        assert!(!test("abc", "abcc"));
        assert!(!test("abc", "notabc"));
        assert!(!test("abc", ""));
    }

    #[test]
    fn test_union_succ() {
        assert!(test("(a|b)", "a"));
        assert!(test("(a|b)", "b"));
        assert!(test("(a|b)b", "bb"));
        assert!(test("(a|b)a", "ba"));
        assert!(test("(a|b).", "at"));
    }

    #[test]
    fn test_union_fail() {
        assert!(!test("(a|b)", "x"));
        assert!(!test("(a|b)", "ax"));
    }

    #[test]
    fn test_star_succ() {
        assert!(test("(0)*1(0)*", "000000000100000"));
        assert!(test("a*b", "b"));
        assert!(test("a*bcd", "aaaaaabcd"));
    }

    #[test]
    fn test_star_fail() {
        assert!(!test("a*b", "aabbbb"));
        assert!(!test("1*0", "20"));
        assert!(!test("(0)*1(0)*", "101100000"));
    }
}
