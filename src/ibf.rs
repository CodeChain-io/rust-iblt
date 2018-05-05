use std::collections::HashSet;
use std::marker::PhantomData;

struct Item {
    count_sum: usize,
    val_sum: Box<[u8]>,
    hash_sum: u64,
}

pub struct IBF<T> {
    size: usize,
    map: Vec<Item>,
    phantom: PhantomData<*const T>,
}

impl<T> IBF<T> {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            map: Vec::with_capacity(size),
            phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, val: &T) {
        unimplemented!();
    }

    pub fn remove(&mut self, val: &T) {
        unimplemented!();
    }

    pub fn decode(self) -> HashSet<T> {
        unimplemented!();
    }
}
