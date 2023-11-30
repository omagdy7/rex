use crate::regex::RegexToken;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Trans {
    Symbol(char),
    Epsilon,
}

#[derive(Debug, Clone, PartialEq)]
struct State {
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
    states: HashMap<usize, State>,
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
                self.add_transition(t_last, first, Trans::Epsilon);
                (first, t_last)
            }
            RegexToken::Dot => {
                let first = self.add_state();
                (first, first)
            }
            RegexToken::None => (self.add_state(), self.add_state()),
        }
    }
    pub fn regex_to_nfa(&mut self, input: RegexToken) {
        self.regex_to_nfa_helper(input);
        self.deduce_accepting_states();
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

    fn deduce_accepting_states(&mut self) {
        for (id, state) in &self.states {
            if state.transitions.is_empty() {
                self.accepting_states.push(*id)
            }
        }
    }

    pub fn simulate(&mut self, input: String) -> bool {
        use Trans::*;
        let mut current = self.states.get(&self.initial_state).unwrap();
        let mut chars = input.chars();
        'simu: while let Some(nxt) = chars.next() {
            println!("current: {current:?}");
            for (id, trans) in &current.transitions {
                println!("nxt: {nxt} ch: {trans:?}");
                if let Symbol(ch) = trans {
                    if *ch == nxt {
                        current = self.states.get(id).unwrap();
                    } else {
                        break 'simu;
                    }
                } else {
                    // Epsilon case
                }
            }
        }
        println!("current node: {current:?}");
        let current_id = current.id;
        self.accepting_states.iter().any(|&id| id == current_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex::Regex;

    #[test]
    fn test_simple_succ() {
        let input = "abc";
        let token = Regex::new(String::from(input));
        let mut nfa = NFA::new();
        nfa.regex_to_nfa(token);
        nfa.add_state();
        let inp = "abc";
        let output = nfa.simulate(String::from(inp));
        assert!(output)
    }

    #[test]
    fn test_simple_fail() {
        let input = "abc";
        let token = Regex::new(String::from(input));
        let mut nfa = NFA::new();
        nfa.regex_to_nfa(token);
        nfa.add_state();
        let inp = "abd";
        let output = nfa.simulate(String::from(inp));
        assert!(!output)
    }
}
