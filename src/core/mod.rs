pub mod domain;
pub mod goal;
pub mod lvar;
pub mod query;
pub mod state;
mod util;
pub mod value;

#[cfg(test)]
mod tests {
    mod test_fork;
    mod test_unify;
    mod test_watch;
    pub(crate) mod util;
}
