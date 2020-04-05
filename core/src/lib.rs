//! Canrun is a [logic programming](https://en.wikipedia.org/wiki/Logic_programming)
//! library inspired by the [\*Kanren](http://minikanren.org/) family of language DSLs.
//!
//! ## Status: Exploratory and Highly Experimental
//!
//! I'm still quite new to both Rust and logic programming, so there are likely
//! to be rough edges. At best the goal is to be a useful implementation of the
//! core concepts of a Kanren in way that is idiomatic to Rust. At worst it may
//! just be a poor misinterpretation with fatel flaws.
//!
//! ## Quick Start
//!
//! ```rust
//! use canrun::{Goal, both, unify, var};
//! use canrun::domains::example::I32;
//!
//! let x = var();
//! let y = var();
//! let goal: Goal<I32> = both(unify(x, y), unify(1, x));
//! let result: Vec<_> = goal.query(y).collect();
//! assert_eq!(result, vec![1])
//! ```
//!
//! ## Concepts
//!
//! - [Domains](crate::domains) constrain the set of types that you can reason
//!   about in a particular context.
//! - [LVars](crate::value::LVar) are bound to other [values](crate::value)
//!   through unification.
//! - [Goals](crate::goal) contain declarative assertions about the
//!   relationships between values.
//! - [States](crate::state) track the process of unifying values and allow
//!   [querying](crate::query) for results.

pub mod domains;
pub mod goal;
pub mod query;
pub mod state;
pub mod unify;
pub mod value;

#[doc(inline)]
pub use goal::project::{assert_1, assert_2, map_1, map_2};
#[doc(inline)]
pub use goal::{both, custom, either, lazy, unify, Goal};
#[doc(inline)]
pub use state::{ResolvedState, State};
#[doc(inline)]
pub use unify::Unify;
#[doc(inline)]
pub use value::{var, LVar, Val};

#[doc(hidden)]
pub use canrun_codegen::domains;

pub mod util;

#[cfg(test)]
mod tests {
    mod test_fork;
    mod test_unify;
    mod test_watch;
}

// #[macro_use]
// extern crate log;
