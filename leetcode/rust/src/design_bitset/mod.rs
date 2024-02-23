struct Bitset {
    full_chunks: Vec<u64>,
    last_chunk: Vec<u8>,
}

impl Bitset {
    fn new(size: i32) -> Self {
        let full_chunks_count = (size / 64) as usize;
        let remaining_bits = (size % 64) as usize;

        let mut full_chunks = Vec::with_capacity(full_chunks_count);
        full_chunks.resize(full_chunks_count, 0);

        let mut last_chunk = Vec::with_capacity(remaining_bits);
        last_chunk.resize(remaining_bits, 0);

        Self {
            full_chunks,
            last_chunk,
        }
    }

    fn fix(&mut self, idx: i32) {
        let chunk_idx = (idx / 64) as usize;
        let bit_idx = (idx % 64) as usize;

        if chunk_idx != self.full_chunks.len() {
            let bit = 1 << bit_idx;
            self.full_chunks[chunk_idx] |= bit;
        } else {
            self.last_chunk[bit_idx] = 255;
        }
    }

    fn unfix(&mut self, idx: i32) {
        let chunk_idx = (idx / 64) as usize;
        let bit_idx = (idx % 64) as usize;

        if chunk_idx != self.full_chunks.len() {
            let bit = !(1 << bit_idx);
            self.full_chunks[chunk_idx] &= bit;
        } else {
            self.last_chunk[bit_idx] = 0;
        }
    }

    fn flip(&mut self) {
        self.full_chunks.iter_mut().for_each(|chunk| *chunk = !*chunk);
        self.last_chunk.iter_mut().for_each(|bit| *bit = !*bit);
    }

    fn all(&self) -> bool {
        self.full_chunks.iter().all(|&chunk| chunk == u64::MAX)
            && self.last_chunk.iter().all(|&chunk| chunk == u8::MAX)
    }

    fn one(&self) -> bool {
        !(self.full_chunks.iter().all(|&chunk| chunk == 0)
            && self.last_chunk.iter().all(|&chunk| chunk == 0))
    }

    fn count(&self) -> i32 {
        let full_count: i32 = self
            .full_chunks
            .iter()
            .map(|&chunk| chunk.count_ones() as i32)
            .sum();
        let last_count = self.last_chunk.iter().filter(|&&bit| bit != 0).count() as i32;

        full_count + last_count
    }

    fn to_string(&self) -> String {
        let mut result = String::with_capacity(self.full_chunks.len() * 64 + self.last_chunk.len());

        for mut chunk in self.full_chunks.iter().cloned() {
            for _ in 0..64 {
                result.push(if chunk % 2 == 0 { '0' } else { '1' });
                chunk >>= 1;
            }
        }

        result.extend(
            self.last_chunk
                .iter()
                .cloned()
                .map(|bit| if bit == 0 { '0' } else { '1' }),
        );

        result
    }
}

#[cfg(test)]
mod test {
    use super::Bitset;

    #[test]
    fn case1() {
        let count = 317;
        let mut set = Bitset::new(count);

        assert_eq!(set.count(), 0);
        set.flip();
        assert_eq!(set.count(), count);
        assert!(set.all());
        set.flip();
        assert_eq!(set.count(), 0);

        assert!(!set.one());
        set.fix(0);
        assert_eq!(set.count(), 1);
        assert!(set.one());
        set.fix(0);
        assert_eq!(set.count(), 1);
        set.fix(1);
        assert_eq!(set.count(), 2);

        set.unfix(1);
        assert_eq!(set.count(), 1);
        set.unfix(1);
        assert_eq!(set.count(), 1);
    }
}
