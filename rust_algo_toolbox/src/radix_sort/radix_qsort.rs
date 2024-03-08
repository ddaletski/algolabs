use std::ops::Deref;

trait RadixSortable {
    fn key_at(&self, idx: usize) -> u16;
}

impl RadixSortable for [u8] {
    fn key_at(&self, idx: usize) -> u16 {
        if idx < self.len() {
            self[idx] as u16 + 1
        } else {
            0 // 'absent key'
        }
    }
}

impl<Container> RadixSortable for Container
where
    Container: Deref<Target = [u8]>,
{
    fn key_at(&self, idx: usize) -> u16 {
        if idx < self.len() {
            self[idx] as u16 + 1
        } else {
            0 // 'absent key'
        }
    }
}

/// Sorts the given strings in lexicographical order using the 3-way radix quicksort algorithm.
pub fn sort_strings(strings: &mut [String]) {
    let vecs = unsafe { std::mem::transmute::<&mut [String], &mut [Vec<u8>]>(&mut *strings) };
    sort_rec(vecs, 0, 0, vecs.len() as i32 - 1);
}

/// Sorts the given byte vectors in lexicographical order using the 3-way radix quicksort algorithm.
pub fn sort_vecs(vecs: &mut [Vec<u8>]) {
    sort_rec(vecs, 0, 0, vecs.len() as i32 - 1);
}

/// Sorts the given byte slices in lexicographical order using the 3-way radix quicksort algorithm.
pub fn sort_slices(data: &mut [&[u8]]) {
    sort_rec(data, 0, 0, data.len() as i32 - 1);
}

fn sort_rec(data: &mut [impl RadixSortable], byte_idx: usize, left: i32, right: i32) {
    if left >= right {
        return;
    }

    let pivot_char = data[left as usize].key_at(byte_idx);

    let mut less_than_pivot = left;
    let mut greater_than_pivot = right;

    let mut i = less_than_pivot + 1;
    while i <= greater_than_pivot {
        match data[i as usize].key_at(byte_idx).cmp(&pivot_char) {
            std::cmp::Ordering::Less => {
                data.swap(i as usize, less_than_pivot as usize);
                less_than_pivot += 1;
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                data.swap(i as usize, greater_than_pivot as usize);
                greater_than_pivot -= 1;
            }
            std::cmp::Ordering::Equal => {
                i += 1;
            }
        }
    }

    sort_rec(data, byte_idx, left, less_than_pivot - 1);
    if pivot_char != 0 {
        sort_rec(data, byte_idx + 1, less_than_pivot, greater_than_pivot);
    }
    sort_rec(data, byte_idx, greater_than_pivot + 1, right);
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
