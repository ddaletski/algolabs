use std::fs::File;

use regex_engine::Regex;

macro_rules! check {
    ($e:expr) => {
        if !$e {
            eprintln!("check failed: {}", stringify!($e));
        }
    };
}

fn main() {
    let re = Regex::compile("a(bc*|de)fg");

    re.generate_dot(File::create("graph.dot").unwrap()).unwrap();

    check!(re.matches("abfg"));
    check!(re.matches("abcfg"));
    check!(re.matches("abccccccfg"));
    check!(re.matches("adefg"));

    check!(!re.matches(""));
    check!(!re.matches("abfg"));
    check!(!re.matches("abcdefg"));
    check!(!re.matches("abefg"));
}
