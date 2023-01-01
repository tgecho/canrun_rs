use crate::core::State;
use crate::goals::Goal;

/** A [`Goal`] that always succeeds.

# Example
```
use canrun2::{Succeed, unify, Value, all, Query};

let x = Value::var();
let goal = all![unify(x.clone(), 1), Succeed];
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![1])
```
*/
#[derive(Debug)]
pub struct Succeed;

impl Goal for Succeed {
    fn apply(&self, state: State) -> Option<State> {
        Some(state)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn succeed() {
        let state = State::new();
        let goal = Succeed;
        let result = goal.apply(state);
        assert!(result.is_some());
    }
}
