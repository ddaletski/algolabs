use std::collections::BTreeMap;

pub struct SnapshotArray {
    stacks: Vec<BTreeMap<u16, u32>>,
    last_snapshot: u16,
}

impl SnapshotArray {
    pub fn new(length: i32) -> Self {
        let length = length as usize;

        let mut zero_vec = Vec::with_capacity(length);
        zero_vec.resize(length, BTreeMap::new());

        SnapshotArray {
            stacks: zero_vec,
            last_snapshot: 0,
        }
    }

    pub fn set(&mut self, index: i32, val: i32) {
        self.stacks[index as usize].insert(self.last_snapshot, val as u32);
    }

    pub fn snap(&mut self) -> i32 {
        self.last_snapshot += 1;
        (self.last_snapshot - 1) as i32
    }

    pub fn get(&self, index: i32, snap_id: i32) -> i32 {
        let map = &self.stacks[index as usize];

        map.range(..((snap_id + 1) as u16))
            .next_back()
            .map(|entry| *entry.1 as i32)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let length = 5;
        let mut obj = SnapshotArray::new(length);
        obj.set(0, 3);
        assert_eq!(obj.snap(), 0);

        obj.set(0, 2);
        assert_eq!(obj.snap(), 1);

        obj.set(0, 1);
        assert_eq!(obj.snap(), 2);

        assert_eq!(obj.get(0, 1), 2);
        assert_eq!(obj.get(0, 2), 1);
        assert_eq!(obj.get(0, 0), 3);
    }
}
