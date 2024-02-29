use super::{TranspEntry, TranspTable};

impl TranspTable {
    pub fn new(size: usize) -> TranspTable {
        let table = vec![None; size];
        TranspTable { table, size }
    }

    pub fn get(&self, hash: u64) -> Option<&TranspEntry> {
        let index = hash as usize % self.size;
        if let Some((stored_hash, value)) = &self.table[index] {
            if *stored_hash == hash {
                return Some(value);
            }
        }
        None
    }

    pub fn put(&mut self, hash: u64, value: TranspEntry) {
        let index = hash as usize % self.size;
        self.table[index] = Some((hash, value)); // TODO add eviction policy
    }
}
