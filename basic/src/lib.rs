#![warn(missing_docs)]

//! [Canrun](canrun) related [goal](canrun) functions and other miscellany
//! related to [ops](std::ops).

extern crate canrun;

pub mod cmp;
pub mod ops;

#[doc(hidden)]
pub use cmp::*;
#[doc(hidden)]
pub use ops::*;
