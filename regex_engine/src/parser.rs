use std::collections::{HashMap, LinkedList};

use crate::nfa::{State, NFA};

pub(super) struct RegexParser {}

enum StackToken {
    LParen,
    Pipe,
}
struct TokenStackEntry {
    token: StackToken,
    position: usize,
}

impl RegexParser {
    pub fn new() -> RegexParser {
        RegexParser {}
    }

    pub fn parse(&self, regex: &str) -> NFA {
        let mut eps_transitions: HashMap<usize, LinkedList<usize>> = HashMap::new();

        let mut link = |from: usize, to: usize| {
            eps_transitions.entry(from).or_default().push_back(to);
        };

        let mut token_stack: LinkedList<TokenStackEntry> = LinkedList::new();

        let mut states = vec![];

        for ch in std::iter::once(b'(')
            .chain(regex.bytes())
            .chain(std::iter::once(b')'))
        {
            let idx = states.len();

            let next_state = State::from(ch);

            match next_state {
                State::Char(_) | State::Success => {}
                State::Star => {
                    link(idx - 1, idx);
                    link(idx, idx - 1);
                    link(idx, idx + 1);
                }
                State::LParen => {
                    token_stack.push_back(TokenStackEntry {
                        token: StackToken::LParen,
                        position: idx,
                    });
                    link(idx, idx + 1);
                }
                State::Pipe => {
                    token_stack.push_back(TokenStackEntry {
                        token: StackToken::Pipe,
                        position: idx,
                    });
                }
                State::RParen => {
                    let mut pipes_positions = vec![];
                    loop {
                        let top_token = token_stack
                            .pop_back()
                            .expect(&format!("unmatched right parenthesis at idx {:}", idx));

                        match top_token.token {
                            StackToken::Pipe => {
                                link(top_token.position, idx);
                                pipes_positions.push(top_token.position);
                            }
                            StackToken::LParen => {
                                for pipe_pos in pipes_positions {
                                    assert!(
                                        matches!(
                                            states[pipe_pos + 1],
                                            State::LParen | State::Char(_)
                                        ),
                                        "wrong right-side of '|' operation at {}",
                                        pipe_pos + 1
                                    );
                                    link(top_token.position, pipe_pos + 1);
                                }
                                break;
                            }
                        };
                    }

                    link(idx, idx + 1);
                }
            }

            states.push(next_state);
        }

        if let Some(_) = token_stack.pop_back() {
            panic!("unmatched left parenthesis");
        }

        states.push(State::Success);

        NFA {
            states,
            eps_transitions,
        }
    }
}
