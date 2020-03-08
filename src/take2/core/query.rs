use super::domain::{Domain, DomainType};
use super::state::IterResolved;
use super::val::Val;

pub trait QueryState<'a, D: Domain, R> {
    type Query;
    type Result;
    fn query(self, query: Self::Query) -> Box<dyn Iterator<Item = Self::Result> + 'a>;
}

impl<'a, D, S, T> QueryState<'a, D, (T,)> for S
where
    D: Domain + DomainType<T> + 'a,
    S: IterResolved<'a, D> + 'a,
    T: Clone + 'a,
{
    type Query = (Val<T>,);
    type Result = (T,);
    fn query(self, query: Self::Query) -> Box<dyn Iterator<Item = Self::Result> + 'a> {
        Box::new(
            self.resolved_iter()
                .filter_map(move |r| Some((r.get(&query.0)?,))),
        )
    }
}

impl<'a, D, S, T1, T2> QueryState<'a, D, (T1, T2)> for S
where
    D: Domain + DomainType<T1> + DomainType<T2> + 'a,
    S: IterResolved<'a, D> + 'a,
    T1: Clone + 'a,
    T2: Clone + 'a,
{
    type Query = (Val<T1>, Val<T2>);
    type Result = (T1, T2);
    fn query(self, query: Self::Query) -> Box<dyn Iterator<Item = Self::Result> + 'a> {
        Box::new(
            self.resolved_iter()
                .filter_map(move |r| Some((r.get(&query.0)?, r.get(&query.1)?))),
        )
    }
}

impl<'a, D, S, T1, T2, T3> QueryState<'a, D, (T1, T2, T3)> for S
where
    D: Domain + DomainType<T1> + DomainType<T2> + DomainType<T3> + 'a,
    S: IterResolved<'a, D> + 'a,
    T1: Clone + 'a,
    T2: Clone + 'a,
    T3: Clone + 'a,
{
    type Query = (Val<T1>, Val<T2>, Val<T3>);
    type Result = (T1, T2, T3);
    fn query(self, query: Self::Query) -> Box<dyn Iterator<Item = Self::Result> + 'a> {
        Box::new(
            self.resolved_iter()
                .filter_map(move |r| Some((r.get(&query.0)?, r.get(&query.1)?, r.get(&query.2)?))),
        )
    }
}
