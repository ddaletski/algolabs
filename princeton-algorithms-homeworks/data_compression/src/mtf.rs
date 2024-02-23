use crate::DataTransformer;

/// [Move-to-front transform](https://en.wikipedia.org/wiki/Move-to-front)
pub struct MTF {}

impl DataTransformer for MTF {
    fn transform(&self, data: &[u8]) -> Vec<u8> {
        let mut bytes_indices: Vec<u8> = (0..=255).into_iter().collect();

        let mut result = Vec::with_capacity(data.len());

        for &byte in data {
            let val_index = bytes_indices.iter().position(|&x| x == byte).unwrap();
            result.push(val_index as u8);
            MTF::move_to_front(val_index, &mut bytes_indices);
        }

        result
    }

    fn inverse_transform(&self, encoded: &[u8]) -> Vec<u8> {
        let mut byte_values: Vec<u8> = (0..=255).into_iter().collect();

        let mut result = Vec::with_capacity(encoded.len());

        for &byte in encoded {
            let position = byte as usize;
            result.push(byte_values[position]);
            MTF::move_to_front(position, &mut byte_values);
        }

        result
    }
}

impl MTF {
    fn move_to_front(val_index: usize, arr: &mut [u8]) {
        if val_index == 0 {
            return;
        }

        let value = arr[val_index];

        for i in (1..=val_index).rev() {
            arr[i] = arr[i - 1];
        }
        arr[0] = value;
    }
}

impl Default for MTF {
    fn default() -> Self {
        MTF {}
    }
}

#[cfg(test)]
mod test {
    use super::MTF;
    use crate::DataTransformer;
    use proptest::test_runner::Config;

    #[test]
    fn transform_works() {
        let data = b"abracadabra!";
        let transformed = MTF::default().transform(data);
        let expected: [u8; 12] = [97, 98, 114, 2, 100, 1, 101, 1, 4, 4, 2, 38];
        assert_eq!(transformed, expected);
    }

    #[test]
    fn inverse_transform_works() {
        let encoded: [u8; 12] = [97, 98, 114, 2, 100, 1, 101, 1, 4, 4, 2, 38];
        let decoded = MTF::default().inverse_transform(&encoded);
        let expected = b"abracadabra!";
        assert_eq!(decoded, expected);
    }

    #[test]
    fn transform_empty_returns_empty() {
        assert_eq!(MTF::default().transform(&[]), vec![]);
    }

    #[test]
    fn inverse_transform_empty_returns_empty() {
        assert_eq!(MTF::default().inverse_transform(&[]), vec![]);
    }

    proptest::proptest! {
        #![proptest_config(Config::with_cases(100))]
        #[test]
        fn transform_is_lossless(s in ".{0,1000}") {
            let orig_data: Vec<u8> = s.bytes().collect();
            let encoded = MTF::default().transform(&orig_data);
            let decoded = MTF::default().inverse_transform(&encoded);

            assert_eq!(decoded, orig_data);
        }
    }

    #[rstest::rstest]
    #[case(&mut [0, 1, 2, 3, 4, 5], 0, &[0, 1, 2, 3, 4, 5])]
    #[case(&mut [0, 1, 2, 3, 4, 5], 1, &[1, 0, 2, 3, 4, 5])]
    #[case(&mut [0, 1, 2, 3, 4, 5], 2, &[2, 0, 1, 3, 4, 5])]
    #[case(&mut [0, 1, 2, 3, 4, 5], 3, &[3, 0, 1, 2, 4, 5])]
    #[case(&mut [0, 1, 2, 3, 4, 5], 4, &[4, 0, 1, 2, 3, 5])]
    #[case(&mut [0, 1, 2, 3, 4, 5], 5, &[5, 0, 1, 2, 3, 4])]
    fn move_to_front_works(
        #[case] input: &mut [u8],
        #[case] moved_value_index: usize,
        #[case] expected: &[u8],
    ) {
        MTF::move_to_front(moved_value_index, input);
        assert_eq!(input, expected);
    }
}
