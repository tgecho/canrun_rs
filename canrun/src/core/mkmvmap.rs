use im_rc::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct MKMVMap<K: Eq + Hash + Clone + fmt::Debug, V: Clone> {
    current_id: usize,
    keys: HashMap<K, HashSet<usize>>,
    values: HashMap<usize, Value<K, V>>,
}

#[derive(Clone)]
pub(crate) struct Value<K, V> {
    id: usize,
    keys: Vec<K>,
    data: V,
}

impl<K: Eq, V: PartialEq> PartialEq for Value<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.keys == other.keys && self.data == other.data
    }
}

impl<K: Eq + Hash + Clone + fmt::Debug, V: Clone + PartialEq> PartialEq for MKMVMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.current_id == other.current_id
            && self.keys == other.keys
            && self.values == other.values
    }
}

impl<K: Eq + Hash + Clone + fmt::Debug, V: Clone> MKMVMap<K, V> {
    pub fn new() -> MKMVMap<K, V> {
        MKMVMap {
            current_id: 0,
            keys: HashMap::new(),
            values: HashMap::new(),
        }
    }

    pub fn add(&mut self, keys: Vec<K>, data: V) {
        let id = self.current_id;
        self.current_id += 1;
        self.keys = keys.iter().fold(self.keys.clone(), |keys, key| {
            keys.alter(
                |existing| Some(existing.map_or_else(|| HashSet::unit(id), |set| set.update(id))),
                key.clone(),
            )
        });
        self.values = self.values.update(id, Value { id, keys, data });
    }

    pub fn extract(&mut self, key: &K) -> Option<Vec<V>> {
        let (ids, keys) = self.keys.extract(key)?;
        self.keys = keys;
        let mut values = Vec::new();
        for id in ids {
            if let Some((value, value_map)) = self.values.extract(&id) {
                self.values = value_map;
                // This attempts to be "correct" by cleaning up all of the ids
                // when a value is extracted, but this does mean doing a fair
                // amount of work every time. In theory we could one not bother
                // and would only pay a minor cost skipping over the garbage,
                // except we have some other areas that depend on this being an
                // accurate reflection of what is actually being watched.
                self.keys = self.keys.alter(
                    |existing| {
                        let updated = existing?.without(&value.id);
                        if updated.is_empty() {
                            None
                        } else {
                            Some(updated)
                        }
                    },
                    key.clone(),
                );
                values.push(value.data);
            }
        }
        Some(values)
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.keys.keys()
    }
}

impl<K: Eq + Hash + Clone + fmt::Debug, V> fmt::Debug for Value<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value {:?} {:?}", self.id, self.keys)
    }
}

#[cfg(test)]
mod tests {
    use super::MKMVMap;

    #[test]
    fn empty() {
        let mut map: MKMVMap<usize, usize> = MKMVMap::new();
        let values = map.extract(&1);
        assert_eq!(values, None);
    }

    #[test]
    fn add_and_extract() {
        let mut map = MKMVMap::new();
        map.add(vec![1, 2], "12");
        let values = map.extract(&1);
        assert_eq!(values, Some(vec!["12"]));
        assert!(map.values.is_empty());
    }

    #[test]
    fn value_eq() {
        let mut a1: MKMVMap<usize, usize> = MKMVMap::new();
        a1.add(vec![1], 1);

        let mut a2: MKMVMap<usize, usize> = MKMVMap::new();
        a2.add(vec![1], 1);

        let b: MKMVMap<usize, usize> = MKMVMap::new();

        assert_eq!(a1, a2);
        assert_ne!(a1, b);
    }

    #[test]
    fn debug_impl() {
        let mut a1: MKMVMap<usize, usize> = MKMVMap::new();
        a1.add(vec![1], 1);

        assert_ne!(format!("{a1:?}"), "");
    }
}
