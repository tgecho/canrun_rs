#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../README.md")]
//! What's next? Go read a breakdown of this example in the deeper [Quick Start
//! explanation](crate::docs::quickstart).

#[cfg(doc)]
pub mod docs;

pub mod collections;
pub mod core;
pub mod goals;
#[doc(hidden)]
pub mod util;

pub use crate::core::*;
pub use collections::*;
pub use goals::Goal;
pub use goals::{both, cmp, custom, either, lazy, ops, project, unify, Fail, Succeed};
