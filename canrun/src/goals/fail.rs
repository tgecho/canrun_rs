use crate::core::State;
use crate::goals::Goal;

/** A [`Goal`] that always fails.

# Example
```
use canrun::{Fail, all, unify, LVar, Query};

let x = LVar::new();
let goal = all![unify(x, 1), Fail];
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![])
```
*/
#[derive(Clone, Debug)]
pub struct Fail;

impl Goal for Fail {
    fn apply(&self, _: State) -> Option<State> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fail() {
        let state = State::new();
        let goal = Fail;
        let result = goal.apply(state);
        assert!(result.is_none());
    }
}
