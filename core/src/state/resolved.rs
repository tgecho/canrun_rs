use super::{State, WatchFns};
use crate::domains::{Domain, DomainType};
use crate::value::{LVar, Val};

#[derive(Clone)]
pub struct ResolvedState<'a, D: Domain<'a> + 'a> {
    pub(super) domain: D,
    pub(super) watches: WatchFns<'a, D>,
}

// TODO: review if we actually want these duplicate get/resolve_val functions on State and ResolvedState

impl<'a, D: Domain<'a> + 'a> ResolvedState<'a, D> {
    pub fn resolve_val<'r, T>(&'r self, val: &'r Val<T>) -> &'r Val<T>
    where
        D: DomainType<'a, T>,
    {
        match val {
            Val::Var(var) => self.domain.values_as_ref().get(var).unwrap_or(val),
            value => value,
        }
    }

    pub fn get<'g, T>(&'g self, var: LVar<T>) -> Result<&'g T, LVar<T>>
    where
        D: DomainType<'a, T>,
    {
        match self.domain.values_as_ref().get(&var) {
            Some(val) => self.resolve_val(val).resolved(),
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
