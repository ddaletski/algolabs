mod nfa;
mod parser;

use std::io::Write;

use nfa::NFA;
use parser::RegexParser;

pub struct Regex {
    nfa: NFA,
}

impl Regex {
    pub fn compile(re: &str) -> Regex {
        let parser = RegexParser::new();
        let nfa = parser.parse(re);

        Regex { nfa }
    }

    pub fn matches(&self, text: &str) -> bool {
        self.nfa.matches(text)
    }

    pub fn generate_dot<W: Write>(&self, writer: W) -> std::io::Result<()> {
        self.nfa.generate_dot(writer)
    }
}
