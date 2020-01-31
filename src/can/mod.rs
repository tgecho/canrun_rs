pub mod lvar;

use crate::goal::GoalIter;
use crate::State;
use lvar::LVar;
use std::fmt;
use std::rc::Rc;

pub trait CanT: Eq + Clone + fmt::Debug {}
impl<T: Eq + Clone + fmt::Debug> CanT for T {}

#[derive(Clone)]
pub enum Can<T: CanT> {
    Nil,
    Var(LVar),
    Val(T),
    Pair {
        l: Box<Can<T>>,
        r: Box<Can<T>>,
    },
    Vec(Vec<Can<T>>),
    Funky {
        v: Box<Can<T>>,
        f: Rc<dyn Fn(Can<T>, Can<T>, State<T>) -> GoalIter<T>>,
    },
}

impl<T: CanT> From<LVar> for Can<T> {
    fn from(lvar: LVar) -> Self {
        Can::Var(lvar)
    }
}

impl<T: CanT> PartialEq for Can<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Can::Nil, Can::Nil) => true,
            (Can::Var(s), Can::Var(o)) => s == o,
            (Can::Val(s), Can::Val(o)) => s == o,
            (Can::Pair { l: sl, r: sr }, Can::Pair { l: ol, r: or }) => sl == ol && sr == or,
            (Can::Vec(s), Can::Vec(o)) => s == o,
            _ => false,
        }
    }
}
// impl<T: CanT> Eq for Can<T> {}

impl<T: CanT> fmt::Debug for Can<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Can::Nil => write!(f, "Nil"),
            Can::Var(v) => write!(f, "Var({:?})", v),
            Can::Val(v) => write!(f, "Val({:?})", v),
            Can::Pair { l, r } => write!(f, "Pair{{ {:?}, {:?} }}", l, r),
            Can::Vec(v) => write!(f, "Vec({:?})", v),
            Can::Funky { v, .. } => write!(f, "Funky{{ {:?} + ?}}", v),
        }
    }
}
