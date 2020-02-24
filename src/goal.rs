use crate::state::empty_iter;
use crate::{Can, CanT, State};
use std::fmt;
use std::rc::Rc;

pub mod both;
pub mod constrain;
pub mod custom;
pub mod either;
pub mod equal;
pub mod extra;
pub mod lazy;
pub mod map;
pub mod not;

#[derive(Clone)]
pub enum Goal<'a, T: CanT> {
    Succeed,
    Fail,
    Equal {
        a: Can<T>,
        b: Can<T>,
    },
    Both {
        a: Box<Goal<'a, T>>,
        b: Box<Goal<'a, T>>,
    },
    Either {
        a: Box<Goal<'a, T>>,
        b: Box<Goal<'a, T>>,
    },
    Lazy(Rc<dyn Fn() -> Goal<'a, T> + 'a>),
    Custom(Rc<dyn Fn(State<'a, T>) -> StateIter<'a, T> + 'a>),
    Not(Box<Goal<'a, T>>),
}

pub type StateIter<'a, T> = Box<dyn Iterator<Item = State<'a, T>> + 'a>;

impl<'a, T: CanT + 'a> Goal<'a, T> {
    pub fn run(self, state: State<'a, T>) -> StateIter<'a, T> {
        match self {
            Goal::Succeed => state.to_iter(),
            Goal::Fail => empty_iter(),
            Goal::Equal { a, b } => equal::run(state, a, b),
            Goal::Both { a, b } => both::run(state, *a, *b),
            Goal::Either { a, b } => either::run(state, *a, *b),
            Goal::Lazy(func) => func().run(state),
            Goal::Custom(func) => func(state),
            Goal::Not(goal) => not::run(state, *goal),
        }
    }
}

impl<'a, T: CanT> fmt::Debug for Goal<'a, T> {
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
