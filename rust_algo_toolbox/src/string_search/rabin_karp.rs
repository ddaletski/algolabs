const BASE: usize = 256;
const BIG_PRIME: usize = 2 << 31 - 1;

fn hash(s: &[u8]) -> usize {
    let mut result = 0;
    for &byte in s {
        result = (result * 256 + (byte as usize)) % BIG_PRIME;
    }

    result
}

fn pow(x: usize, p: usize) -> usize {
    let mut result = 1;

    for _ in 0..p {
        result = (result * x) % BIG_PRIME;
    }

    result
}

pub fn rabin_karp(text: &str, pattern: &str) -> Option<usize> {
    let Some(pos) = rabin_karp_slice(text.as_bytes(), pattern.as_bytes()) else {
        return None;
    };

    for (char_pos, (byte_pos, _)) in text.char_indices().enumerate() {
        if byte_pos >= pos {
            return Some(char_pos);
        }
    }

    Some(text.len() - pattern.len())
}

pub fn rabin_karp_slice(text: &[u8], pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }

    if text.len() < pattern.len() {
        return None;
    }

    let base_pow_n = pow(BASE, pattern.len() - 1);

    let pattern_hash = hash(pattern);
    let mut substring_hash = hash(&text[..pattern.len()]);

    if substring_hash == pattern_hash {
        if pattern == &text[..pattern.len()] {
            return Some(0);
        }
    }

    for start_idx in 1..=(text.len() - pattern.len()) {
        let removed_char = text[start_idx - 1] as usize;
        let added_char = text[start_idx + pattern.len() - 1] as usize;

        // removing previous character (`BIG_PRIME` is added to avoid underflow)
        substring_hash = substring_hash + removed_char * (BIG_PRIME - base_pow_n);

        // adding next character
        substring_hash = (substring_hash * BASE + added_char) % BIG_PRIME;

        if substring_hash == pattern_hash {
            if pattern == &text[start_idx..(start_idx + pattern.len())] {
                return Some(start_idx);
            }
        }
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
        assert_returns!(expected, rabin_karp, text, pattern);
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
        assert_returns!(expected, rabin_karp, text, pattern);
    }

    #[rstest]
    #[case("a", "", Some(0))]
    #[case("", "", Some(0))]
    fn returns_start_when_pattern_is_empty(
        #[case] text: &str,
        #[case] pattern: &str,
        #[case] expected: Option<usize>,
    ) {
        assert_returns!(expected, rabin_karp, text, pattern);
    }

    #[rstest]
    #[case("", "a", None)]
    #[case("", "13786", None)]
    fn returns_none_when_text_is_empty(
        #[case] text: &str,
        #[case] pattern: &str,
        #[case] expected: Option<usize>,
    ) {
        assert_returns!(expected, rabin_karp, text, pattern);
    }
}
