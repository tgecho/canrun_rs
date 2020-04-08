//! Constrain the set of types that you can reason about in a particular context.

use crate::state::State;
use crate::value::{LVar, Val};
use im_rc::HashMap;
use std::fmt::Debug;

pub mod example {
    //! Basic domains for simple use cases.
    //!
    //! | Domain   | Types |
    //! | ------   | ----- |
    //! | I32      | i32 |
    //! | VecI32   | i32, Vec<Val<i32>> |
    //! | TupleI32 | i32, (Val<i32>, Val<i32>) |

    // Figure out how to get the macro to generate docs with these types listed out.

    use crate::value::Val;

    canrun_codegen::canrun_internal_domains! {
        pub domain I32 { i32 }
        pub domain VecI32 {
            i32,
            Vec<Val<i32>>,
        }
        pub domain TupleI32 {
            i32,
            (Val<i32>, Val<i32>),
        }
    }
}

/// Manage values for a set of specific types.
///
/// Works with the [`DomainType<T>`](DomainType) trait to allow access to actual values.
///
/// Domains are typically generated with the [domains!](./macro.domains.html)
/// macro. There isn't currently much use case for interacting with a domain
/// directly in user code. The only reason it is public is to allow implementing
/// custom domains through the macro.
///
/// ```
/// use canrun::{domains, State, Goal, unify, var};
///
/// domains! {
///     domain MyDomain {
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
    /// An individual value that may contain any of the valid types in this domain.
    ///
    /// Typically for internal use.
    type Value: Debug + Clone + 'a;

    /// Create a new, valid domain.
    ///
    /// Typically for internal use.
    fn new() -> Self;

    /// Apply [.unify()](crate::state::State::unify()) to two [domain level
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

#[derive(Debug)]
pub struct DomainValues<T>(pub(crate) HashMap<LVar<T>, Val<T>>);

impl<T> DomainValues<T> {
    #[doc(hidden)]
    pub fn new() -> Self {
        DomainValues(HashMap::new())
    }
}
impl<'a, T> Clone for DomainValues<T> {
    fn clone(&self) -> Self {
        DomainValues(self.0.clone())
    }
}

/// Allows a [state](crate::state) to retrieve values of a specific type from a
/// [domain](crate::domains).
///
/// This trait is automatically implemented by the `domains!` macro.
///
/// As of now there shouldn't be much of a need to use this trait's
/// functionality in user facing code. It may need to be used as a constraint,
/// though [`Unify`](crate::Unify) is often the better, higher level choice.
pub trait DomainType<'a, T>: Domain<'a> {
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
/// A few [example domains](crate::domains::example) are available for simple use cases.
///
/// # Examples
/// Begin each declaration with the domain keyword.
/// ```
/// use canrun::domains;
/// domains! {
///     domain MyDomain { i32 }
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
/// # use canrun::domains;
/// domains! {
///     pub domain MyPublicDomain { i32 }
/// }
/// # fn main() {}
/// ```
///
/// You can define multiple values, include containers. Note that a wrapping
/// [Val](crate::value::Val) is required in the Vec type.
/// ```
/// # use canrun::domains;
/// use canrun::Val;
/// domains! {
///     pub domain MyBigDomain {
///         i32,
///         String,
///         Vec<Val<i32>>,
///     }
/// }
/// # fn main() {}
/// ```
///
/// Any types you add to a domain must implement the
/// [Unify](crate::unify::Unify) trait. Canrun includes default implementations
/// for almost all primitive types and `Vec<Val<T: Unify>>`.
///
/// Once you have a domain, you can use it to parameterize other types, such as
/// [State](crate::state::State) and [Goal](crate::goal::Goal):
/// ```
/// # use canrun::domains;
/// # use canrun::state::State;
/// # use canrun::goal::{Goal, unify};
/// # use canrun::value::var;
/// # domains! { domain MyDomain { i32 } }
/// # fn main() {
/// # let x = var();
/// let state: State<MyDomain> = State::new();
/// let goal: Goal<MyDomain> = unify(x, 1);
/// # }
/// ```
#[doc(inline)]
pub use canrun_codegen::domains;
