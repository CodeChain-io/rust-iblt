use std::collections::hash_map::{DefaultHasher, HashMap};
use std::fmt::Debug;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};
use std::marker::PhantomData;

use bincode;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use item::Item;

fn calc_hash<T, H>(val: &T, mut hasher: H) -> u64
where
    T: Hash,
    H: Hasher, {
    val.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug)]
pub enum Error {
    DecodeFail,
    MalformedData(bincode::Error),
}

pub struct IBLT<K, V, S = BuildHasherDefault<DefaultHasher>> {
    hash_count: usize,
    map: Vec<Item>,
    hash_builder: S,
    phantom: PhantomData<*const (K, V)>,
}

impl<K, V> IBLT<K, V, BuildHasherDefault<DefaultHasher>>
where
    K: Eq + Hash + DeserializeOwned + Serialize + Debug,
    V: Eq + Hash + DeserializeOwned + Serialize,
{
    pub fn new(size: usize, hash_count: usize) -> Self {
        Self::with_hasher(size, hash_count, Default::default())
    }
}

pub type DecodeResult<K, V> = Result<(HashMap<K, V>, HashMap<K, V>), Error>;

impl<K, V, S> IBLT<K, V, S>
where
    K: Eq + Hash + DeserializeOwned + Serialize + Debug,
    V: Eq + Hash + DeserializeOwned + Serialize,
    S: BuildHasher,
{
    pub fn with_hasher(size: usize, hash_count: usize, hash_builder: S) -> Self {
        Self {
            hash_count,
            map: vec![Item::default(); size],
            hash_builder,
            phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, key: &K, val: &V) -> Result<(), bincode::Error> {
        let size = self.map.len();
        let key_bin = bincode::serialize(key)?;
        let val_bin = bincode::serialize(val)?;
        let hash = calc_hash(&key_bin, self.hash_builder.build_hasher());
        let mut index = hash as usize;
        for _ in 0..self.hash_count {
            index = calc_hash(&index, self.hash_builder.build_hasher()) as usize;
            self.map[index % size] += Item {
                count: 1,
                key_sum: key_bin.clone(),
                val_sum: val_bin.clone(),
                hash_sum: hash,
            };
        }

        Ok(())
    }

    pub fn remove(&mut self, key: &K, val: &V) -> Result<(), bincode::Error> {
        let size = self.map.len();
        let key_bin = bincode::serialize(key)?;
        let val_bin = bincode::serialize(val)?;
        let hash = calc_hash(&key_bin, self.hash_builder.build_hasher());
        let mut index = hash as usize;
        for _ in 0..self.hash_count {
            index = calc_hash(&index, self.hash_builder.build_hasher()) as usize;
            self.map[index % size] -= Item {
                count: 1,
                key_sum: key_bin.clone(),
                val_sum: val_bin.clone(),
                hash_sum: hash,
            };
        }

        Ok(())
    }

    pub fn decode(mut self) -> DecodeResult<K, V> {
        let mut left = HashMap::new();
        let mut right = HashMap::new();
        loop {
            let pure_item = self
                .map
                .iter()
                .find(|item| {
                    (item.count == 1 || item.count == -1)
                        && item.hash_sum == calc_hash(&item.key_sum, self.hash_builder.build_hasher())
                })
                .cloned();

            if let Some(item) = pure_item {
                let key = bincode::deserialize(item.key_sum.as_slice()).map_err(Error::MalformedData)?;
                let val = bincode::deserialize(item.val_sum.as_slice()).map_err(Error::MalformedData)?;
                self.remove(&key, &val).unwrap();
                if item.count > 0 {
                    left.insert(key, val);
                } else {
                    right.insert(key, val);
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
