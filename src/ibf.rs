use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};
use std::marker::PhantomData;

use bincode;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use item::Item;

fn calc_hash<T, H>(val: &T, mut hasher: H) -> u64 where T: Hash, H: Hasher {
    val.hash(&mut hasher);
    hasher.finish()
}

pub enum Error {
    DecodeFail,
    SerializeError(bincode::Error),
    MalformedData(bincode::Error),
}

pub struct IBF<T, S = BuildHasherDefault<DefaultHasher>> {
    hash_count: usize,
    map: Vec<Item>,
    hash_builder: S,
    phantom: PhantomData<*const T>,
}

impl<T: Eq + Hash + DeserializeOwned + Serialize> IBF<T, BuildHasherDefault<DefaultHasher>> {
    pub fn new(size: usize, hash_count: usize) -> Self {
        Self::with_hasher(size, hash_count, Default::default())
    }
}

impl<T: Eq + Hash + DeserializeOwned + Serialize, S: BuildHasher> IBF<T, S> {
    pub fn with_hasher(size: usize, hash_count: usize, hash_builder: S) -> Self {
        Self {
            hash_count,
            map: vec![Item::default(); size],
            hash_builder,
            phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, val: &T) -> Result<(), bincode::Error> {
        let size = self.map.len();
        let hash = calc_hash(val, self.hash_builder.build_hasher());
        let val_bin = bincode::serialize(val)?;
        let mut index = hash as usize;
        for _ in 0..self.hash_count {
            index = calc_hash(&index, self.hash_builder.build_hasher()) as usize;
            self.map[index % size] += Item {
                count: 1,
                val_sum: val_bin.clone(),
                hash_sum: hash,
            };
        }

        Ok(())
    }

    pub fn remove(&mut self, val: &T) -> Result<(), bincode::Error> {
        let size = self.map.len();
        let hash = calc_hash(val, self.hash_builder.build_hasher());
        let val_bin = bincode::serialize(val)?;
        let mut index = hash as usize;
        for _ in 0..self.hash_count {
            index = calc_hash(&index, self.hash_builder.build_hasher()) as usize;
            self.map[index % size] -= Item {
                count: 1,
                val_sum: val_bin.clone(),
                hash_sum: hash,
            };
        }

        Ok(())
    }

    pub fn decode(mut self) -> Result<(HashSet<T>, HashSet<T>), Error> {
        let mut left = HashSet::new();
        let mut right = HashSet::new();

        loop {
            let pure_item = self.map.iter().find(|item| {
                (item.count == 1 || item.count == -1) &&
                item.hash_sum == calc_hash(&item.val_sum, self.hash_builder.build_hasher())
            }).cloned();

            if let Some(item) = pure_item {
                match bincode::deserialize(item.val_sum.as_slice()) {
                    Ok(val) => {
                        self.remove(&val).map_err(|e| Error::SerializeError(e))?;
                        if item.count > 0 {
                            left.insert(val);
                        } else {
                            right.insert(val);
                        }
                    }
                    Err(e) => return Err(Error::MalformedData(e)),
                }
            } else {
                break
            }
        }

        if self.map.iter().all(|item| item.is_empty()) {
            Ok((left, right))
        } else {
            Err(Error::DecodeFail)
        }
    }
}
