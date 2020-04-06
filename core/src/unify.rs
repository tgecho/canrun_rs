use crate::domains::DomainType;
use crate::state::State;
use std::rc::Rc;

mod tuples;
mod vec;

pub trait Unify<'a, T>: DomainType<'a, T> {
    fn unify_resolved(state: State<'a, Self>, a: Rc<T>, b: Rc<T>) -> Option<State<'a, Self>>;
}

macro_rules! impl_unify_eq {
    ($($type:ty),+) => {
        $(
            impl<'a, D: DomainType<'a, $type>> Unify<'a, $type> for D {
                fn unify_resolved(state: State<'a, Self>, a: Rc<$type>, b: Rc<$type>) -> Option<State<'a, Self>> {
                    if a == b {
                        Some(state)
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
