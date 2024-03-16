mod decoder_tree;
mod encoder_tree;

use std::collections::HashMap;

use bitvec::{field::BitField, order::Lsb0, vec::BitVec};

use crate::{huffman::decoder_tree::decoding_tree, DataTransformer};

use self::encoder_tree::{dump_tree, encoding_map, encoding_tree};

pub struct HuffmanTransform {}

impl DataTransformer for HuffmanTransform {
    fn transform(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return vec![];
        }

        let freq_map = Self::freq_map(&data);
        let encoding_tree = encoding_tree(&freq_map);
        let encoding_map = encoding_map(&encoding_tree.data);

        let max_tree_size = encoding_map.len() * 3;
        let mut result = Vec::with_capacity(4 + max_tree_size + (data.len() + 1));
        result.extend_from_slice(&(data.len() as u32).to_le_bytes());
        dump_tree(&encoding_tree.data, &mut result);

        let mut bit_buf: BitVec<u64, Lsb0> = BitVec::new();
        for byte in data {
            let encoded_byte = &encoding_map[byte];
            bit_buf.extend(encoded_byte);

            if bit_buf.len() > 64 {
                let (full_bytes, tail) = unsafe { bit_buf.split_at_unchecked(64) };
                result.extend_from_slice(&full_bytes.load::<u64>().to_le_bytes());
                bit_buf = tail.to_bitvec();
            }
        }

        let last_chunk: u64 = bit_buf.load();
        result.extend_from_slice(&last_chunk.to_le_bytes());

        result
    }

    fn inverse_transform(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return vec![];
        }

        let mut data_iter = data.iter();
        let result_size = u32::from_le_bytes([
            *data_iter.next().unwrap(),
            *data_iter.next().unwrap(),
            *data_iter.next().unwrap(),
            *data_iter.next().unwrap(),
        ]);

        let decoding_tree = decoding_tree(&mut data_iter);
        let encoded_bits: BitVec<u8, Lsb0> = BitVec::from_iter(data_iter);

        let mut result = vec![];

        let mut current_node = &decoding_tree;

        let mut bits_iter = encoded_bits.into_iter();

        let mut decoded_count = 0;
        while decoded_count < result_size {
            if let decoder_tree::Node::Leaf { value } = current_node {
                result.push(*value);
                current_node = &decoding_tree;
                decoded_count += 1;
            }

            let Some(bit) = bits_iter.next() else {
                break;
            };

            if let decoder_tree::Node::Inner { left, right } = current_node {
                if bit {
                    // 1 -> right
                    current_node = right;
                } else {
                    // 0 -> left
                    current_node = left;
                }
            }
        }

        result
    }
}

impl HuffmanTransform {
    fn freq_map(data: &[u8]) -> HashMap<u8, u32> {
        let mut map = HashMap::new();
        for &byte in data.iter() {
            map.entry(byte).and_modify(|freq| *freq += 1).or_insert(1);
        }
        map
    }
}

impl Default for HuffmanTransform {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod test {
    use proptest::test_runner::Config;

    use crate::{huffman::HuffmanTransform, DataTransformer};

    #[test]
    fn empty_encodes_to_empty() {
        let encoded = HuffmanTransform::default().transform(&[]);
        assert_eq!(encoded, &[]);
    }

    #[test]
    fn empty_decodes_to_empty() {
        let encoded = HuffmanTransform::default().inverse_transform(&[]);
        assert_eq!(encoded, &[]);
    }

    #[test]
    fn min_case() {
        let orig_data = b"hello!";
        let encoded = HuffmanTransform::default().transform(orig_data);
        let decoded = HuffmanTransform::default().inverse_transform(&encoded);

        assert_eq!(decoded, orig_data);
    }

    proptest::proptest! {
        #![proptest_config(Config::with_cases(100))]
        #[test]
        fn transform_is_lossless(s in ".{0,1000}") {
            let orig_data: Vec<u8> = s.bytes().collect();
            let encoded = HuffmanTransform::default().transform(&orig_data);
            let decoded = HuffmanTransform::default().inverse_transform(&encoded);

            assert_eq!(decoded, orig_data);
        }
    }
}
