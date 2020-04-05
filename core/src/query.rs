use crate::domains::Domain;
use crate::state::IterResolved;

mod query_impls;

pub trait Queryable<'a, D: Domain<'a> + 'a> {
    fn query<Q>(self, query: Q) -> Box<dyn Iterator<Item = Q::Result> + 'a>
    where
        Q: Query<'a, D>;
}

impl<'a, D: Domain<'a> + 'a, S: IterResolved<'a, D>> Queryable<'a, D> for S {
    fn query<Q>(self, query: Q) -> Box<dyn Iterator<Item = Q::Result> + 'a>
    where
        Q: Query<'a, D>,
    {
        query.query_in(self)
    }
}

/// A struct that is able to query for resolved values in a
/// [ResolvedState](crate::state::ResolvedState).
pub trait Query<'a, D: Domain<'a> + 'a> {
    type Result;
    fn query_in<S: IterResolved<'a, D>>(
        self,
        state: S,
    ) -> Box<dyn Iterator<Item = Self::Result> + 'a>;
}
