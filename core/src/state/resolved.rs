use super::{State, WatchFns};
use crate::domains::{Domain, DomainType};
use crate::value::LVar;

#[derive(Clone)]
pub struct ResolvedState<'a, D: Domain<'a> + 'a> {
    pub(super) domain: D,
    pub(super) watches: WatchFns<'a, D>,
}

impl<'a, D: Domain<'a> + 'a> ResolvedState<'a, D> {
    pub fn get<'g, T>(&'g self, var: LVar<T>) -> Result<&'g T, LVar<T>>
    where
        D: DomainType<'a, T>,
    {
        match self.domain.values_as_ref().get(&var) {
            Some(val) => val.resolved(),
            None => Err(var),
        }
    }

    pub fn reopen(self) -> State<'a, D> {
        State {
            domain: self.domain,
            watches: self.watches,
            forks: im::Vector::new(),
        }
    }
}
