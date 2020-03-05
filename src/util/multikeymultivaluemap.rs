use im::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub(crate) struct MKMVMap<K: Eq + Hash + Clone + fmt::Debug, V: Clone> {
    current_id: usize,
    keys: HashMap<K, HashSet<usize>>,
    values: HashMap<usize, Value<K, V>>,
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

impl<K: Eq + Hash + Clone + fmt::Debug, V: Clone + PartialEq + fmt::Debug> PartialEq
    for MKMVMap<K, V>
{
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

    pub(crate) fn add(&self, keys: Vec<K>, value: V) -> Self {
        let id = self.current_id;
        MKMVMap {
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

    pub(crate) fn extract(&self, key: &K) -> (Self, Vec<V>) {
        self.keys.get(&key).iter().flat_map(|set| set.iter()).fold(
            (self.clone(), vec![] as Vec<V>),
            |(mkmv, values), id| match mkmv.values.extract(id) {
                // This attempts to be "correct" by cleaning up all of the ids
                // when a value is extracted, but this does mean doing a fair
                // amount of work every time. In theory we could just not bother
                // and would only pay a slight in skipping over the garbage.
                None => (mkmv, values),
                Some((value, value_map)) => {
                    let keys = mkmv.keys.alter(
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
                    let mkmv = MKMVMap {
                        current_id: mkmv.current_id,
                        values: value_map,
                        keys,
                    };
                    let values = values.clone_and_push(value.value);
                    (mkmv, values)
                }
            },
        )
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
        let map: MKMVMap<usize, usize> = MKMVMap::new();
        let (_, values) = map.extract(&1);
        assert_eq!(values, vec![]);
    }

    #[test]
    fn add_and_extract() {
        let map = MKMVMap::new().add(vec![1, 2], "12");
        let (updated, values) = map.extract(&1);
        assert_eq!(values, vec!["12"]);
        assert!(updated.values.is_empty());
    }
}
