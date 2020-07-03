//! Constrain the set of types that you can reason about in a particular
//! context.

use crate::state::State;
use crate::value::{LVar, Val};
use im_rc::HashMap;
use std::fmt::Debug;

/// Manage values for a set of specific types.
///
/// Works with the [`DomainType<T>`](DomainType) trait to allow access to actual
/// values.
///
/// Domains are typically generated with the [`domain!`](./macro.domain.html)
/// macro. There isn't currently much use case for interacting with a domain
/// directly in user code. The only reason it is public is to allow implementing
/// custom domains through the macro.
///
/// ```
/// use canrun::{State, Goal, unify, var};
///
/// canrun::domain! {
///     MyDomain {
///         i32
///     }
/// }
///
/// # fn main() -> () {
/// let x = var();
/// let state: State<MyDomain> = State::new();
/// let goal: Goal<MyDomain> = unify(x, 1);
/// # }
/// ```
pub trait Domain<'a>: Clone + Debug {
    /// An individual value that may contain any of the valid types in this
    /// domain.
    ///
    /// Typically for internal use.
    type Value: Debug + Clone + 'a;

    /// Create a new, valid domain.
    ///
    /// Typically for internal use.
    fn new() -> Self;

    /// Apply [`.unify()`](crate::state::State::unify()) to two [domain level
    /// values](crate::domains::Domain::Value).
    ///
    /// The unification will fail if the inner values are not of the same type.
    /// This should not be able to happen.
    ///
    /// Typically for internal use.
    fn unify_domain_values(
        state: State<'a, Self>,
        a: Self::Value,
        b: Self::Value,
    ) -> Option<State<'a, Self>>;
}

/// A type specific container used by a [`Domain`](crate::domains::Domain) to
/// hold values.
///
/// Created by the `domain!` macro and intended for internal use.
#[derive(Debug)]
pub struct DomainValues<T: Debug>(pub(crate) HashMap<LVar<T>, Val<T>>);

impl<T: Debug> DomainValues<T> {
    #[doc(hidden)]
    pub fn new() -> Self {
        DomainValues(HashMap::new())
    }
}
impl<'a, T: Debug> Clone for DomainValues<T> {
    fn clone(&self) -> Self {
        DomainValues(self.0.clone())
    }
}

/// Allows a [`State`](crate::state) to retrieve values of a specific type from
/// a [domain](crate::domains).
///
/// This trait is automatically implemented by the `domain!` macro.
///
/// As of now there shouldn't be much of a need to use this trait's
/// functionality in user facing code. The trait itself may need to be used as a
/// constraint, though [`UnifyIn`](crate::UnifyIn) is often the better, higher
/// level choice.
pub trait DomainType<'a, T: Debug>: Domain<'a> {
    #[doc(hidden)]
    fn resolve<'r>(&'r self, val: &'r Val<T>) -> &'r Val<T>
    where
        T: Debug,
    {
        match val {
            Val::Var(var) => {
                let resolved = self.values_as_ref().0.get(var);
                match resolved {
                    Some(Val::Var(found)) if found == var => val,
                    Some(found) => self.resolve(found),
                    _ => val,
                }
            }
            value => value,
        }
    }

    #[doc(hidden)]
    fn update(&mut self, key: LVar<T>, value: Val<T>) {
        self.values_as_mut().0.insert(key, value);
    }

    #[doc(hidden)]
    fn values_as_ref(&self) -> &DomainValues<T>;
    #[doc(hidden)]
    fn values_as_mut(&mut self) -> &mut DomainValues<T>;
    #[doc(hidden)]
    fn into_domain_val(val: Val<T>) -> Self::Value;
}

/// Generate [Domain] structs and other associated types and impls.
///
/// Manually implementing a [Domain] would be tedious and finicky. This macro
/// attempts to simplify most of the general cases by building out everything
/// required to reason about values of various types.
///
/// A few [example domains](crate::example) are available for simple
/// use cases.
///
/// # Examples
/// Begin each declaration with the domain keyword.
/// ```
/// canrun::domain! {
///     MyDomain { i32 }
/// }
/// # fn main() -> () {}
/// # // keep this `-> ()` to quell `needless_doctest_main` warning
/// # // https://github.com/rust-lang/rust-clippy/issues/4698
/// ```
///
/// The optional [visibility
/// modifier](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
/// works just as in normal Rust.
/// ```
/// canrun::domain! {
///     pub MyPublicDomain { i32 }
/// }
/// # fn main() {}
/// ```
///
/// You can define multiple values, include structures. Note that a wrapping
/// [`Val`](crate::value::Val) is required in the tuple.
/// ```
/// use canrun::Val;
/// canrun::domain! {
///     pub MyBigDomain {
///         i32,
///         String,
///         (Val<i32>, Val<String>),
///     }
/// }
/// # fn main() {}
/// ```
///
/// Any types you add to a domain must implement the
/// [`UnifyIn`](crate::unify::UnifyIn) trait. Canrun includes default
/// implementations for almost all primitive types and collection types are
/// available in [`canrun_collections`].
///
/// Once you have a domain, you can use it to parameterize other types, such as
/// [`State`](crate::state::State) and [`Goal`](crate::goals::Goal):
/// ```
/// # use canrun::state::State;
/// # use canrun::goals::{Goal, unify};
/// # use canrun::value::var;
/// # canrun::domain! { MyDomain { i32 } }
/// # fn main() {
/// # let x = var();
/// let state: State<MyDomain> = State::new();
/// let goal: Goal<MyDomain> = unify(x, 1);
/// # }
/// ```
pub use canrun_codegen::domain;
