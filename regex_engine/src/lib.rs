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

#[cfg(test)]
mod test {
    use super::Regex;
    use rstest::rstest;

    #[rstest::fixture]
    fn re() -> Regex {
        Regex::compile("a(bc*|de)fg|h.i*j.*")
    }

    #[rstest]
    #[case("abfg")]
    #[case("abcfg")]
    #[case("abccccccfg")]
    #[case("adefg")]
    #[case("h0ij00000")]
    #[case("h0jbfg")]
    #[case("hzj")]
    #[case("hzij")]
    #[case("h0iij")]
    #[case("hzjalskdhaklfh")]
    fn positive_cases(re: Regex, #[case] text: &str) {
        assert!(re.matches(text));
    }

    #[rstest]
    #[case("")]
    #[case("acfg")]
    #[case("abcdefg")]
    #[case("abefg")]
    #[case("hhhij")]
    #[case("j")]
    fn negative_cases(re: Regex, #[case] text: &str) {
        assert!(!re.matches(text));
    }

    #[rstest]
    fn string_fully_fed() {
        let re = Regex::compile("a");
        let string = "aaaa";
        assert!(!re.matches(string));
    }
}
