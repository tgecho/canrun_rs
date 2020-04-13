use crate::{DomainType, State, UnifyIn};
use std::rc::Rc;

macro_rules! impl_unify_eq {
    ($($type:ty),+) => {
        $(
            impl <'a, D: DomainType<'a, $type>> UnifyIn<'a, D> for $type {
                fn unify_resolved(state: State<'a, D>, a: Rc<$type>, b: Rc<$type>) -> Option<State<'a, D>> {
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
