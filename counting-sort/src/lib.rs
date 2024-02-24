use std::{iter::zip, mem::MaybeUninit};

/// Perform a counting sort of a given data and return a sorted vector.
///
/// # Arguments
/// * `data` - The data slice to sort.
/// * `keygen` - A function that takes an element of the array and returns a `u8` key to sort by.
///
/// # Returns
/// A new array containing the sorted elements.
///
/// # Example
/// ```
/// # use counting_sort::counting_sort;
/// #
/// let arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
/// let sorted = counting_sort(&arr, |x| *x);
/// assert_eq!(sorted, vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
/// ```
///
/// # Note
/// The sorting is stable.
pub fn counting_sort<T, KeyGen>(data: &[T], keygen: KeyGen) -> Vec<T>
where
    T: Clone,
    KeyGen: FnMut(&T) -> u8,
{
    let keys: Vec<u8> = data.iter().map(keygen).collect();

    let mut keys_write_pos = {
        let mut counts = vec![0usize; u8::MAX as usize + 2];

        for &key in &keys {
            counts[key as usize + 1] += 1;
        }
        for i in 1..counts.len() {
            counts[i] += counts[i - 1];
        }
        counts
    };

    let mut sorted = Vec::with_capacity(data.len());
    sorted.resize_with(data.len(), || MaybeUninit::uninit());

    for (key, value) in zip(keys, data) {
        let write_pos = &mut keys_write_pos[key as usize];
        sorted[*write_pos].write(value.clone());
        *write_pos += 1;
    }

    unsafe { std::mem::transmute(sorted) }
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest::proptest! {
        #[test]
        fn u8_sorting_correct(arr: Vec<u8>) {
            let sorted = counting_sort(&arr, |x| *x);
            let mut expected = arr;
            expected.sort();

            proptest::prop_assert_eq!(sorted, expected);
        }
    }
}
