use super::{Reify, StateIterator};

pub trait Query<'a> {
    fn query<Q: Reify + 'a>(self, query: Q) -> Box<dyn Iterator<Item = Q::Reified> + 'a>;
}

impl<'a, S: StateIterator + 'a> Query<'a> for S {
    fn query<Q: Reify + 'a>(self, query: Q) -> Box<dyn Iterator<Item = Q::Reified> + 'a> {
        Box::new(
            self.into_states()
                .filter_map(move |resolved| query.reify_in(&resolved)),
        )
    }
}
