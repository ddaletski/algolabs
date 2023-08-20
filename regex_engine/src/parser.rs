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
        let mut states = vec![];
        let mut eps_transitions: HashMap<usize, LinkedList<usize>> = HashMap::new();

        let mut add_eps_transition = |from: usize, to: usize| {
            eps_transitions.entry(from).or_default().push_back(to);
        };

        // stack for '(' and '|' characters
        let mut token_stack: LinkedList<TokenStackEntry> = LinkedList::new();

        // add parentheses around an actual expression
        for next_char in std::iter::once(b'(')
            .chain(regex.bytes())
            .chain(std::iter::once(b')'))
        {
            let pos = states.len();

            let next_state = State::from(next_char);

            match next_state {
                State::Char(_) | State::Success => {}
                State::Star => {
                    add_eps_transition(pos - 1, pos);
                    add_eps_transition(pos, pos - 1);
                    add_eps_transition(pos, pos + 1);
                }
                State::LParen => {
                    token_stack.push_back(TokenStackEntry {
                        token: StackToken::LParen,
                        position: pos,
                    });
                    add_eps_transition(pos, pos + 1);
                }
                State::Pipe => {
                    token_stack.push_back(TokenStackEntry {
                        token: StackToken::Pipe,
                        position: pos,
                    });
                }
                State::RParen => {
                    let mut pipes_positions = vec![];
                    loop {
                        let top_token = token_stack
                            .pop_back()
                            .expect(&format!("unmatched right parenthesis at idx {}", pos));

                        match top_token.token {
                            StackToken::Pipe => {
                                add_eps_transition(top_token.position, pos);
                                pipes_positions.push(top_token.position);
                            }
                            StackToken::LParen => {
                                for pipe_pos in pipes_positions {
                                    assert!(
                                        matches!(
                                            states[pipe_pos + 1],
                                            State::LParen | State::Char(_)
                                        ),
                                        "wrong right-side of '|' operation at idx {}",
                                        pipe_pos + 1
                                    );
                                    add_eps_transition(top_token.position, pipe_pos + 1);
                                }
                                break;
                            }
                        };
                    }

                    add_eps_transition(pos, pos + 1);
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
