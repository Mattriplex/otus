use super::{TranspositionEntry, TranspositionTable};

impl TranspositionTable {
    pub fn new(size: usize) -> TranspositionTable {
        let table = vec![None; size];
        TranspositionTable {
            table,
            size,
            occupancy: 0,
        }
    }

    pub fn get(&self, hash: u64) -> Option<&TranspositionEntry> {
        let index = hash as usize % self.size;
        if let Some((stored_hash, value)) = &self.table[index] {
            if *stored_hash == hash {
                return Some(value);
            }
        }
        None
    }

    pub fn put(&mut self, hash: u64, value: TranspositionEntry) {
        let index = hash as usize % self.size;
        if self.table[index].is_none() {
            self.occupancy += 1;
        }
        self.table[index] = Some((hash, value)); // TODO add eviction policy
    }

    pub fn get_occupancy_factor(&self) -> f32 {
        self.occupancy as f32 / self.size as f32
    }
}
