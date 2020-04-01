#![feature(specialization)]

pub mod domain;
pub mod goal;
pub mod query;
pub mod state;
pub mod value;

pub use canrun_codegen::domains;

pub mod util;

#[cfg(test)]
pub mod tests {
    pub mod domains;
    mod test_fork;
    mod test_unify;
    mod test_watch;
}

// #[macro_use]
// extern crate log;
