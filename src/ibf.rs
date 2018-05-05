use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};
use std::marker::PhantomData;

use bincode::serialize;
use serde::{Deserialize, Serialize};

fn calc_hash<T, H>(val: &T, mut hasher: H) -> u64 where T: Hash, H: Hasher {
    val.hash(&mut hasher);
    hasher.finish()
}

struct Item {
    count: usize,
    val_sum: Box<[u8]>,
    hash_sum: u64,
}

pub struct IBF<T, S = BuildHasherDefault<DefaultHasher>> {
    size: usize,
    hash_count: usize,
    map: Vec<Item>,
    hash_builder: S,
    phantom: PhantomData<*const T>,
}

impl<'de, T: Hash + Deserialize<'de> + Serialize> IBF<T, BuildHasherDefault<DefaultHasher>> {
    pub fn new(size: usize, hash_count: usize) -> Self {
        Self::with_hasher(size, hash_count, Default::default())
    }
}

impl<'de, T: Hash + Deserialize<'de> + Serialize, S: BuildHasher> IBF<T, S> {
    pub fn with_hasher(size: usize, hash_count: usize, hash_builder: S) -> Self {
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
            hash_count,
            map,
            hash_builder,
            phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, val: &T) {
        let hash = calc_hash(val, self.hash_builder.build_hasher());
        let val_bin = serialize(val);
        let mut index = hash as usize;
        for _ in 0..self.hash_count {
            index = calc_hash(&index, self.hash_builder.build_hasher()) as usize;
            let item = &mut self.map[index % self.size];
            item.count += 1;
            item.hash_sum ^= hash;
            // TODO: insert to val_sum
        }
    }

    pub fn remove(&mut self, val: &T) {
        unimplemented!();
    }

    pub fn decode(self) -> HashSet<T> {
        unimplemented!();
    }
}
