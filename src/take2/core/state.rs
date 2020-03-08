use super::domain::{Domain, DomainType};
use super::val::{Val, Val::Var};
use crate::can::lvar::LVar;
use crate::util::multikeymultivaluemap::MKMVMap;
use std::iter::once;
use std::rc::Rc;

pub type StateIter<'s, D> = Box<dyn Iterator<Item = State<'s, D>> + 's>;
pub type ResolvedIter<'s, D> = Box<dyn Iterator<Item = ResolvedState<'s, D>> + 's>;
type WatchFns<'s, D> = MKMVMap<LVar, Rc<dyn Fn(State<'s, D>) -> WatchResult<State<'s, D>> + 's>>;

#[derive(Clone)]
pub struct State<'a, D: Domain> {
    domain: D,
    watches: WatchFns<'a, D>,
    forks: im::Vector<Rc<dyn Fn(Self) -> StateIter<'a, D> + 'a>>,
}

#[derive(Clone)]
pub struct ResolvedState<'a, D: Domain> {
    domain: D,
    watches: WatchFns<'a, D>,
}

impl<'a, D: Domain> ResolvedState<'a, D> {
    pub fn get_rc<T>(&self, key: &Val<T>) -> Option<Rc<T>>
    where
        D: DomainType<T>,
    {
        match key {
            Val::Var(var) => self
                .domain
                .values_as_ref()
                .get(var)
                .and_then(|k| self.get_rc(k)),
            Val::Resolved(resolved) => Some(resolved.clone()),
        }
    }

    pub fn get<T>(&self, key: &Val<T>) -> Option<T>
    where
        T: Clone,
        D: DomainType<T>,
    {
        self.get_rc(key).map(|rc| (*rc).clone())
    }

    pub fn reopen(self) -> State<'a, D> {
        State {
            domain: self.domain,
            watches: self.watches,
            forks: im::Vector::new(),
        }
    }
}

pub trait IterResolved<'a, D: Domain> {
    fn resolved_iter(self) -> ResolvedIter<'a, D>;
}
impl<'a, D: Domain + 'a> IterResolved<'a, D> for State<'a, D> {
    fn resolved_iter(self) -> ResolvedIter<'a, D> {
        Box::new(self.iter_forks().map(|s| ResolvedState {
            domain: s.domain,
            watches: s.watches,
        }))
    }
}
impl<'a, D: Domain + 'a> IterResolved<'a, D> for Result<State<'a, D>, State<'a, D>> {
    fn resolved_iter(self) -> ResolvedIter<'a, D> {
        Box::new(self.into_iter().flat_map(|s| s.resolved_iter()))
    }
}
impl<'a, D: Domain + 'a> IterResolved<'a, D> for Vec<ResolvedState<'a, D>> {
    fn resolved_iter(self) -> ResolvedIter<'a, D> {
        Box::new(self.into_iter())
    }
}

#[derive(Debug)]
pub(crate) enum WatchResult<State> {
    Done(Result<State, State>),
    Waiting(State, Vec<LVar>),
}

// pub fn run<'a, D: Domain + 'a, F: Fn(State<D>) -> Result<State<D>, State<D>>>(
//     func: F,
// ) -> ResolvedIter<'a, D> {
//     match func(State::new()) {
//         Err(_) => Box::new(std::iter::empty()),
//         Ok(state) => state.resolved_iter(),
//     }
// }

impl<'a, D: Domain + 'a> State<'a, D> {
    pub fn new() -> Self {
        State {
            domain: D::new(),
            watches: MKMVMap::new(),
            forks: im::Vector::new(),
        }
    }

    pub fn apply<F>(self, func: F) -> Result<Self, Self>
    where
        F: Fn(Self) -> Result<Self, Self>,
    {
        func(self)
    }

    fn iter_forks(mut self) -> StateIter<'a, D> {
        let fork = self.forks.pop_front();
        match fork {
            None => Box::new(once(self)),
            Some(fork) => Box::new(fork(self).flat_map(|s: State<'a, D>| s.iter_forks())),
        }
    }

    pub(super) fn resolve<'r, T>(&'r self, key: &'r Val<T>) -> &'r Val<T>
    where
        D: DomainType<T>,
    {
        match key {
            Val::Var(var) => self.domain.values_as_ref().get(var).unwrap_or(key),
            value => value,
        }
    }

    pub(super) fn unify<T>(mut self, a: Val<T>, b: Val<T>) -> Result<Self, Self>
    where
        T: PartialEq,
        D: DomainType<T>,
    {
        let a = self.resolve(&a);
        let b = self.resolve(&b);
        match (a, b) {
            (a, b) if a == b => Ok(self),
            (Var(var), val) | (val, Var(var)) => {
                let key = *var;
                let value = val.clone();

                // TODO: Add occurs check?

                // Assign lvar to value
                self.domain.values_as_mut().insert(key, value);

                // check watches matching newly assigned lvar
                if let Some(watches) = self.watches.extract(&key) {
                    watches
                        .into_iter()
                        .try_fold(self, |state, func| state.watch(func))
                } else {
                    Ok(self)
                }
            }
            _ => Err(self),
        }
    }

    pub(super) fn watch(
        self,
        func: Rc<dyn Fn(Self) -> WatchResult<Self> + 'a>,
    ) -> Result<Self, Self> {
        match func(self) {
            WatchResult::Done(state) => state,
            WatchResult::Waiting(mut state, vars) => {
                state.watches.add(vars, func);
                Ok(state)
            }
        }
    }

    pub(super) fn fork(
        mut self,
        func: Rc<dyn Fn(Self) -> StateIter<'a, D> + 'a>,
    ) -> Result<Self, Self> {
        self.forks.push_back(func);
        Ok(self)
    }
}

use std::fmt;
impl<'a, D: Domain> fmt::Debug for State<'a, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "State ??")
    }
}
