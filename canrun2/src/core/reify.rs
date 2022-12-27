use super::{State, Unify};
use crate::value::Value;

pub trait Reify {
    type Concrete;
    fn reify_in(&self, state: &State) -> Option<Self::Concrete>;
}

impl<T: Unify + Reify> Reify for Value<T> {
    type Concrete = T::Concrete;
    fn reify_in(&self, state: &State) -> Option<Self::Concrete> {
        state.resolve(self).resolved()?.reify_in(state)
    }
}

macro_rules! impl_reify_copy {
    ($($type:ty),+) => {
        $(
            impl Reify for $type {
                type Concrete = $type;
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
                type Concrete = $type;
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
