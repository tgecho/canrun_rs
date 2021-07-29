use im_rc::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub(crate) struct MKMVMap<K: Eq + Hash + Clone + fmt::Debug, V: Clone> {
    current_id: usize,
    keys: HashMap<K, HashSet<usize>>,
    values: HashMap<usize, Value<K, V>>,
}

impl<K: Eq + Hash + Clone + fmt::Debug, V: Clone> MKMVMap<K, V> {
    pub(crate) fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Clone)]
pub(crate) struct Value<K, V> {
    id: usize,
    pub keys: Vec<K>,
    pub value: V,
}

impl<K: Eq, V: PartialEq> PartialEq for Value<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.keys == other.keys && self.value == other.value
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
    pub(crate) fn new() -> MKMVMap<K, V> {
        MKMVMap {
            current_id: 0,
            keys: HashMap::new(),
            values: HashMap::new(),
        }
    }

    pub(crate) fn add(&mut self, keys: Vec<K>, value: V) {
        let id = self.current_id;
        self.current_id += 1;
        self.keys = keys.iter().fold(self.keys.clone(), |keys, key| {
            keys.alter(
                |existing| Some(existing.map_or_else(|| HashSet::unit(id), |set| set.update(id))),
                key.clone(),
            )
        });
        self.values = self.values.update(id, Value { id, keys, value });
    }

    pub(crate) fn extract(&mut self, key: &K) -> Option<Vec<V>> {
        let (ids, keys) = self.keys.extract(key)?;
        self.keys = keys;
        let mut values = Vec::new();
        for id in ids {
            if let Some((value, value_map)) = self.values.extract(&id) {
                self.values = value_map;
                // This attempts to be "correct" by cleaning up all of the ids
                // when a value is extracted, but this does mean doing a fair
                // amount of work every time. In theory we could one not bother
                // and would only pay a minor cost skipping over the garbage.
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
                values.push(value.value);
            }
        }
        Some(values)
    }
}

impl<K: Eq + Hash + Clone + fmt::Debug, V> fmt::Debug for Value<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value {:?} {:?}", self.id, self.keys)
    }
}

pub(crate) trait DirtyImmutable<T> {
    fn clone_and_push(&self, t: T) -> Self;
}
impl<T: Clone> DirtyImmutable<T> for Vec<T> {
    fn clone_and_push(&self, t: T) -> Self {
        let mut cloned = self.to_vec();
        cloned.push(t);
        cloned
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
}
