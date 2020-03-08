use super::domain::Domain;
use super::state::{State, WatchResult};
use std::fmt;
use std::iter::repeat;
use std::rc::Rc;

#[derive(Clone)]
pub struct Thunk<'a, D: Domain + 'a>(Rc<dyn Fn(State<'a, D>) -> Option<State<'a, D>> + 'a>);

#[derive(Clone, Debug)]
pub enum Goal<'a, D: Domain> {
    Both(Box<Goal<'a, D>>, Box<Goal<'a, D>>),
    All(Vec<Goal<'a, D>>),
    Either(Box<Goal<'a, D>>, Box<Goal<'a, D>>),
    Any(Vec<Goal<'a, D>>),
    Thunk(Thunk<'a, D>),
}

impl<'a, D: Domain + 'a> Goal<'a, D> {
    pub(crate) fn apply(self, state: State<'a, D>) -> Option<State<'a, D>> {
        match self {
            Goal::Both(a, b) => a.apply(state).and_then(|s| b.apply(s)),
            Goal::All(goals) => goals.into_iter().try_fold(state, |s, g| g.apply(s)),
            Goal::Either(a, b) => state.fork(Rc::new(move |s| {
                let a = a.clone().apply(s.clone()).into_iter();
                let b = b.clone().apply(s).into_iter();
                Box::new(a.chain(b))
            })),
            Goal::Any(goals) => state.fork(Rc::new(move |s| {
                Box::new(
                    goals
                        .clone()
                        .into_iter()
                        .zip(repeat(s))
                        .flat_map(|(g, s)| g.apply(s).into_iter()),
                )
            })),
            Goal::Thunk(Thunk(func)) => func(state),
        }
    }

    pub(crate) fn thunk<F: Fn(State<'a, D>) -> Option<State<'a, D>> + 'a>(f: F) -> Goal<'a, D> {
        Goal::Thunk(Thunk(Rc::new(f)))
    }
}

impl<'a, D: Domain> fmt::Debug for Thunk<'a, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Thunk ??")
    }
}
