use im::{HashMap, HashSet};
use std::hash::Hash;

pub struct MultiKeyMultiValueMap<K: Eq + Hash + Clone, V: Clone> {
    current_id: usize,
    keys: HashMap<K, HashSet<usize>>,
    values: HashMap<usize, Value<K, V>>,
}

#[derive(Clone)]
pub struct Value<K, V> {
    id: usize,
    keys: Vec<K>,
    value: V,
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

    pub fn add(&self, keys: Vec<K>, value: V) -> Self {
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
    fn add() {
        let map = MultiKeyMultiValueMap::new().add(vec![1, 2], "12");
        let values: Vec<_> = map.get(&1).iter().map(|v| v.value).collect();
        assert_eq!(values, vec!["12"]);
    }

    #[test]
    fn remove() {
        let map = MultiKeyMultiValueMap::new().add(vec![1, 2], "12");
        let map = map.remove(map.get(&1)[0]);
        assert!(map.get(&1).is_empty());
    }
}
