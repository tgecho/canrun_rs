use crate::domains::DomainType;
use crate::state::State;
use std::rc::Rc;

mod tuples;
mod vec;

/// How compatible values are matched with each other.
///
/// See
/// [Unification](https://en.wikipedia.org/wiki/Unification_(computer_science))
/// for a formal and probably more correct definition. This will attempt to
/// describe unification as implemented (and understood by the author).
///
/// The simplest example of unification looks like equality or variable
/// assignment. In `x=1`, if the variable `x` is unbound, the statement succeeds
/// and `x` is considered equal to `1`. `1=1` is also valid, though slightly
/// silly. Unification does not care about direction, so `1=x` is equally valid
/// and has the same effect.
///
/// A follow-up assertion that `x=2` would fail, because `x` is already bound to
/// `1`.
///
/// Unifying structures containing other types of values can get interesting
/// very fast. Unifying a free (unbound) variable with a structure simply binds
/// that variable to the entire structure (e.g. `x=(1,2)`). However, binding two
/// compatible structures with each other allows binding to values inside the
/// structures. In `(x,2)=(1,2)`, the `x` in the first structure is bound to the
/// `1` in the second.
///
/// Arbitrarily nested structures can be unified by recursively applying this
/// simple pattern matching.
///
/// For simple types, unification is essentially the same thing as equality (and
/// implementations are provided for these simplest cases). The general pattern
/// for structures is to define a way to match up their component parts and
/// recursively attempt to unify them.
///
/// # Implementation
///
/// Default implementations are provided for most primitive types and some
/// collections. You can implement it for your own types.
/// ```
/// use canrun::{domains, State, DomainType, Unify};
/// use std::rc::Rc;
///
/// domains! {
///     domain MyDomain { MyType }
/// }
///
/// #[derive(PartialEq, Debug)]
/// struct MyType;
///
/// impl<'a> Unify<'a, MyType> for MyDomain {
///     fn unify_resolved(
///         state: State<'a, MyDomain>,
///         a: Rc<MyType>,
///         b: Rc<MyType>
///     ) -> Option<State<'a, MyDomain>> {
///         if a == b { Some(state) } else { None }
///     }
/// }
///
/// # fn main() {}
/// ```
///
/// Because the trait is implemented for a [domain](crate::domains), which are
/// typically generated through the [domains!](crate::domains#macro) macro, you
/// should be able to implement Unify for outside types, so long as you don't
/// conflict with an existing implementation.
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
