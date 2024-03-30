use std::collections::HashMap;

pub fn boyer_moore(text: &str, pattern: &str) -> Option<usize> {
    let Some(pos) = boyer_moore_slice(text.as_bytes(), pattern.as_bytes()) else {
        return None;
    };

    for (char_pos, (byte_pos, _)) in text.char_indices().enumerate() {
        if byte_pos >= pos {
            return Some(char_pos);
        }
    }

    Some(text.len() - pattern.len())
}

pub fn boyer_moore_slice(text: &[u8], pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }

    if text.len() < pattern.len() {
        return None;
    }

    let mut last_idx_table = HashMap::new();
    for (idx, &byte) in pattern.iter().enumerate() {
        last_idx_table
            .entry(byte)
            .and_modify(|pos| *pos = idx)
            .or_insert(idx);
    }

    let mut start = 0;
    while start <= text.len() - pattern.len() {
        let mut found = true;
        for pattern_idx in (0..pattern.len()).rev() {
            let str_byte = text[start + pattern_idx];

            if str_byte == pattern[pattern_idx] {
                continue;
            }

            let Some(&byte_pos) = last_idx_table.get(&str_byte) else {
                found = false;
                start += pattern.len() - 1;
                break;
            };

            if byte_pos > pattern_idx {
                found = false;
                break;
            }

            found = false;
            let shift = pattern_idx - byte_pos;
            start += shift - 1;
            break;
        }

        if found {
            return Some(start);
        }

        start += 1;
    }

    None
}

#[cfg(test)]
mod test {
    use crate::assert_returns;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcabc", "abc", Some(0))]
    #[case("a", "a", Some(0))]
    #[case("abcabc", "ca", Some(2))]
    #[case("qwertyuiop", "uiop", Some(6))]
    #[case("abcd", "d", Some(3))]
    #[case("abÁcdËÜf", "ËÜ", Some(5))]
    #[case("abÁcdËÜfabÁcdËÜf", "fa", Some(7))]
    fn found_when_present(
        #[case] text: &str,
        #[case] pattern: &str,
        #[case] expected: Option<usize>,
    ) {
        assert_returns!(expected, boyer_moore, text, pattern);
    }

    #[rstest]
    #[case("abcabc", "def", None)]
    #[case("a", "b", None)]
    #[case("abcabc", "abcd", None)]
    fn not_found_when_absent(
        #[case] text: &str,
        #[case] pattern: &str,
        #[case] expected: Option<usize>,
    ) {
        assert_returns!(expected, boyer_moore, text, pattern);
    }

    #[rstest]
    #[case("a", "", Some(0))]
    #[case("", "", Some(0))]
    fn returns_start_when_pattern_is_empty(
        #[case] text: &str,
        #[case] pattern: &str,
        #[case] expected: Option<usize>,
    ) {
        assert_returns!(expected, boyer_moore, text, pattern);
    }

    #[rstest]
    #[case("", "a", None)]
    #[case("", "13786", None)]
    fn returns_none_when_text_is_empty(
        #[case] text: &str,
        #[case] pattern: &str,
        #[case] expected: Option<usize>,
    ) {
        assert_returns!(expected, boyer_moore, text, pattern);
    }
}
