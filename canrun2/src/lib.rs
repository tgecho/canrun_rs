#![doc = include_str!("../README.md")]

pub mod collections;
pub mod core;
pub mod goals;
pub mod util;

pub use crate::core::*;
pub use collections::*;
pub use goals::{both, either, lazy, project, unify, Fail, Succeed};
