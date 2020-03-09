pub mod domain;
pub mod goal;
pub mod query;
pub mod state;
pub mod val;

#[cfg(test)]
mod tests {
    mod test_fork;
    mod test_unify;
    mod test_watch;
    mod utils;
}
