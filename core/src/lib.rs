#![feature(external_doc)]
#![doc(include = "../../README.md")]

pub mod domains;
pub mod goal;
pub mod query;
pub mod state;
pub mod unify;
pub mod value;

pub use goal::project::{assert_1, assert_2, map_1, map_2};
pub use goal::{both, custom, either, lazy, unify, Goal};
pub use state::{ResolvedState, State};
pub use unify::Unify;
pub use value::{var, LVar, Val};

#[doc(hidden)]
pub use canrun_codegen::domains;

pub mod util;

#[cfg(test)]
pub mod tests {
    mod test_fork;
    mod test_unify;
    mod test_watch;
}

// #[macro_use]
// extern crate log;
