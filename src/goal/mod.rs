use crate::{Cell, State};
use itertools::Itertools;
use std::iter::{empty, once};
use std::rc::Rc;

pub mod append;
pub mod both;
pub mod either;
pub mod equal;
pub mod lazy;

pub use both::both;
pub use either::either;
pub use equal::equal;
pub use lazy::{lazy, with1, with2, with3};

#[derive(Clone)]
pub enum Goal<T: Eq + Clone + 'static> {
    Equal { a: Cell<T>, b: Cell<T> },
    Both { a: Box<Goal<T>>, b: Box<Goal<T>> },
    Either { a: Box<Goal<T>>, b: Box<Goal<T>> },
    Lazy(Rc<dyn Fn() -> Goal<T>>),
}

type GoalIter<T> = Box<dyn Iterator<Item = State<T>>>;

pub trait Pursue<T: Eq + Clone> {
    fn run<'a>(self, state: &'a State<T>) -> GoalIter<T>;
}

impl<T: Eq + Clone + 'static> Goal<T> {
    pub fn run<'a>(self, state: &'a State<T>) -> GoalIter<T> {
        match self {
            Goal::Equal { a, b } => Box::new(state.unify(&a, &b).into_iter()),
            Goal::Both { a, b } => Box::new(
                (a.run(&state))
                    .zip(once(b).cycle())
                    .flat_map(|(s, b)| b.run(&s)),
            ),
            Goal::Either { a, b } => Box::new(a.run(&state).interleave(b.run(&state))),
            Goal::Lazy(func) => func().run(state),
        }
    }

    pub fn run_in_each(self, mut states: GoalIter<T>) -> GoalIter<T> {
        let first = states.next();
        match first {
            Some(state) => {
                let head = self.clone().run(&state);
                let tail = self.clone().run_in_each(states);
                Box::new(head.interleave(tail))
            }
            None => Box::new(empty()),
        }
    }
}
