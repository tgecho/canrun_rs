use crate::{Can, CanT, State};
use std::fmt;
use std::iter::{empty, once};
use std::rc::Rc;

pub mod both;
pub mod custom;
pub mod either;
pub mod equal;
pub mod extra;
pub mod lazy;
pub mod not;

#[derive(Clone)]
pub enum Goal<T: CanT + 'static> {
    Succeed,
    Fail,
    Equal { a: Can<T>, b: Can<T> },
    Both { a: Box<Goal<T>>, b: Box<Goal<T>> },
    Either { a: Box<Goal<T>>, b: Box<Goal<T>> },
    Lazy(Rc<dyn Fn() -> Goal<T>>),
    Custom(Rc<dyn Fn(&State<T>) -> StateIter<T>>),
    Not(Box<Goal<T>>),
}

pub type StateIter<T> = Box<dyn Iterator<Item = State<T>>>;

impl<T: CanT> Goal<T> {
    pub fn run(&self, state: &State<T>) -> StateIter<T> {
        match self {
            Goal::Succeed => Box::new(once(state.clone())),
            Goal::Fail => Box::new(empty()),
            Goal::Equal { a, b } => equal::run(state, a, b),
            Goal::Both { a, b } => both::run(state, &a, &b),
            Goal::Either { a, b } => either::run(state, &a, &b),
            Goal::Lazy(func) => func().run(state),
            Goal::Custom(func) => func(state),
            Goal::Not(goal) => not::run(state, goal),
        }
    }
}

impl<T: CanT> fmt::Debug for Goal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Goal::Succeed => write!(f, "Succeed"),
            Goal::Fail => write!(f, "Fail"),
            Goal::Equal { a, b } => write!(f, "Equal {{ {:?}, {:?} }}", a, b),
            Goal::Both { a, b } => write!(f, "Both {{ {:?}, {:?} }}", a, b),
            Goal::Either { a, b } => write!(f, "Either {{ {:?}, {:?} }}", a, b),
            Goal::Lazy(func) => write!(f, "Lazy(|| => {:?})", func()),
            Goal::Custom(_) => write!(f, "Custom(?)"),
            Goal::Not(goal) => write!(f, "Not({:?})", goal),
        }
    }
}
