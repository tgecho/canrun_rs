#![warn(missing_docs)]

//! [Canrun](canrun) related [goal](canrun) functions and other miscellany related to [ops](std::ops).

extern crate canrun;

mod cmp;
mod ops;

pub use cmp::*;
pub use ops::*;
