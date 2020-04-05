pub mod domains;
pub mod goal;
pub mod query;
pub mod state;
pub mod unify;
pub mod value;

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
