use super::{State, Unify};
use crate::value::{LVar, Value};

pub trait Reify {
    type Reified;
    fn reify_in(&self, state: &State) -> Option<Self::Reified>;
}

impl<T: Unify + Reify> Reify for Value<T> {
    type Reified = T::Reified;
    fn reify_in(&self, state: &State) -> Option<Self::Reified> {
        state.resolve(self).resolved()?.reify_in(state)
    }
}

impl<T: Unify + Reify> Reify for LVar<T> {
    type Reified = T::Reified;
    fn reify_in(&self, state: &State) -> Option<Self::Reified> {
        state.resolve(&self.into()).resolved()?.reify_in(state)
    }
}

macro_rules! impl_reify_copy {
    ($($type:ty),+) => {
        $(
            impl Reify for $type {
                type Reified = $type;
                fn reify_in(&self, _: &State) -> Option<$type> {
                    Some(*self)
                }
            }
        )+
    }
}
macro_rules! impl_reify_clone {
    ($($type:ty),+) => {
        $(
            impl Reify for $type {
                type Reified = $type;
                fn reify_in(&self, _: &State) -> Option<$type> {
                    Some(self.clone())
                }
            }
        )+
    }
}

impl_reify_copy!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);
impl_reify_copy!(&'static str, bool, char);
impl_reify_clone!(String);
