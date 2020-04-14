#![warn(missing_docs)]

//! [Canrun](canrun) collection related [goal](canrun) functions and other miscellany.

extern crate canrun;

/// Example domains for canrun collections
pub mod example;
mod lmap;
mod macros;
mod member;

pub use lmap::LMap;
pub use member::member;
