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
/// # use algo_toolbox::counting_sort::sort;
/// #
/// let arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
/// let sorted = sort(&arr, |x| *x);
/// assert_eq!(sorted, vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
/// ```
///
/// # Note
/// The sorting is stable.
pub fn sort<T, KeyGen>(data: &[T], keygen: KeyGen) -> Vec<T>
where
    T: Clone,
    KeyGen: FnMut(&T) -> u8,
{
    let mut sorted = Vec::with_capacity(data.len());
    sorted.resize_with(data.len(), || MaybeUninit::uninit());

    sort_to_uninit_slice(data, keygen, &mut sorted);

    unsafe { std::mem::transmute(sorted) }
}

/// Perform a counting sort of a given data and write it to a destination slice
///
/// # Arguments
/// * `src` - Input data to sort.
/// * `keygen` - A function that takes an element of the array and returns a `u8` key to sort by.
/// * `dst` - Destination slice to put sorted data. Should be big enough to fit all data from the input.
///
/// # Example
/// ```
/// # use algo_toolbox::counting_sort::sort_to_slice;
/// #
/// let arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
/// let mut sorted = vec![0; arr.len()];
/// sort_to_slice(&arr, |x| *x, &mut sorted);
/// assert_eq!(&sorted, &[1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
/// ```
///
/// # Note
/// The sorting is stable.
pub fn sort_to_slice<T, KeyGen>(src: &[T], keygen: KeyGen, dst: &mut [T])
where
    T: Clone,
    KeyGen: FnMut(&T) -> u8,
{
    assert!(
        dst.len() >= src.len(),
        "output buffer is not large enough to fit sorted data"
    );

    let keys: Vec<u8> = src.iter().map(keygen).collect();

    let mut keys_write_pos = keys_dst_idx(&keys);

    for (key, value) in zip(keys, src) {
        let write_pos = &mut keys_write_pos[key as usize];
        dst[*write_pos] = value.clone();
        *write_pos += 1;
    }
}

/// Perform a counting sort of a given data and write it to a destination uninitialized slice
///
/// # Arguments
/// * `src` - Input data to sort.
/// * `keygen` - A function that takes an element of the array and returns a `u8` key to sort by.
/// * `dst` - Destination uninitialized slice to put sorted data. Should be big enough to fit all data from the input.
///
/// # Returns
/// Initialized `dst` slice
///
/// # Example
/// ```
/// # use algo_toolbox::counting_sort::sort_to_uninit_slice;
/// # use std::mem::MaybeUninit;
/// #
/// let arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
///
/// // allocate a vector of uninitialize u8 integers
/// let mut sorted = Vec::with_capacity(arr.len());
/// sorted.resize_with(arr.len(), || MaybeUninit::uninit());
///
/// let sorted = sort_to_uninit_slice(&arr, |x| *x, &mut sorted);
/// assert_eq!(sorted, &[1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
/// ```
///
/// # Note
/// The sorting is stable.
pub fn sort_to_uninit_slice<'dst, T, KeyGen>(
    src: &[T],
    keygen: KeyGen,
    dst: &'dst mut [MaybeUninit<T>],
) -> &'dst mut [T]
where
    T: Clone,
    KeyGen: FnMut(&T) -> u8,
{
    assert!(
        dst.len() >= src.len(),
        "output buffer is not large enough to fit sorted data"
    );

    let keys: Vec<u8> = src.iter().map(keygen).collect();

    let mut keys_write_pos = keys_dst_idx(&keys);

    for (key, value) in zip(keys, src) {
        let write_pos = &mut keys_write_pos[key as usize];
        dst[*write_pos].write(value.clone());
        *write_pos += 1;
    }

    unsafe { std::mem::transmute(dst) }
}

/// Compute starting index in a sorted array for each possible key in the alphabet
fn keys_dst_idx(items_keys: &[u8]) -> Vec<usize> {
    let mut counts = vec![0usize; u8::MAX as usize + 1];

    for &key in items_keys {
        if key == 255 {
            continue;
        }
        counts[key as usize + 1] += 1;
    }
    for i in 1..counts.len() {
        counts[i] += counts[i - 1];
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest::proptest! {
        #[test]
        fn u8_sorting_correct(arr: Vec<u8>) {
            let sorted = sort(&arr, |x| *x);
            let mut expected = arr;
            expected.sort();

            proptest::prop_assert_eq!(sorted, expected);
        }
    }
}
