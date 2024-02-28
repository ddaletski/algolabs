use std::iter::zip;

use itertools::Itertools;

/// Sorts the given strings in lexicographical order using the LSD radix sort algorithm.
pub fn sort_strings(strings: &mut [String]) {
    let bytes_view = strings.iter().map(|s| s.as_bytes()).collect_vec();

    let permutation = sorted_permutation(&bytes_view);

    strings.clone_from_slice(
        &permutation
            .into_iter()
            .map(|i| strings[i].clone())
            .collect_vec(),
    );
}

/// Sorts the given byte vectors in lexicographical order using the LSD radix sort algorithm.
pub fn sort_vecs(vecs: &mut [Vec<u8>]) {
    let bytes_view = vecs.iter().map(|s| s.as_slice()).collect_vec();

    let permutation = sorted_permutation(&bytes_view);

    vecs.clone_from_slice(
        &permutation
            .into_iter()
            .map(|i| vecs[i].clone())
            .collect_vec(),
    );
}

/// Compute a permutation of input data to sort it in lexicographical order.
pub fn sorted_permutation(data: &[&[u8]]) -> Vec<usize> {
    if data.is_empty() {
        return vec![];
    }

    let max_len = data.iter().map(|s| s.len()).max().unwrap();

    let mut keys = vec![0u16; data.len()];
    // keys are in range `[0, 256]`, with 0 meaning 'absent key'
    // so we need `(256 + 2) = 258` elements to store the write position of each key
    // including 'absent key' and the last element to store the total count
    let mut key_dst_pos = vec![0usize; 258];

    let mut permutations = (
        (0..data.len()).into_iter().collect_vec(),
        (0..data.len()).into_iter().collect_vec(),
    );

    for byte_idx in (0..max_len).rev() {
        permutations
            .0
            .iter()
            .map(|&str_idx| key_at(data[str_idx], byte_idx))
            .enumerate()
            .for_each(|(i, key)| {
                keys[i] = key;
            });

        unsafe {
            sort_by_keys(
                &mut permutations.0,
                &keys,
                &mut key_dst_pos,
                &mut permutations.1,
            )
        };

        std::mem::swap(&mut permutations.0, &mut permutations.1);
    }

    permutations.0
}

#[inline]
fn key_at(data: &[u8], idx: usize) -> u16 {
    if idx < data.len() {
        data[idx] as u16 + 1
    } else {
        0 // 'absent key'
    }
}

/// Sort `src` by `keys` and write the result to `dst`.
///
/// `key_dst_pos` is used as a temporary buffer to store the write position of each key
/// to avoid its reallocation between calls.
///
/// Keys should be in range `[0, 256]`.
/// `keys_dst_pos.len()` should be >= 258
unsafe fn sort_by_keys<'a, T>(src: &[T], keys: &[u16], key_dst_pos: &mut [usize], dst: &mut [T])
where
    T: Clone,
{
    key_dst_pos.fill(0);
    for &key in keys {
        *key_dst_pos.get_unchecked_mut((key + 1) as usize) += 1;
    }
    for i in 1..key_dst_pos.len() {
        *key_dst_pos.get_unchecked_mut(i) += *key_dst_pos.get_unchecked_mut(i - 1);
    }

    for (&key, value) in zip(keys, src) {
        let write_pos = key_dst_pos.get_unchecked_mut(key as usize);
        *dst.get_unchecked_mut(*write_pos) = value.clone();
        *write_pos += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::{arbitrary::any, prelude::prop::collection::vec as pvec};

    proptest::proptest! {
        #[test]
        fn test_sort_strings(mut data in pvec(".{0,500}", 0..20)) {
            let mut expected = data.clone();
            expected.sort();

            sort_strings(&mut data);

            proptest::prop_assert_eq!(data, expected);
        }

        #[test]
        fn test_sort_vecs(mut data in pvec(pvec(any::<u8>(), 0..500), 0..20)) {
            let mut expected = data.clone();
            expected.sort();

            sort_vecs(&mut data);

            proptest::prop_assert_eq!(data, expected);
        }
    }
}
