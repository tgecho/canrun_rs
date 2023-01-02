use super::Goal;
use crate::core::State;

/**
A [Goal](crate::goals::Goal) that only succeeds if both sub-goals succeed. Create with [`both`].
 */
#[derive(Debug)]
pub struct Both {
    a: Box<dyn Goal>,
    b: Box<dyn Goal>,
}

/**
Create a [goal](crate::goals::Goal) that only succeeds if both sub-goals
succeed.

This is essentially an "AND" operation. The resulting state will be the
result of the combining the two sub-goals.

If the first goal fails, the second goal will not be attempted.

# Examples

Two successful goals allow values to flow between vars:
```
use canrun::{both, unify, LVar, Query};

let x = LVar::new();
let y = LVar::new();
let goal = both(unify(y, x), unify(1, x));
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![1])
```

A failing goal will cause the entire goal to fail:
```
# use canrun::{both, unify, LVar, Query};
# let x = LVar::new();
let goal = both(unify(2, x), unify(1, x));
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![]) // Empty result
```
*/
pub fn both(a: impl Goal, b: impl Goal) -> Both {
    Both {
        a: Box::new(a),
        b: Box::new(b),
    }
}

impl Goal for Both {
    fn apply(&self, state: State) -> Option<State> {
        self.a.apply(state).and_then(|s| self.b.apply(s))
    }
}

#[cfg(test)]
mod test {
    use crate::goals::{fail::Fail, succeed::Succeed};

    use super::*;

    #[test]
    fn both_succeed() {
        let state = State::new();
        let goal = both(Succeed, Succeed);
        let result = goal.apply(state);
        assert!(result.is_some());
    }

    #[test]
    fn both_succeed_then_fail() {
        let state = State::new();
        let goal = both(Succeed, Fail);
        let result = goal.apply(state);
        assert!(result.is_none());
    }

    #[test]
    fn both_fail_then_succeed() {
        let state = State::new();
        let goal = both(Fail, Succeed);
        let result = goal.apply(state);
        assert!(result.is_none());
    }
}
