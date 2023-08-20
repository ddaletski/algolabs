use std::io::BufWriter;

use regex_engine::Regex;

fn main() {
    let re = Regex::compile("(a(bc*|de)fg)|(hi*j)");

    let mut buf_writer = BufWriter::new(vec![]);
    re.generate_dot(&mut buf_writer).unwrap();

    let bytes = buf_writer.into_inner().unwrap();
    let dot = String::from_utf8(bytes).unwrap();

    println!("{}", dot);
}
