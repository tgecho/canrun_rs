use super::{State, StateIter};

/** Fork a [`State`] into zero or more alternate states.

Added to a [`State`] with [`.fork()`](State::fork()).

# Example:
```
use canrun2::{Fork, Query, State, StateIter, Value};
use std::rc::Rc;

struct Is1or2 {
    x: Value<i32>,
}

impl Fork for Is1or2 {
    fn fork(&self, state: &State) -> StateIter {
        let s1 = state.clone().unify(&self.x, &Value::new(1));
        let s2 = state.clone().unify(&self.x, &Value::new(2));
        Box::new(s1.into_iter().chain(s2.into_iter()))
    }
}

# fn main() {
let x = Value::var();
let state = State::new();
let state = state.fork(Is1or2 { x: x.clone() });
let results: Vec<_> = state.query(x).collect();
assert_eq!(results, vec![1, 2]);
# }
```
*/
pub trait Fork: 'static {
    /// Given a [`State`], return an iterator of states that result from the
    /// fork operation.
    fn fork(&self, state: &State) -> StateIter;
}

impl<F: 'static> Fork for F
where
    F: Fn(&State) -> StateIter,
{
    fn fork(&self, state: &State) -> StateIter {
        self(state)
    }
}
