use crate::regex::Regex;
use std::{collections::HashMap, usize};

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

impl From<Regex> for NFA {
    fn from(regex: Regex) -> Self {
        let mut nfa = NFA::new();
        nfa.regex_to_nfa(regex);
        nfa
    }
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

    pub fn regex_to_nfa_helper(&mut self, regex: Regex) -> (usize, usize) {
        match regex {
            Regex::Symbol(ch) => {
                let first = self.add_state();
                let last = self.add_state();
                self.add_transition(first, last, Trans::Symbol(ch));
                (first, last)
            }
            Regex::Concat((left, right)) => {
                let first = self.add_state();
                let (l_first, l_last) = self.regex_to_nfa_helper(*left);
                let (r_first, r_last) = self.regex_to_nfa_helper(*right);
                self.add_transition(first, l_first, Trans::Epsilon);
                self.add_transition(l_last, r_first, Trans::Epsilon);
                (first, r_last)
            }
            Regex::Union((left, right)) => {
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
            Regex::Plus(_) => todo!(),
            Regex::Star(tok) => {
                let first = self.add_state();
                let (t_first, t_last) = self.regex_to_nfa_helper(*tok);
                self.add_transition(first, t_first, Trans::Epsilon);
                self.add_transition(t_last, first, Trans::Epsilon);
                let last = self.add_state();
                self.add_transition(first, last, Trans::Epsilon);
                self.add_transition(t_last, last, Trans::Epsilon);
                (first, last)
            }
            Regex::Dot => {
                let first = self.add_state();
                (first, first)
            }
            Regex::None => {
                let state = self.add_state();
                (state, state)
            }
        }
    }
    pub fn regex_to_nfa(&mut self, regex: Regex) {
        let (_, last) = self.regex_to_nfa_helper(regex);
        self.accepting_states.push(last);
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

    fn matches_helper(&self, state: &State, input: &str, mut idx: usize) -> bool {
        if self.accepting_states.contains(&state.id) {
            return idx >= input.len();
        }
        let mut result = false;
        for (id, trans) in &state.transitions {
            match trans {
                // TODO: chars.nth() Work at O(n) consider using bytes instead of losing UTF-8 support
                Trans::Symbol(ch) => match input.chars().nth(idx) {
                    Some(nxt) if nxt == *ch => {
                        idx += 1;
                        result |= self.matches_helper(self.states.get(id).unwrap(), input, idx);
                        idx -= 1;
                    }
                    None => {}
                    _ => {}
                },
                Trans::Epsilon => {
                    result |= self.matches_helper(self.states.get(id).unwrap(), input, idx);
                }
            }
        }
        result
    }

    pub fn matches(&self, input: &str) -> bool {
        self.matches_helper(self.states.get(&self.initial_state).unwrap(), input, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex::Regex;

    fn test(regex: &str, input: &str) -> bool {
        let token = Regex::new(regex);
        let mut nfa = NFA::new();
        nfa.regex_to_nfa(token);
        let mut x: Vec<(&usize, &State)> = nfa.states.iter().map(|(k, v)| (k, v)).collect();
        x.sort();
        nfa.matches(input)
    }

    #[test]
    fn test_concat_succ() {
        assert!(test("abc", "abc"));
        assert!(test("", ""));
        assert!(test("Thisshouldmatch", "Thisshouldmatch"));
    }

    #[test]
    fn test_concat_fail() {
        assert!(!test("abc", "abd"));
        assert!(!test("abc", "abd"));
        assert!(!test("abc", "abcc"));
        assert!(!test("abc", "notabc"));
        assert!(!test("abc", ""));
    }

    #[test]
    fn test_union_succ() {
        assert!(test("(a|b)", "a"));
        assert!(test("(a|b|c|d)", "a"));
        assert!(test("(a|b|c|d)", "b"));
        assert!(test("(a|b|c|d)", "d"));
        assert!(test("(a|b|c|d)", "c"));
        assert!(test("(a|b)", "b"));
        assert!(test("(a|b)b", "bb"));
        assert!(test("(a|b)a", "ba"));
    }

    #[test]
    fn test_union_fail() {
        assert!(!test("(a|b)", "x"));
        assert!(!test("(a|b)", "ax"));
    }

    #[test]
    fn test_star_empty_input() {
        assert!(test("", ""));
        assert!(test("a*", ""));
        assert!(test("b*", ""));
        assert!(test("(a|b)*", ""));
    }

    #[test]
    fn test_star_succ() {
        assert!(test("(0)*1(0)*", "000000000100000"));
        assert!(test("(a)*abc(a)*", "aaaaaaabcaaaaaa"));
        assert!(test("a*b", "b"));
        assert!(test("a*bcd", "aaaaaabcd"));
    }

    #[test]
    fn test_star_fail() {
        assert!(!test("a*b", "aabbbb"));
        assert!(!test("1*0", "20"));
        assert!(!test("(0)*1(0)*", "101100000"));
    }

    #[test]
    fn test_complex_succ() {
        assert!(test("(a|b|c)*", "abababababcbcba"));
        assert!(test("(a|b)*cc", "aaabababaabacc"));
    }
}
