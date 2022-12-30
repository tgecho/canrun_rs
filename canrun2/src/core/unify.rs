use std::any::Any;
use std::fmt::Debug;
use std::rc::Rc;

use crate::core::State;
use crate::core::Value;

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

macro_rules! impl_unify_tuple {
    ($($t:ident => $r:ident),+) => {
        impl<$($t,)*> Unify for ($(Value<$t>),*)
        where
            $($t: Unify, )*
        {
            fn unify(
                state: State,
                l: Rc<Self>,
                r: Rc<Self>,
            ) -> Option<State> {
                #![allow(non_snake_case)]
                #![allow(clippy::needless_question_mark)]
                let ($($t),*) = l.as_ref();
                // Abusing the "reified" ident as "right" since
                // it's available. If we did this as a proc-macro
                // we could actually make up our own names.
                let ($($r),*) = r.as_ref();
                Some(state$(.unify($t, $r)?)*)
            }
        }
    };
}

impl_unify_tuple!(Av => Ar, Bv => Br);
impl_unify_tuple!(Av => Ar, Bv => Br, Cv => Cr);
impl_unify_tuple!(Av => Ar, Bv => Br, Cv => Cr, Dv => Dr);
impl_unify_tuple!(Av => Ar, Bv => Br, Cv => Cr, Dv => Dr, Ev => Er);
