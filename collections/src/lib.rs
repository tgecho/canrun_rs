#![warn(missing_docs)]

//! [Canrun](canrun) collections and related [goal](canrun) functions.
//!
//! # NOTE: These are not very battle tested and may have some pathological performance characteristics.
//!
//! Unifying large or complex collections may involve forking the state for
//! every possible combination of values. Also, the inherent complexity of
//! specifying and implementing these operations correctly means that they could
//! be flat out wrong. More testing, benchmarking and refinement is required.

extern crate canrun;

pub mod example;
pub mod lmap;
pub mod lvec;

#[doc(hidden)]
pub use lmap::LMap;
#[doc(hidden)]
pub use lvec::LVec;
