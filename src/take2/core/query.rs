use super::domain::{Domain, DomainType};
use super::state::{IterResolved, ResolvedState};
use super::val::Val;

pub trait QueryState<'a, D: Domain> {
    type ResultType;
    fn resolve_in<S: IterResolved<'a, D>>(
        &'a self,
        state: S,
    ) -> Box<dyn Iterator<Item = Self::ResultType> + 'a>;
}

fn resolve_with<'a, D, S, F, R>(state: S, func: F) -> Box<dyn Iterator<Item = R> + 'a>
where
    S: IterResolved<'a, D>,
    D: Domain + 'a,
    F: Fn(ResolvedState<'a, D>) -> Option<R> + 'a,
{
    Box::new(state.iter_resolved().filter_map(func))
}

impl<'a, D, T> QueryState<'a, D> for Val<T>
where
    T: Clone + 'a,
    D: Domain + DomainType<T> + 'a,
{
    type ResultType = T;
    fn resolve_in<S: IterResolved<'a, D>>(
        &'a self,
        state: S,
    ) -> Box<dyn Iterator<Item = Self::ResultType> + 'a> {
        resolve_with(state, move |r| r.get(self))
    }
}

impl<'a, D, T1, T2> QueryState<'a, D> for (Val<T1>, Val<T2>)
where
    T1: Clone + 'a,
    T2: Clone + 'a,
    D: Domain + DomainType<T1> + DomainType<T2> + 'a,
{
    type ResultType = (T1, T2);
    fn resolve_in<S: IterResolved<'a, D>>(
        &'a self,
        state: S,
    ) -> Box<dyn Iterator<Item = Self::ResultType> + 'a> {
        resolve_with(state, move |r| Some((r.get(&self.0)?, r.get(&self.1)?)))
    }
}
