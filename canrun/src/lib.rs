#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
//! What's next? Go read a breakdown of this example in the deeper [Quick Start
//! explanation](crate::docs::quickstart).

#[cfg(doc)]
pub mod docs;

pub mod collections;
pub mod domains;
pub mod example;
pub mod goals;
mod impls;
mod query;
mod reify;
pub mod state;
mod unify;
pub mod util;
pub mod value;

pub use collections::*;
pub use domains::*;
pub use goals::*;
pub use impls::tuples::*;
pub use query::*;
pub use reify::*;
pub use state::*;
pub use unify::*;
pub use value::*;

#[cfg(test)]
mod tests {
    mod test_constrain;
    mod test_fork;
    mod test_unify;
}
