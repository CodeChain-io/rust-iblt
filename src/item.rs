use std::ops::{AddAssign, SubAssign};

#[derive(Clone)]
pub(crate) struct Item {
    pub(crate) count: usize,
    pub(crate) val_sum: Vec<u8>,
    pub(crate) hash_sum: u64,
}

fn xor_assign_slice(target: &mut Vec<u8>, rhs: Vec<u8>) {
    if target.len() < rhs.len() {
        let len_diff = rhs.len() - target.len();
        target.append(&mut vec![0; len_diff]);
    }
    for i in 0..rhs.len() {
        target[i] ^= rhs[i];
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            count: 0,
            val_sum: Vec::new(),
            hash_sum: 0,
        }
    }
}

impl AddAssign for Item {
    fn add_assign(&mut self, rhs: Self) {
        self.count += rhs.count;
        self.hash_sum ^= rhs.hash_sum;
        xor_assign_slice(&mut self.val_sum, rhs.val_sum);
    }
}

impl SubAssign for Item {
    fn sub_assign(&mut self, rhs: Self) {
        self.count -= rhs.count;
        self.hash_sum ^= rhs.hash_sum;
        xor_assign_slice(&mut self.val_sum, rhs.val_sum);
    }
}
