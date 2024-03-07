use itertools::Itertools;

/// Sorts the given strings in lexicographical order using the LSD radix sort algorithm.
pub fn sort_strings(strings: &mut [String]) {
    let copy = strings.to_vec();
    let mut bytes_view = copy.iter().map(|s| s.as_bytes()).collect_vec();

    sort_slices(&mut bytes_view);

    for (idx, slice) in bytes_view.into_iter().enumerate() {
        strings[idx] = unsafe { String::from_utf8_unchecked(slice.to_vec()) };
    }
}

/// Sorts the given byte vectors in lexicographical order using the LSD radix sort algorithm.
pub fn sort_vecs(vecs: &mut [Vec<u8>]) {
    let copy = vecs.to_vec();
    let mut bytes_view = copy.iter().map(|s| s.as_slice()).collect_vec();
    sort_slices(&mut bytes_view);

    for (idx, slice) in bytes_view.into_iter().enumerate() {
        vecs[idx] = slice.to_vec();
    }
}

/// Sort the given byte slices in lexicographical order using the LSD radix sort algorithm.
pub fn sort_slices(data: &mut [&[u8]]) {
    if data.is_empty() {
        return;
    }

    let max_len = data.iter().map(|s| s.len()).max().unwrap();

    // keys are in range `[0, 256]`, with 0 meaning 'absent key'
    // so we need `(256 + 2) = 258` elements to store the write position of each key
    // including 'absent key' and the last element to store the total count
    let mut key_dst_pos = vec![0usize; 258];

    let mut aux = vec![&[] as &[u8]; data.len()];
    let mut buffers: (&mut [&[u8]], &mut [&[u8]]) = (data, aux.as_mut_slice());

    let mut nswaps = 0;
    for byte_idx in (0..max_len).rev() {
        unsafe { sort_by_byte(buffers.0, byte_idx, &mut key_dst_pos, buffers.1) };

        std::mem::swap(&mut buffers.0, &mut buffers.1);
        nswaps += 1;
    }

    if nswaps % 2 != 0 {
        let (src, dst) = buffers;
        dst.copy_from_slice(src);
    }
}

#[inline]
fn key_at(data: &[u8], idx: usize) -> u16 {
    if idx < data.len() {
        data[idx] as u16 + 1
    } else {
        0 // 'absent key'
    }
}

unsafe fn sort_by_byte<'a>(
    src: &[&'a [u8]],
    byte_idx: usize,
    key_dst_pos: &mut [usize],
    dst: &mut [&'a [u8]],
) {
    key_dst_pos.fill(0);
    for key in src.iter().map(|s| key_at(s, byte_idx)) {
        *key_dst_pos.get_unchecked_mut((key + 1) as usize) += 1;
    }
    for i in 1..key_dst_pos.len() {
        *key_dst_pos.get_unchecked_mut(i) += *key_dst_pos.get_unchecked_mut(i - 1);
    }

    for &value in src {
        let write_pos = key_dst_pos.get_unchecked_mut(key_at(value, byte_idx) as usize);
        *dst.get_unchecked_mut(*write_pos) = value;
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
