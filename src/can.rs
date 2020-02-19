// pub mod hoc;
pub mod lvar;
pub mod pair;
pub mod vec;

// use crate::can::hoc::Hoc;
use lvar::LVar;
use std::fmt;

pub trait CanT: PartialEq + Clone + fmt::Debug {}
impl<T: PartialEq + Clone + fmt::Debug> CanT for T {}

#[derive(Clone)]
pub enum Can<T: CanT> {
    Nil,
    Var(LVar),
    Val(T),
    Pair { l: Box<Can<T>>, r: Box<Can<T>> },
    Vec(Vec<Can<T>>),
    // Hoc(Hoc<T>),
}

impl<T: CanT> From<T> for Can<T> {
    fn from(t: T) -> Self {
        Can::Val(t)
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

impl<T: CanT> fmt::Debug for Can<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Can::Nil => write!(f, "Nil"),
            Can::Var(v) => write!(f, "Var({:?})", v),
            Can::Val(v) => write!(f, "Val({:?})", v),
            Can::Pair { l, r } => write!(f, "Pair{{ {:?}, {:?} }}", l, r),
            Can::Vec(v) => write!(f, "Vec({:?})", v),
            // Can::Hoc(_) => write!(f, "Hoc",),
        }
    }
}
