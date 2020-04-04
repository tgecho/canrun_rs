use crate::domains::{Domain, DomainType};
use crate::goal::Goal;
use crate::state::{IterResolved, State};
use crate::value::LVar;

// TODO: Come up with better names for these query traits

pub trait StateQuery<'a, D: Domain<'a> + 'a> {
    fn query<Q>(self, query: Q) -> Box<dyn Iterator<Item = Q::Result> + 'a>
    where
        Q: QueryState<'a, D>;
}
impl<'a, D: Domain<'a> + 'a, S: IterResolved<'a, D>> StateQuery<'a, D> for S {
    fn query<Q>(self, query: Q) -> Box<dyn Iterator<Item = Q::Result> + 'a>
    where
        Q: QueryState<'a, D>,
    {
        query.run(self)
    }
}

impl<'a, D: Domain<'a> + 'a> Goal<'a, D> {
    pub fn query<Q>(self, query: Q) -> Box<dyn Iterator<Item = Q::Result> + 'a>
    where
        Q: QueryState<'a, D>,
    {
        let state = self.apply(State::new());
        query.run(state)
    }
}

pub trait QueryState<'a, D: Domain<'a> + 'a> {
    type Result;
    fn run<S: IterResolved<'a, D>>(self, state: S) -> Box<dyn Iterator<Item = Self::Result> + 'a>;
}

impl<'a, D, T> QueryState<'a, D> for LVar<T>
where
    D: Domain<'a> + DomainType<'a, T> + 'a,
    T: Clone + 'a,
{
    type Result = T;
    fn run<S: IterResolved<'a, D>>(self, state: S) -> Box<dyn Iterator<Item = Self::Result> + 'a> {
        Box::new(
            state
                .resolved_iter()
                .filter_map(move |r| r.get(self).ok().cloned()),
        )
    }
}

impl<'a, D, T1, T2> QueryState<'a, D> for (LVar<T1>, LVar<T2>)
where
    D: Domain<'a> + DomainType<'a, T1> + DomainType<'a, T2> + 'a,
    T1: Clone + 'a,
    T2: Clone + 'a,
{
    type Result = (T1, T2);
    fn run<S: IterResolved<'a, D>>(self, state: S) -> Box<dyn Iterator<Item = Self::Result> + 'a> {
        Box::new(state.resolved_iter().filter_map(move |r| {
            Some((r.get(self.0).ok().cloned()?, r.get(self.1).ok().cloned()?))
        }))
    }
}

impl<'a, D, T1, T2, T3> QueryState<'a, D> for (LVar<T1>, LVar<T2>, LVar<T3>)
where
    D: Domain<'a> + DomainType<'a, T1> + DomainType<'a, T2> + DomainType<'a, T3> + 'a,
    T1: Clone + 'a,
    T2: Clone + 'a,
    T3: Clone + 'a,
{
    type Result = (T1, T2, T3);
    fn run<S: IterResolved<'a, D>>(self, state: S) -> Box<dyn Iterator<Item = Self::Result> + 'a> {
        Box::new(state.resolved_iter().filter_map(move |r| {
            Some((
                r.get(self.0).ok().cloned()?,
                r.get(self.1).ok().cloned()?,
                r.get(self.2).ok().cloned()?,
            ))
        }))
    }
}
