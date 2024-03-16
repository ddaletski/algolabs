use std::collections::HashMap;

use crate::{circular_suffix_array::CircularSuffixArray, DataTransformer};

/// [Burrows-Wheeler transform](https://en.wikipedia.org/wiki/Burrows%E2%80%93Wheeler_transform)
pub struct BWT {}

impl DataTransformer for BWT {
    fn transform(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return vec![];
        }

        let suffix_array = CircularSuffixArray::new(data);

        let original_index = suffix_array.pos_of_suffix(0).unwrap() as u32;

        let mut last_column: Vec<u8> = suffix_array
            .suffixes()
            .map(|suffix| suffix.last())
            .collect();

        last_column.extend_from_slice(&original_index.to_le_bytes());

        last_column
    }

    fn inverse_transform(&self, encoded: &[u8]) -> Vec<u8> {
        if encoded.is_empty() {
            return vec![];
        }

        let data: Vec<u8> = encoded.iter().take(encoded.len() - 4).cloned().collect();
        let mut data_sorted = data.clone();
        BWT::count_sort(&mut data_sorted);

        let idx_slice = &encoded[data.len()..];
        let original_index =
            u32::from_le_bytes([idx_slice[0], idx_slice[1], idx_slice[2], idx_slice[3]]) as usize;

        // key: byte from the alphabet
        // value: list of all positions where this byte appears in data buffer, sorted in descending order
        let mut bytes_positions = {
            let mut positions: HashMap<u8, Vec<u32>> = HashMap::new();
            for i in 0..(data.len()) {
                positions.entry(data[i]).or_default().push(i as u32);
            }

            positions.values_mut().for_each(|v| {
                v.sort_unstable();
                v.reverse();
            });

            positions
        };

        // index: permutation index in a sorted permutations list
        // value an index of the following (lexicographically) permutation in this list
        let mut following_permutations = vec![0; data.len()];
        for (idx, first_char) in data_sorted.iter().enumerate() {
            let position_in_data = bytes_positions.get_mut(first_char).unwrap().pop().unwrap();
            following_permutations[idx] = position_in_data;
        }

        let mut result = Vec::with_capacity(data.len());
        let mut idx = original_index;
        for _ in 0..data.len() {
            let byte = data_sorted[idx];
            result.push(byte);
            idx = following_permutations[idx] as usize;
        }

        result
    }
}

impl BWT {
    fn count_sort(data: &mut [u8]) {
        let mut counts = vec![0; 256];
        for &byte in data.iter() {
            counts[byte as usize] += 1;
        }

        let mut write_pos = 0;
        for (val, freq) in counts.into_iter().enumerate() {
            for _ in 0..freq {
                data[write_pos] = val as u8;
                write_pos += 1;
            }
        }
    }
}

impl Default for BWT {
    fn default() -> Self {
        BWT {}
    }
}

#[cfg(test)]
mod test {
    use super::BWT;
    use crate::DataTransformer;
    use proptest::test_runner::Config;

    #[test]
    fn transform_works() {
        let data = b"ABRACADABRA!";

        let transformed = BWT::default().transform(data);

        assert_eq!(transformed, b"ARD!RCAAAABB\x03\0\0\0")
    }

    #[test]
    fn inverse_transform_works() {
        let transformed = b"ARD!RCAAAABB\x03\0\0\0";

        let original = BWT::default().inverse_transform(transformed);

        unsafe {
            assert_eq!(String::from_utf8_unchecked(original), "ABRACADABRA!");
        }
    }

    #[test]
    fn transform_empty_returns_empty() {
        assert_eq!(BWT::default().transform(&[]), vec![]);
    }

    #[test]
    fn inverse_transform_empty_returns_empty() {
        assert_eq!(BWT::default().inverse_transform(&[]), vec![]);
    }

    proptest::proptest! {
        #![proptest_config(Config::with_cases(100))]
        #[test]
        fn transform_is_lossless(s in ".{0,1000}") {
            let orig_data: Vec<u8> = s.bytes().collect();
            let encoded = BWT::default().transform(&orig_data);
            let decoded = BWT::default().inverse_transform(&encoded);

            assert_eq!(decoded, orig_data);
        }
    }

    proptest::proptest! {
        #![proptest_config(Config::with_cases(100))]
        #[test]
        fn count_sort_works(s in ".{0,1000}") {
            let bytes: Vec<u8> = s.bytes().collect();

            let mut sorted_counting = bytes.clone();
            BWT::count_sort(&mut sorted_counting);

            let mut sorted_std = bytes.clone();
            sorted_std.sort_unstable();

            assert_eq!(sorted_counting, sorted_std);
        }
    }
}
