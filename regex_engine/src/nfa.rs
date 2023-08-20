use std::{
    collections::{HashMap, HashSet, LinkedList},
    fmt::{Debug, Display},
    io::Write,
};

pub(super) enum State {
    Char(u8),
    LParen,
    RParen,
    Star,
    Pipe,
    Success,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let State::Char(ch) = self {
            f.write_str(&format!("{}", char::from_u32(*ch as u32).unwrap()))
        } else {
            let display_str = match self {
                State::LParen => "(",
                State::RParen => ")",
                State::Star => "*",
                State::Pipe => "|",
                State::Success => "end",
                _ => unreachable!(),
            };
            f.write_str(display_str)
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl From<u8> for State {
    fn from(value: u8) -> Self {
        match value {
            b'(' => State::LParen,
            b')' => State::RParen,
            b'*' => State::Star,
            b'|' => State::Pipe,
            _ => State::Char(value),
        }
    }
}

#[derive(Debug)]
pub(super) struct NFA {
    pub(super) states: Vec<State>,
    pub(super) eps_transitions: HashMap<usize, LinkedList<usize>>,
}

impl NFA {
    pub fn matches(&self, text: &str) -> bool {
        let mut reachable_states: HashSet<usize> = HashSet::new();
        reachable_states.insert(0);

        // find all reachable states from the start state
        self.visit_connected_states(&mut reachable_states);

        for text_char in text.bytes() {
            let mut next_states: HashSet<usize> = HashSet::new();

            // read the next text character and do direct transitions if the character matches
            for &state_idx in &reachable_states {
                if let State::Char(re_char) = self.states[state_idx] {
                    if re_char == text_char {
                        next_states.insert(state_idx + 1);
                    }
                }
            }

            // do all possible eps transitions from the candidate states
            self.visit_connected_states(&mut next_states);

            reachable_states = next_states;

            if reachable_states.contains(&(self.states.len() - 1)) {
                return true;
            }
        }

        false
    }

    fn visit_connected_states(&self, reachable: &mut HashSet<usize>) {
        let mut stack: Vec<usize> = reachable.clone().into_iter().collect();
        while let Some(current_state) = stack.pop() {
            for &next_state in self
                .eps_transitions
                .get(&current_state)
                .unwrap_or(&LinkedList::new())
                .iter()
            {
                if reachable.contains(&next_state) {
                    continue;
                }
                stack.push(next_state);
                reachable.insert(next_state);
            }
        }
    }


    pub fn generate_dot<W: Write>(&self, mut writer: W) -> std::io::Result<()> {
        write!(&mut writer, "digraph NFA {{\n")?;

        write!(&mut writer, "  rankdir=LR;\n")?;

        for (idx, state) in self.states.iter().enumerate() {
            write!(
                &mut writer,
                "  N{}[label=\"{}\"; weight={}];\n",
                idx,
                state,
                idx + 1
            )?;
        }

        for (idx, state) in self.states.iter().enumerate() {
            if let State::Char(_) = state {
                write!(&mut writer, "  N{} -> N{};\n", idx, idx + 1)?;
            }
        }

        for (src, dst_list) in &self.eps_transitions {
            for dst in dst_list {
                write!(&mut writer, "  N{} -> N{} [color=\"red\"];\n", src, dst)?;
            }
        }

        write!(&mut writer, "}}\n")?;

        Ok(())
    }
}
