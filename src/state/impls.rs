use super::{Domain, State};
use std::fmt;

impl<'a, D: Domain<'a> + 'a> fmt::Debug for State<'a, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "State ??")
    }
}
