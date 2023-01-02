use std::any::Any;
use std::fmt::Debug;
use std::rc::Rc;

use crate::core::State;

/**
How compatible values are matched with each other.

See
[Unification](https://en.wikipedia.org/wiki/Unification_(computer_science))
for a formal and probably more correct definition. This will attempt to
describe unification as implemented (and understood by me).

The simplest example of unification looks like equality or variable
assignment. In `x=1`, if the variable `x` is unbound, the statement succeeds
and `x` is considered equal to `1`. `1=1` is also valid, though slightly
silly. Unification does not care about direction, so `1=x` is equally valid
and has the same effect.

A follow-up assertion that `x=2` would fail, because `x` is already bound to
`1`.

Unifying structures containing other types of values can get interesting
very fast. Unifying a free (unbound) variable with a structure simply binds
that variable to the entire structure (e.g. `x=(1,2)`). However, binding two
compatible structures with each other allows binding to values inside the
structures. In `(x,2)=(1,2)`, the `x` in the first structure is bound to the
`1` in the second. Structurally incompatible values will fail immediately: `(x,2,3)=(1,2)`.

Arbitrarily nested structures can be unified by recursively applying this
simple pattern matching.

For simple types, unification is essentially the same thing as equality (and
implementations are provided for most simple primitive types). The general pattern
for structures is to define a way to match up their component parts and
recursively attempt to unify them.

# Implementation

Default implementations are provided for most primitive types, and a few general
"logic collections". You can also implement it for your own types.

TODO: Create a derive macro
```
use canrun2::core::{State, Unify, Value};
use std::rc::Rc;

#[derive(Debug)]
struct MyType<T: Unify> {
    inside: Value<T>
}

impl<T: Unify> Unify for MyType<T> {
    fn unify(state: State, a: Rc<Self>, b: Rc<Self> ) -> Option<State> {
        state.unify(&a.inside, &b.inside)
    }
}
# fn main() {}
```
*/
pub trait Unify: Any + Debug {
    /**
    Attempt to unify two fully resolved values.

    This function accepts `Rc<T>`s to simplify the borrow checking. The
    `Option<_>` return type allows recursive unification of structures that
    hold additional values.
    */
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
