use indexmap::{Equivalent, IndexMap};
use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
    hash::{BuildHasher, Hash},
};
use thiserror::Error;

/// Error returned by `MapExt::replace_key`.
#[derive(Error, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ReplaceKeyErr {
    #[error("replace_key: the old key does not exist")]
    /// The old key does not exist.
    OldKeyNotExist,
    #[error("replace_key: the new key is already occupied")]
    /// The new key is already occupied.
    NewKeyOccupied,
}

/// Some general extensions to `Maps` (such as [`HashMap`], [`BTreeMap`], [`IndexMap`]).
pub trait MapExt<K, Q: ?Sized = K> {
    /// Replace an existing key with a new (non-existing) one.
    ///
    /// If k1 does not exist, return `Err(ReplaceKeyErr::OldKeyNotExist)`.
    ///
    /// Otherwise, if k2 and k1 are equal, do nothing and return `Ok(())`.
    ///
    /// Otherwise, if k2 also exists, return `Err(ReplaceKeyErr::NewKeyOccupied)`.
    ///
    /// Otherwise, return `Ok(())` after the replacement is completed.
    fn replace_key(&mut self, k1: &Q, k2: K) -> Result<(), ReplaceKeyErr>
    where
        K: Borrow<Q>;
}

impl<K, Q, V, S> MapExt<K, Q> for HashMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Hash + Eq + ?Sized,
    S: BuildHasher,
{
    fn replace_key(&mut self, k1: &Q, k2: K) -> Result<(), ReplaceKeyErr> {
        if !self.contains_key(k1) {
            return Err(ReplaceKeyErr::OldKeyNotExist);
        }

        if k1 == k2.borrow() {
            return Ok(());
        }

        if self.contains_key(k2.borrow()) {
            return Err(ReplaceKeyErr::NewKeyOccupied);
        }

        let v = self.remove(k1).expect("this should be unreachable");
        self.insert(k2, v);
        Ok(())
    }
}

impl<K, Q, V> MapExt<K, Q> for BTreeMap<K, V>
where
    K: Borrow<Q> + Ord,
    Q: Ord + ?Sized,
{
    fn replace_key(&mut self, k1: &Q, k2: K) -> Result<(), ReplaceKeyErr> {
        if !self.contains_key(k1) {
            return Err(ReplaceKeyErr::OldKeyNotExist);
        }

        if k1 == k2.borrow() {
            return Ok(());
        }

        if self.contains_key(k2.borrow()) {
            return Err(ReplaceKeyErr::NewKeyOccupied);
        }

        let v = self.remove(k1).expect("this should be unreachable");
        self.insert(k2, v);
        Ok(())
    }
}

impl<K, Q, V, S> MapExt<K, Q> for IndexMap<K, V, S>
where
    K: Borrow<Q> + Hash + Eq,
    Q: ?Sized + Hash + Equivalent<K>,
    S: BuildHasher,
{
    fn replace_key(&mut self, k1: &Q, k2: K) -> Result<(), ReplaceKeyErr> {
        let Some(i) = self.get_index_of(k1) else {
            return Err(ReplaceKeyErr::OldKeyNotExist);
        };

        if k1.equivalent(&k2) {
            return Ok(());
        }

        if self.contains_key(k2.borrow()) {
            return Err(ReplaceKeyErr::NewKeyOccupied);
        }

        // Note, this temporarily displaces the last entry into `i`,
        // but we'll swap it back after we insert the new key.
        // See: https://github.com/indexmap-rs/indexmap/issues/362
        let Some((_, v)) = self.swap_remove_index(i) else {
            return Err(ReplaceKeyErr::OldKeyNotExist);
        };

        let (j, _) = self.insert_full(k2, v);
        self.swap_indices(i, j);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_key_hashmap() {
        let mut map = HashMap::new();

        map.insert("k1".to_string(), 123);
        map.insert("k2".to_string(), 456);

        assert_eq!(map["k1"], 123);
        assert_eq!(map["k2"], 456);
        assert_eq!(
            map.replace_key("k3", "k2".to_string()),
            Err(ReplaceKeyErr::OldKeyNotExist)
        );
        assert_eq!(
            map.replace_key("k3", "k3".to_string()),
            Err(ReplaceKeyErr::OldKeyNotExist)
        );
        assert_eq!(
            map.replace_key("k3", "k4".to_string()),
            Err(ReplaceKeyErr::OldKeyNotExist)
        );

        let cloned = map.clone();
        assert_eq!(map.replace_key("k1", "k1".to_string()), Ok(()));
        assert_eq!(map, cloned);

        assert_eq!(
            map.replace_key("k1", "k2".to_string()),
            Err(ReplaceKeyErr::NewKeyOccupied)
        );
        assert_eq!(map.replace_key("k1", "k3".to_string()), Ok(()));
        assert!(!map.contains_key("k1"));
        assert_eq!(map["k3"], 123);
        assert_eq!(map["k2"], 456);
    }

    #[test]
    fn replace_key_btreemap() {
        let mut map = BTreeMap::new();

        map.insert("k1".to_string(), 123);
        map.insert("k2".to_string(), 456);

        assert_eq!(map["k1"], 123);
        assert_eq!(map["k2"], 456);
        assert_eq!(
            map.replace_key("k3", "k2".to_string()),
            Err(ReplaceKeyErr::OldKeyNotExist)
        );
        assert_eq!(
            map.replace_key("k3", "k3".to_string()),
            Err(ReplaceKeyErr::OldKeyNotExist)
        );
        assert_eq!(
            map.replace_key("k3", "k4".to_string()),
            Err(ReplaceKeyErr::OldKeyNotExist)
        );

        let cloned = map.clone();
        assert_eq!(map.replace_key("k1", "k1".to_string()), Ok(()));
        assert_eq!(map, cloned);

        assert_eq!(
            map.replace_key("k1", "k2".to_string()),
            Err(ReplaceKeyErr::NewKeyOccupied)
        );
        assert_eq!(map.replace_key("k1", "k3".to_string()), Ok(()));
        assert!(!map.contains_key("k1"));
        assert_eq!(map["k3"], 123);
        assert_eq!(map["k2"], 456);
    }

    #[test]
    fn replace_key_indexmap() {
        let mut map = indexmap::indexmap! {
            "k1".to_string() => 123,
            "k2".to_string() => 456
        };

        assert_eq!(map["k1"], 123);
        assert_eq!(map["k2"], 456);
        assert_eq!(map.get_index_of("k1"), Some(0));
        assert_eq!(map.get_index(0), Some((&"k1".to_string(), &123)));

        assert_eq!(
            map.replace_key("k3", "k2".to_string()),
            Err(ReplaceKeyErr::OldKeyNotExist)
        );
        assert_eq!(
            map.replace_key("k3", "k3".to_string()),
            Err(ReplaceKeyErr::OldKeyNotExist)
        );
        assert_eq!(
            map.replace_key("k3", "k4".to_string()),
            Err(ReplaceKeyErr::OldKeyNotExist)
        );

        let cloned = map.clone();
        assert_eq!(map.replace_key("k1", "k1".to_string()), Ok(()));
        assert_eq!(map, cloned);

        assert_eq!(
            map.replace_key("k1", "k2".to_string()),
            Err(ReplaceKeyErr::NewKeyOccupied)
        );

        assert_eq!(map.replace_key("k1", "k3".to_string()), Ok(()));
        assert!(!map.contains_key("k1"));
        assert_eq!(map["k3"], 123);
        assert_eq!(map["k2"], 456);
        assert_eq!(map.get_index_of("k3"), Some(0));
        assert_eq!(map.get_index(0), Some((&"k3".to_string(), &123)));
    }
}
