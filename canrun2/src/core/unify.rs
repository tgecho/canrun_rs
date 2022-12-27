use std::any::Any;
use std::fmt::Debug;
use std::rc::Rc;

use crate::core::State;

pub trait Unify: Any + Debug {
    fn unify(state: State, a: Rc<Self>, b: Rc<Self>) -> Option<State>;
}

macro_rules! impl_unify_eq {
    ($($type:ty),+) => {
        $(
            impl Unify for $type {
                fn unify(state: State, a: Rc<$type>, b: Rc<$type>) -> Option<State> {
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

impl_unify_eq!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);
impl_unify_eq!(String, &'static str, bool, char);
impl_unify_eq!(std::ops::Range<usize>);
