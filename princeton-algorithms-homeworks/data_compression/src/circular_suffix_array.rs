use std::{cmp::Ordering, ops::Index};

use itertools::Itertools;

#[derive(PartialEq, Eq)]
pub struct CircularSuffix<'a> {
    string: &'a [u8],
    index: usize,
}

impl<'a> CircularSuffix<'a> {
    pub fn new(string: &'a [u8], index: usize) -> Self {
        assert!(!string.is_empty());

        Self { string, index }
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }

    pub fn idx(&self) -> usize {
        self.index
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec = self.string.to_vec();
        vec.rotate_left(self.index);
        vec
    }

    /// get first byte
    pub fn first(&self) -> u8 {
        self[0]
    }

    /// get last byte
    pub fn last(&self) -> u8 {
        self[self.string.len() - 1]
    }
}

impl<'a> Index<usize> for CircularSuffix<'a> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.string[(self.index + index) % self.string.len()]
    }
}

impl<'a> PartialOrd for CircularSuffix<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for CircularSuffix<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        for idx in 0..(self.string.len()) {
            let a = self[idx];
            let b = other[idx];

            match a.cmp(&b) {
                Ordering::Equal => {}
                other => return other,
            };
        }

        Ordering::Equal
    }
}

pub struct CircularSuffixArray<'a> {
    data: &'a [u8],
    indices: Vec<usize>,
}

impl<'a> CircularSuffixArray<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        assert!(!data.is_empty());

        let mut indices = (0..data.len()).into_iter().collect_vec();

        // sort circular suffixes indices without generating them all
        indices.sort_unstable_by(|&idx1, &idx2| {
            CircularSuffix {
                string: data,
                index: idx1,
            }
            .cmp(&CircularSuffix {
                string: data,
                index: idx2,
            })
        });

        Self { data, indices }
    }

    pub fn len(&self) -> usize {
        self.indices.len()
    }

    /// get suffix at a given position in a sorted suffix array
    pub fn suffix_at(&self, order: usize) -> Option<CircularSuffix> {
        self.indices
            .get(order)
            .map(|&idx| CircularSuffix::new(self.data, idx))
    }

    /// get order of N-th suffix (its position in a sorted suffix array)
    pub fn pos_of_suffix(&self, suffix_idx: usize) -> Option<usize> {
        self.indices.iter().position(|&v| v == suffix_idx)
    }

    /// get iterator over sorted suffixes
    pub fn suffixes(&self) -> impl Iterator<Item = CircularSuffix> + '_ {
        self.indices.iter().map(|&idx| CircularSuffix {
            string: &self.data,
            index: idx,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn banana() -> CircularSuffixArray<'static> {
        CircularSuffixArray::new(b"banana")
    }

    #[rstest]
    fn suffix_to_vec_works() {
        let data = b"abcd";
        let expected = vec![b"abcd", b"bcda", b"cdab", b"dabc"];

        let suffixes = (0..4)
            .map(|idx| CircularSuffix::new(data, idx).to_vec())
            .collect_vec();

        assert_eq!(suffixes, expected);
    }

    #[rstest]
    fn suffixes_are_ordered(banana: CircularSuffixArray<'_>) {
        let suffixes = banana.suffixes().map(|s| s.to_vec()).collect_vec();
        let expected: Vec<&[u8]> = vec![
            b"abanan", b"anaban", b"ananab", b"banana", b"nabana", b"nanaba",
        ];

        assert_eq!(suffixes, expected);
    }

    #[rstest]
    fn pos_of_suffix_works(banana: CircularSuffixArray<'_>) {
        assert_eq!(banana.pos_of_suffix(0), Some(3));
        assert_eq!(banana.pos_of_suffix(1), Some(2));
        assert_eq!(banana.pos_of_suffix(2), Some(5));
        assert_eq!(banana.pos_of_suffix(3), Some(1));
        assert_eq!(banana.pos_of_suffix(4), Some(4));
        assert_eq!(banana.pos_of_suffix(5), Some(0));
    }

    #[rstest]
    fn suffix_idx_at_works(banana: CircularSuffixArray<'_>) {
        assert_eq!(banana.suffix_at(0).unwrap().idx(), 5);
        assert_eq!(banana.suffix_at(1).unwrap().idx(), 3);
        assert_eq!(banana.suffix_at(2).unwrap().idx(), 1);
        assert_eq!(banana.suffix_at(3).unwrap().idx(), 0);
        assert_eq!(banana.suffix_at(4).unwrap().idx(), 4);
        assert_eq!(banana.suffix_at(5).unwrap().idx(), 2);
    }
}
