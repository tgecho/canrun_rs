#![warn(missing_docs)]

//! [Canrun](canrun) collection related [goal](canrun) functions and other miscellany.

extern crate canrun;

pub mod example;
pub mod lmap;
pub mod lvec;

#[doc(hidden)]
pub use lmap::LMap;
#[doc(hidden)]
pub use lvec::LVec;
