#![warn(missing_docs)]
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
//! use canrun::example::I32;
//!
//! let x = var();
//! let y = var();
//! let goal: Goal<I32> = both(unify(x, y), unify(1, x));
//! let result: Vec<_> = goal.query(y).collect();
//! assert_eq!(result, vec![1])
//! ```

pub mod collections;
pub mod domains;
pub mod example;
pub mod goals;
mod impls;
mod query;
mod reify;
pub mod state;
mod unify;
pub mod util;
pub mod value;

pub use collections::*;
pub use domains::*;
pub use goals::*;
pub use impls::tuples::*;
pub use query::*;
pub use reify::*;
pub use state::*;
pub use unify::*;
pub use value::*;

#[cfg(test)]
mod tests {
    mod test_constrain;
    mod test_fork;
    mod test_unify;
}
