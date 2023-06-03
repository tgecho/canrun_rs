//! [`Goal`](crate::goals)s that deal with resolved values.
//!
//! Not all relationships can be expressed with the simpler low level
//! operations, especially when involve values of different types.
//!
//! The project family of goals use
//! [`State.constrain()`](crate::State::constrain()) to allow dealing with
//! resolved values. These goals are relatively low level and may be a bit
//! subtle to use correctly. They are provided as a foundation for
//! building higher level goals.

mod assert_1;
mod assert_2;
mod map_1;
mod map_2;
mod project_1;
mod project_2;
mod reified;

pub use assert_1::*;
pub use assert_2::*;
pub use map_1::*;
pub use map_2::*;
pub use project_1::*;
pub use project_2::*;
pub use reified::*;
