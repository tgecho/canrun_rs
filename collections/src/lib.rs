#![warn(missing_docs)]

//! [Canrun](canrun) collection related [goal](canrun) functions and other miscellany.

extern crate canrun;

pub mod example;
pub mod lmap;
mod macros;
mod member;

pub use lmap::LMap;
pub use member::member;
