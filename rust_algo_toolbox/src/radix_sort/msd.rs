use std::{collections::VecDeque, iter::zip, ops::Range};

use itertools::Itertools;

/// Sorts the given strings in lexicographical order using the MSD radix sort algorithm.
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

/// Sorts the given byte vectors in lexicographical order using the MSD radix sort algorithm.
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

/// Sort the given byte slices in lexicographical order using the MSD radix sort algorithm.
pub fn sort_slices(slices: &mut [&[u8]]) {
    let permutation = sorted_permutation(&slices);

    slices.clone_from_slice(&permutation.into_iter().map(|i| slices[i]).collect_vec());
}

/// Compute a permutation of input data to sort it in lexicographical order.
pub fn sorted_permutation(data: &[&[u8]]) -> Vec<usize> {
    if data.is_empty() {
        return vec![];
    }

    let mut keys = vec![0u16; data.len()];
    // keys are in range `[0, 256]`, with 0 meaning 'absent key'
    // so we need `(256 + 2) = 258` elements to store the write position of each key
    // including 'absent key' and the last element to store the total count
    let mut key_dst_pos = vec![0usize; 258];

    let mut permutations = (
        (0..data.len()).into_iter().collect_vec(),
        (0..data.len()).into_iter().collect_vec(),
    );

    let mut queue: VecDeque<(Range<usize>, usize)> = VecDeque::new();
    queue.push_back((0..data.len(), 0));

    let mut last_byte_idx = 0;
    while let Some((range, byte_idx)) = queue.pop_front() {
        if byte_idx != last_byte_idx {
            permutations.0 = permutations.1;
            permutations.1 = permutations.0.clone();
            last_byte_idx = byte_idx;
        }

        permutations.0[range.clone()]
            .iter()
            .map(|&str_idx| key_at(data[str_idx], byte_idx))
            .enumerate()
            .for_each(|(i, key)| {
                keys[i] = key;
            });

        unsafe {
            sort_by_u16_keys(
                &permutations.0[range.clone()],
                &keys[..range.len()],
                &mut key_dst_pos,
                &mut permutations.1[range.clone()],
            )
        };

        // determine subranges with the same current character 
        let subrange_separators = key_dst_pos.iter().cloned().unique().collect_vec();
        let subranges = subrange_separators
            .windows(2)
            .map(|window| (range.start + window[0])..(range.start + window[1]))
            .filter(|r| r.len() > 1);

        for subrange in subranges {
            queue.push_back((subrange, byte_idx + 1));
        }
    }

    permutations.1
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
unsafe fn sort_by_u16_keys<'a, T>(src: &[T], keys: &[u16], key_dst_pos: &mut [usize], dst: &mut [T])
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
