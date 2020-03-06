use crate::can::lvar::LVar;
use crate::take2::domain::{Domain, DomainType};
use crate::take2::val::{Val, Val::Var};
use crate::util::multikeymultivaluemap::MKMVMap;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct State<'a, D: Domain> {
    domain: D,
    watches: MKMVMap<LVar, Rc<dyn Fn(Self) -> WatchResult<Self> + 'a>>,
    forks: im::Vector<Rc<dyn Fn(Self) -> StateIter<'a, Self> + 'a>>,
}

impl<'a, D: Domain + 'a> State<'a, D> {
    pub(crate) fn new() -> Self {
        State {
            domain: D::new(),
            watches: MKMVMap::new(),
            forks: im::Vector::new(),
        }
    }

    pub(crate) fn run(&self) -> StateIter<'a, Self> {
        let mut state = self.clone();
        let fork = state.forks.pop_front();
        match fork {
            None => Box::new(std::iter::once(state)),
            Some(fork) => Box::new(fork(state).flat_map(|s| s.run())),
        }
    }

    pub(crate) fn resolve<'r, T>(&'r self, key: &'r Val<T>) -> &'r Val<T>
    where
        D: DomainType<T>,
    {
        match key {
            Val::Var(var) => self.domain.values_as_ref().get(var).unwrap_or(key),
            value => value,
        }
    }

    pub(crate) fn unify<T>(mut self, a: Val<T>, b: Val<T>) -> Result<Self, Self>
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
                let (watches, extracted) = self.watches.extract(&key);
                self.watches = watches;
                (extracted.into_iter()).try_fold(self, |state, func| state.watch(func))
            }
            _ => Err(self),
        }
    }

    pub(crate) fn watch<T>(
        self,
        func: Rc<dyn Fn(Self) -> WatchResult<Self> + 'a>,
    ) -> Result<Self, Self>
    where
        D: DomainType<T>,
    {
        match func(self) {
            WatchResult::Done(state) => state,
            WatchResult::Waiting(state, vars) => {
                state.watches.add(vars, func);
                Ok(state)
            }
        }
    }

    pub(crate) fn fork<T, F>(mut self, func: F) -> Result<Self, Self>
    where
        D: DomainType<T>,
        F: Fn(Self) -> StateIter<'a, Self> + 'a,
    {
        self.forks.push_back(Rc::new(func));
        Ok(self)
    }
}

// TODO: Naming?
pub(crate) enum WatchResult<State> {
    Done(Result<State, State>),
    Waiting(State, Vec<LVar>), // TODO: does this need to be by T row?
}

pub type StateIter<'s, State> = Box<dyn Iterator<Item = State> + 's>;
