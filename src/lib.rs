pub mod can;
pub mod goal;
pub mod state;

pub use can::lvar::{LVar, var};
pub use can::pair::pair;
pub use can::{Can, CanT};
pub use state::{ResolveResult, State, UnifyError, UnifyResult};

// Goals
pub use goal::all::all;
pub use goal::any::any;
pub use goal::both::both;
pub use goal::either::either;
pub use goal::equal::{equal, Equals};
pub use goal::lazy::{lazy, with1, with2, with3};
pub use goal::not::not;
pub use goal::Goal;
pub use goal::StateIter;

#[macro_use]
extern crate log;
