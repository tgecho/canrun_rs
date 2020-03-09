use crate::core::domain::{Domain, DomainType};
use crate::core::state::IterResolved;
use crate::value::Val;

pub trait StateQuery<'a, D: Domain<'a> + 'a> {
    fn query<Q: QueryState<'a, D>>(self, query: Q) -> Box<dyn Iterator<Item = Q::Result> + 'a>;
}
impl<'a, D: Domain<'a> + 'a, S: IterResolved<'a, D>> StateQuery<'a, D> for S {
    fn query<Q: QueryState<'a, D>>(self, query: Q) -> Box<dyn Iterator<Item = Q::Result> + 'a> {
        query.query(self)
    }
}

pub trait QueryState<'a, D: Domain<'a> + 'a> {
    type Result;
    fn query<S: IterResolved<'a, D>>(self, state: S)
        -> Box<dyn Iterator<Item = Self::Result> + 'a>;
}

impl<'a, D, T> QueryState<'a, D> for &'a Val<T>
where
    D: Domain<'a> + DomainType<'a, T> + 'a,
    T: Clone + 'a,
{
    type Result = T;
    fn query<S: IterResolved<'a, D>>(
        self,
        state: S,
    ) -> Box<dyn Iterator<Item = Self::Result> + 'a> {
        Box::new(state.resolved_iter().filter_map(move |r| r.get(self)))
    }
}

impl<'a, D, T1, T2> QueryState<'a, D> for (&'a Val<T1>, &'a Val<T2>)
where
    D: Domain<'a> + DomainType<'a, T1> + DomainType<'a, T2> + 'a,
    T1: Clone + 'a,
    T2: Clone + 'a,
{
    type Result = (T1, T2);
    fn query<S: IterResolved<'a, D>>(
        self,
        state: S,
    ) -> Box<dyn Iterator<Item = Self::Result> + 'a> {
        Box::new(
            state
                .resolved_iter()
                .filter_map(move |r| Some((r.get(self.0)?, r.get(self.1)?))),
        )
    }
}

impl<'a, D, T1, T2, T3> QueryState<'a, D> for (&'a Val<T1>, &'a Val<T2>, &'a Val<T3>)
where
    D: Domain<'a> + DomainType<'a, T1> + DomainType<'a, T2> + DomainType<'a, T3> + 'a,
    T1: Clone + 'a,
    T2: Clone + 'a,
    T3: Clone + 'a,
{
    type Result = (T1, T2, T3);
    fn query<S: IterResolved<'a, D>>(
        self,
        state: S,
    ) -> Box<dyn Iterator<Item = Self::Result> + 'a> {
        Box::new(
            state
                .resolved_iter()
                .filter_map(move |r| Some((r.get(self.0)?, r.get(self.1)?, r.get(self.2)?))),
        )
    }
}
