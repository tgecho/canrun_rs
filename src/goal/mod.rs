use crate::{Cell, State};
use itertools::Itertools;
use std::fmt;
use std::iter::{empty, once};
use std::rc::Rc;

pub mod all;
pub mod any;
pub mod append;
pub mod both;
pub mod either;
pub mod equal;
pub mod lazy;
pub mod not;

#[derive(Clone)]
pub enum Goal<T: Eq + Clone + 'static> {
    Succeed,
    Fail,
    Equal { a: Cell<T>, b: Cell<T> },
    Both { a: Box<Goal<T>>, b: Box<Goal<T>> },
    Either { a: Box<Goal<T>>, b: Box<Goal<T>> },
    Lazy(Rc<dyn Fn() -> Goal<T>>),
    Not(Box<Goal<T>>),
}

type GoalIter<T> = Box<dyn Iterator<Item = State<T>>>;

pub trait Pursue<T: Eq + Clone> {
    fn run(self, state: State<T>) -> GoalIter<T>;
}

impl<T: Eq + Clone> Goal<T> {
    pub fn run(self, state: State<T>) -> GoalIter<T> {
        match self {
            Goal::Succeed => Box::new(once(state.clone())),
            Goal::Fail => Box::new(empty()),
            Goal::Equal { a, b } => Box::new(state.unify(&a, &b).into_iter()),
            Goal::Both { a, b } => Box::new(
                (a.run(state))
                    .zip(once(b).cycle())
                    .flat_map(|(s, b)| b.run(s)),
            ),
            Goal::Either { a, b } => Box::new(a.run(state.clone()).interleave(b.run(state))),
            Goal::Lazy(func) => func().run(state),
            Goal::Not(goal) => {
                let mut iter = goal.run(state.clone());
                if iter.next().is_some() {
                    Box::new(empty())
                } else {
                    Box::new(once(state))
                }
            }
        }
    }
}

impl<T: Eq + Clone + fmt::Debug> fmt::Debug for Goal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Goal::Succeed => write!(f, "Succeed"),
            Goal::Fail => write!(f, "Fail"),
            Goal::Equal { a, b } => write!(f, "Equal {{ {:?}, {:?} }}", a, b),
            Goal::Both { a, b } => write!(f, "Both {{ {:?}, {:?} }}", a, b),
            Goal::Either { a, b } => write!(f, "Either {{ {:?}, {:?} }}", a, b),
            Goal::Lazy(lazy) => write!(f, "Lazy(|| => {:?})", lazy()),
            Goal::Not(goal) => write!(f, "Not({:?})", goal),
        }
    }
}
