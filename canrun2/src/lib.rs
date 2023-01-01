#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../README.md")]

pub mod collections;
pub mod core;
pub mod goals;
#[doc(hidden)]
pub mod util;

pub use crate::core::*;
pub use collections::*;
pub use goals::{both, either, lazy, project, unify, Fail, Succeed};
