#![feature(specialization)]
#![feature(trace_macros)]

pub mod domain;
pub mod goal;
pub mod query;
pub mod state;
pub mod value;

mod util;

#[cfg(test)]
pub(crate) mod tests {
    pub mod test_fork;
    pub mod test_unify;
    pub mod test_watch;
    pub mod util;
}

// #[macro_use]
// extern crate log;
