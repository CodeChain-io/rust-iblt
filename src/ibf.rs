use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasher, BuildHasherDefault, Hash};
use std::marker::PhantomData;

struct Item {
    count: usize,
    val_sum: Box<[u8]>,
    hash_sum: u64,
}

pub struct IBF<T, S = BuildHasherDefault<DefaultHasher>> {
    size: usize,
    map: Vec<Item>,
    hash_builder: S,
    phantom: PhantomData<*const T>,
}

impl<T: Hash + Into<Box<[u8]>>> IBF<T, BuildHasherDefault<DefaultHasher>> {
    pub fn new(size: usize) -> Self {
        Self::with_hasher(size, Default::default())
    }
}

impl<T: Hash + Into<Box<[u8]>>, S: BuildHasher> IBF<T, S> {
    pub fn with_hasher(size: usize, hash_builder: S) -> Self {
        let mut map = Vec::with_capacity(size);
        for _ in 0..size {
            map.push(Item {
                count: 0,
                val_sum: Box::new([]),
                hash_sum: 0,
            });
        }

        Self {
            size,
            map,
            hash_builder,
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
