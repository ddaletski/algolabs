pub mod bwt;
pub mod circular_suffix_array;
pub mod huffman;
pub mod mtf;

pub trait DataTransformer {
    fn transform(&self, data: &[u8]) -> Vec<u8>;
    fn inverse_transform(&self, data: &[u8]) -> Vec<u8>;
}
