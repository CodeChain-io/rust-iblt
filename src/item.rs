use std::ops::{AddAssign, SubAssign};

#[derive(Clone, Debug)]
pub(crate) struct Item {
    pub(crate) count: isize,
    pub(crate) key_sum: Vec<u8>,
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

impl Item {
    pub fn is_empty(&self) -> bool {
        self.count == 0
            && self.key_sum.iter().all(|v| *v == 0)
            && self.val_sum.iter().all(|v| *v == 0)
            && self.hash_sum == 0
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            count: 0,
            key_sum: Vec::new(),
            val_sum: Vec::new(),
            hash_sum: 0,
        }
    }
}

impl AddAssign for Item {
    fn add_assign(&mut self, rhs: Self) {
        self.count += rhs.count;
        self.hash_sum ^= rhs.hash_sum;
        xor_assign_slice(&mut self.key_sum, rhs.key_sum);
        xor_assign_slice(&mut self.val_sum, rhs.val_sum);
    }
}

impl SubAssign for Item {
    fn sub_assign(&mut self, rhs: Self) {
        self.count -= rhs.count;
        self.hash_sum ^= rhs.hash_sum;
        xor_assign_slice(&mut self.key_sum, rhs.key_sum);
        xor_assign_slice(&mut self.val_sum, rhs.val_sum);
    }
}
