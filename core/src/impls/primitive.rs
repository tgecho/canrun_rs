use crate::{Domain, DomainType, ReifyIn, ResolvedState, State, UnifyIn};
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

macro_rules! impl_reify_copy {
    ($($type:ty),+) => {
        $(
            impl<'a, D: Domain<'a>> ReifyIn<'a, D> for $type {
                type Reified = $type;
                fn reify_in(&self, _: &ResolvedState<D>) -> Option<$type> {
                    Some(*self)
                }
            }
        )+
    }
}

macro_rules! impl_reify_clone {
    ($($type:ty),+) => {
        $(
            impl<'a, D: Domain<'a>> ReifyIn<'a, D> for $type {
                type Reified = $type;
                fn reify_in(&self, _: &ResolvedState<D>) -> Option<$type> {
                    Some(self.clone())
                }
            }
        )+
    }
}

impl_unify_eq!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);
impl_unify_eq!(String, &'static str, bool, char);

impl_reify_copy!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);
impl_reify_clone!(String);
impl_reify_copy!(&'static str, bool, char);
