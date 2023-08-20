use std::{
    collections::{HashMap, LinkedList},
    fmt::Display,
    io::Write,
};

#[derive(Debug)]
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
        false
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
