use im::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

#[derive(Clone)]
pub struct MultiKeyMultiValueMap<K: Eq + Hash + Clone, V: Clone> {
    current_id: usize,
    keys: HashMap<K, HashSet<usize>>,
    values: HashMap<usize, Value<K, V>>,
}

#[derive(Clone)]
pub struct Value<K, V> {
    id: usize,
    pub keys: Vec<K>,
    pub value: V,
}

impl<K: Eq, V: PartialEq> PartialEq for Value<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.keys == other.keys && self.value == other.value
    }
}

impl<K: Eq + Hash + Clone, V: Clone + PartialEq> PartialEq for MultiKeyMultiValueMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.current_id == other.current_id
            && self.keys == other.keys
            && self.values == other.values
    }
}

impl<K: Eq + Hash + Clone, V: Clone> MultiKeyMultiValueMap<K, V> {
    pub fn new() -> MultiKeyMultiValueMap<K, V> {
        MultiKeyMultiValueMap {
            current_id: 0,
            keys: HashMap::new(),
            values: HashMap::new(),
        }
    }

    pub fn get(&self, lvar: &K) -> Vec<&Value<K, V>> {
        self.keys
            .get(lvar)
            .iter()
            .flat_map(|set| set.iter())
            .filter_map(|id| self.values.get(id))
            .collect()
    }

    pub fn set(&self, keys: Vec<K>, value: V) -> Self {
        let id = self.current_id;
        MultiKeyMultiValueMap {
            current_id: id + 1,
            keys: keys.iter().fold(self.keys.clone(), |keys, key| {
                keys.alter(
                    |existing| {
                        Some(existing.map_or_else(|| HashSet::unit(id), |set| set.update(id)))
                    },
                    key.clone(),
                )
            }),
            values: self.values.update(id, Value { id, keys, value }),
        }
    }

    pub fn add_key(&self, key: K, value: &Value<K, V>) -> Self {
        let id = value.id;
        MultiKeyMultiValueMap {
            current_id: self.current_id,
            keys: self.keys.alter(
                |existing| Some(existing.map_or_else(|| HashSet::unit(id), |set| set.update(id))),
                key,
            ),
            values: self.values.clone(),
        }
    }

    pub fn remove(&self, stored: &Value<K, V>) -> Self {
        let id = stored.id;
        let keys = stored.keys.iter().fold(self.keys.clone(), |keys, key| {
            keys.alter(
                |existing| {
                    let updated = existing?.without(&id);
                    if updated.is_empty() {
                        None
                    } else {
                        Some(updated)
                    }
                },
                key.clone(),
            )
        });
        MultiKeyMultiValueMap {
            current_id: self.current_id,
            keys,
            values: self.values.without(&stored.id),
        }
    }
}

impl<K: Eq + Hash + Clone, V: Clone> fmt::Debug for MultiKeyMultiValueMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MultiKeyMultiValueMap {{}}")
    }
}

impl<K: Eq + Hash + Clone + fmt::Debug, V: Clone + fmt::Debug> fmt::Debug for Value<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value {:?} {:?} {:?}", self.id, self.keys, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::MultiKeyMultiValueMap;

    #[test]
    fn empty() {
        let map: MultiKeyMultiValueMap<usize, usize> = MultiKeyMultiValueMap::new();
        let values: Vec<_> = map.get(&1).iter().map(|v| v.value).collect();
        assert_eq!(values, vec![]);
    }

    #[test]
    fn set() {
        let map = MultiKeyMultiValueMap::new().set(vec![1, 2], "12");
        let values: Vec<_> = map.get(&1).iter().map(|v| v.value).collect();
        assert_eq!(values, vec!["12"]);
    }

    #[test]
    fn remove() {
        let map = MultiKeyMultiValueMap::new().set(vec![1, 2], "12");
        let map = map.remove(map.get(&1)[0]);
        assert!(map.get(&1).is_empty());
    }
}
