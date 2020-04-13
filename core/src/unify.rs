use crate::domains::DomainType;
use crate::state::State;
use std::fmt::Debug;
use std::rc::Rc;

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
/// collections. You can also implement it for your own types.
///
/// TODO: Create a derive macro
/// ```
/// use canrun::{State, DomainType, UnifyIn};
/// use std::rc::Rc;
///
/// #[derive(PartialEq, Debug)]
/// struct MyType;
///
/// impl<'a, D> UnifyIn<'a, D> for MyType
/// where
///     // The domain must be constrained to
///     // those that contain yur type
///     D: DomainType<'a, Self>
/// {
///     fn unify_resolved(
///         state: State<'a, D>,
///         a: Rc<Self>,
///         b: Rc<Self>
///     ) -> Option<State<'a, D>> {
///         if a == b { Some(state) } else { None }
///     }
/// }
/// # fn main() {}
/// ```
/// Because the trait is parameterized with a [domain](crate::domains), you
/// should be able to implement UnifyIn for third-party types without running into
/// the orphan trait rule, so long as you don't conflict with an existing
/// implementation.
/// ```
/// # // Just a random foreign type to make this an accurate doctest.
/// # // I'm pretty sure no one will want to unify this for real :)
/// # use std::convert::Infallible as SomeForeignType;
/// use canrun::{State, DomainType, UnifyIn};
/// use std::rc::Rc;
///
/// canrun::domain! {
///     MyDomain {
///         SomeForeignType
///     }
/// }
///
/// impl<'a> UnifyIn<'a, MyDomain> for SomeForeignType {
///     // ...
/// #    fn unify_resolved(
/// #        state: State<'a, MyDomain>,
/// #        a: Rc<Self>,
/// #        b: Rc<Self>
/// #    ) -> Option<State<'a, MyDomain>> {
/// #        if a == b { Some(state) } else { None }
/// #    }
/// }
/// # fn main() {}
/// ```
pub trait UnifyIn<'a, D: DomainType<'a, Self>>: Sized + Debug {
    /// Attempt to unify two fully resolved values.
    ///
    /// This function accepts `Rc<T>`s to simplify the borrow checking. The
    /// `Option<_>` allows recursive unification of structures that hold
    /// additional values.
    fn unify_resolved(state: State<'a, D>, a: Rc<Self>, b: Rc<Self>) -> Option<State<'a, D>>;
}
