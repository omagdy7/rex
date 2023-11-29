#![allow(dead_code, unused_variables, unused_mut)]

#[derive(Debug)]
struct State {
    transitions: Vec<(Trans, usize)>, // (input symbol, destination state index)
}

#[derive(Debug)]
struct NFA {
    graph: Vec<State>,
    initial_state: usize,
    accepting_states: Vec<usize>,
}

impl NFA {
    fn new(graph: Vec<State>, initial_state: usize, accepting_states: Vec<usize>) -> Self {
        NFA {
            graph,
            initial_state,
            accepting_states,
        }
    }

    fn simulate(&self, input: &str) -> bool {
        let mut current_states = vec![self.initial_state];

        for c in input.chars() {
            let mut next_states = vec![];

            for state in &current_states {
                for (trans, dest_state) in &self.graph[*state].transitions {
                    // println!("char: {c}, trans: {trans:?}, dest_state: {dest_state:?}");
                    match trans {
                        Trans::Symbol(symbol) if *symbol == c => {
                            next_states.push(*dest_state);
                        }
                        Trans::Epsilon => {
                            next_states.push(*dest_state);
                        }
                        _ => {}
                    }
                }
            }

            current_states = next_states;
        }

        // println!("states: {:?}", current_states);

        current_states
            .iter()
            .any(|&state| self.accepting_states.contains(&state))
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Trans {
    Symbol(char),
    Epsilon,
}

struct Regex {
    regex: &'static str,
}

impl Regex {
    fn new(input: &'static str) -> Self {
        Regex { regex: input }
    }

    fn compile_nfa(&self) -> NFA {
        let mut graph: Vec<State> = vec![State {
            transitions: vec![],
        }];
        let initial_state = 0;
        let mut accepting_states = vec![];

        let mut current_state = initial_state;
        let mut prev_state: Option<usize> = None;

        let input: Vec<char> = self.regex.chars().collect();

        for (i, &c) in input.iter().enumerate() {
            match c {
                '*' => {}
                '+' => todo!(),
                '.' => {
                    let next_state = graph.len();
                    println!("{next_state}");
                    graph.push(State {
                        transitions: vec![],
                    });

                    graph[next_state]
                        .transitions
                        .push((Trans::Epsilon, next_state));

                    current_state = next_state;
                }
                '|' => {
                    let before = input[i - 1];
                    let after = input[i + 1];
                    let next_state = graph.len();
                    graph.push(State {
                        transitions: vec![],
                    });
                    graph[current_state]
                        .transitions
                        .push((Trans::Symbol(c), next_state));

                    current_state = next_state;
                }
                _ => {
                    // let after = input[i + 1];
                    // if after != '|' {
                    let next_state = graph.len();
                    graph.push(State {
                        transitions: vec![],
                    });
                    graph.push(State {
                        transitions: vec![],
                    });

                    graph[current_state]
                        .transitions
                        .push((Trans::Symbol(c), next_state + 1));
                    graph[next_state]
                        .transitions
                        .push((Trans::Epsilon, next_state + 1));

                    current_state = next_state + 1;
                    // }
                }
            }
        }

        accepting_states.push(current_state);

        NFA::new(graph, initial_state, accepting_states)
    }
}

fn main() {
    let regex = Regex::new("a.bc");
    let nfa = regex.compile_nfa();

    println!("NFA: {:#?}", nfa);

    let input = vec!["a", "abcc", "abc", "abbc", "abbbc", "abbbbc", "ac", "ab"];

    println!("Regex: {:#?}", regex.regex);

    for inp in input {
        println!("{} -> {:?}", inp, nfa.simulate(inp)); // Should match
    }
}
