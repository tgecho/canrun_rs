#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::module_name_repetitions)]
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
pub use goals::{both, cmp, custom, either, lazy, not, ops, project, unify, Fail, Succeed};
