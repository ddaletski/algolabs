use std::io::{self, Read};

use data_compression::{bwt::BWT, huffman::HuffmanTransform, mtf::MTF, DataTransformer};

fn main() {
    let mut data = Vec::with_capacity(1024);
    io::stdin().read_to_end(&mut data).unwrap();
    data.pop(); // pop \0 from the end

    println!("original size: {}", data.len());

    let transformers: &[(&'static str, &dyn DataTransformer)] = &[
        ("bwt", &BWT {}),
        ("mtf", &MTF {}),
        ("huffman", &HuffmanTransform {}),
    ];

    let encoded = transformers
        .iter()
        .fold(data, |buf, (_, transformer)| transformer.transform(&buf));

    println!("encoded size: {}", encoded.len());

    let decoded = transformers
        .iter()
        .rev()
        .fold(encoded, |buf, (_, transformer)| {
            transformer.inverse_transform(&buf)
        });

    println!("decoded size: {}", decoded.len());
}
