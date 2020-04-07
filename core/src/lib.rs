#![deny(unused_braces)]
//! Canrun is a [logic
//! programming](https://en.wikipedia.org/wiki/Logic_programming) library
//! inspired by the [\*Kanren](http://minikanren.org/) family of language DSLs.
//!
//! ## Status: Exploratory and Highly Experimental
//!
//! I'm still quite new to both Rust and logic programming, so there are likely
//! to be rough edges. At best the goal is to be a useful implementation of the
//! core concepts of a Kanren in way that is idiomatic to Rust. At worst it may
//! just be a poor misinterpretation with fatal flaws.
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
//! - [Values](crate::value) are either resolved or [LVars](crate::value::LVar) that
//!   can be bound to other values through unification.
//! - [Goals](crate::goal) contain declarative assertions about the
//!   relationships between values.
//! - [States](crate::state) track value bindings and constraints during
//!   evaluation of a logic program.
//! - [Queries](crate::query) allow easy extraction of resolved values.

pub mod domains;
pub mod goal;
pub mod query;
pub mod state;
mod unify;
pub mod value;

#[doc(inline)]
pub use domains::DomainType;
#[doc(inline)]
pub use goal::project::{assert_1, assert_2, map_1, map_2};
#[doc(inline)]
pub use goal::{both, custom, either, lazy, unify, Goal};
#[doc(inline)]
pub use query::{Query, Queryable};
#[doc(inline)]
pub use state::{IterResolved, ResolvedState, State};
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
