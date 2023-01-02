/*! A helper macro and some blanket implementations to facilitate working with tuples of [`Value`]s. */

use crate::core::{Reify, State, Unify, Value};
use std::rc::Rc;

/** Create a tuple of [logical values](crate::Value) with automatic `Into<Value<T>>`
wrapping.

The primary benefit is that it allows freely mixing resolved values and
[`LVar`s](crate::LVar).

# Example:
```
use canrun::{LVar, ltup, Value};
let x = LVar::new();
let tuple: (Value<i32>, Value<i32>, Value<&'static str>) = ltup!(x, 1, "two");
```
*/
#[macro_export]
macro_rules! ltup {
    ($($item:expr),* $(,)?) => {
        ($($crate::core::Value::from($item)),*)
    };
}

#[doc(inline)]
pub use ltup;

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

macro_rules! impl_reify_tuple {
    ($($t:ident => $r:ident),+) => {

        impl<$($t: Reify< Reified = $r>, $r,)*> Reify for ($($t),*) {
            type Reified = ($($t::Reified),*);
            fn reify_in(&self, state: &State) -> Option<Self::Reified> {
                #![allow(non_snake_case)]
                let ($($t),*) = self;
                Some(($($t.reify_in(state)?),*))
            }
        }
    };
}

impl_reify_tuple!(Av => Ar, Bv => Br);
impl_reify_tuple!(Av => Ar, Bv => Br, Cv => Cr);
impl_reify_tuple!(Av => Ar, Bv => Br, Cv => Cr, Dv => Dr);
impl_reify_tuple!(Av => Ar, Bv => Br, Cv => Cr, Dv => Dr, Ev => Er);
