use crate::domains::DomainType;
use crate::state::State;
use std::rc::Rc;

mod tuples;
mod vec;

pub trait Unify<'a, T>: Sized {
    fn unify_resolved(self, a: Rc<T>, b: Rc<T>) -> Option<Self>;
}

macro_rules! impl_unify_eq {
    ($($type:ty),+) => {
        $(
            impl<'a, D: DomainType<'a, $type>> Unify<'a, $type> for State<'a, D> {
                fn unify_resolved(self, a: Rc<$type>, b: Rc<$type>) -> Option<Self> {
                    if a == b {
                        Some(self)
                    } else {
                        None
                    }
                }
            }
        )+
    };
}

impl_unify_eq!(bool, char, String);
impl_unify_eq!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);
