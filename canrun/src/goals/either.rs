use std::rc::Rc;

use crate::core::{Fork, State, StateIter};

use super::Goal;

/**
A [Goal](crate::goals::Goal) that succeeds if either sub-goal
succeed. Create with [`either`].
 */
#[derive(Clone, Debug)]
pub struct Either {
    a: Rc<dyn Goal>,
    b: Rc<dyn Goal>,
}

/**
Create a [goal](crate::goals::Goal) that succeeds if either sub-goal
succeed.

This is essentially an "OR" operation, and will eventually lead to zero, one
or two [resolved states](crate::State), depending on the
success or failure of the sub-goals.

# Examples

Two successful goals will yield up two different results:
```
use canrun::{either, unify, LVar, Query};

let x = LVar::new();
let goal = either(unify(x, 1), unify(x, 2));
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![1, 2])
```

One failing goal will not cause the other to fail:
```
# use canrun::{either, unify, LVar, Query};
# let x = LVar::new();
let goal = either(unify(1, 2), unify(x, 3));
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![3])
```

Both goals can fail, leading to no results:
```
# use canrun::{either, unify, LVar, Query};
# let x: LVar<usize> = LVar::new();
let goal = either(unify(6, 5), unify(1, 2));
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![]) // Empty result
```
*/
pub fn either(a: impl Goal, b: impl Goal) -> Either {
    Either {
        a: Rc::new(a),
        b: Rc::new(b),
    }
}

impl Goal for Either {
    fn apply(&self, state: State) -> Option<State> {
        state.fork(self.clone())
    }
}

impl Fork for Either {
    fn fork(&self, state: &State) -> StateIter {
        let a = self.a.apply(state.clone()).into_iter();
        let b = self.b.apply(state.clone()).into_iter();
        Box::new(a.chain(b))
    }
}

#[cfg(test)]
mod test {
    use crate::core::StateIterator;

    use crate::goals::{fail::Fail, succeed::Succeed};

    use super::*;

    #[test]
    fn either_succeed() {
        let state = State::new();
        let goal = either(Succeed, Succeed);
        let result = goal.apply(state);
        assert_eq!(result.into_states().count(), 2);
    }

    #[test]
    fn either_succeed_or_fail() {
        let state = State::new();
        let goal = either(Succeed, Fail);
        let result = goal.apply(state);
        assert_eq!(result.into_states().count(), 1);
    }

    #[test]
    fn either_fail_or_succeed() {
        let state = State::new();
        let goal = either(Fail, Succeed);
        let result = goal.apply(state);
        assert_eq!(result.into_states().count(), 1);
    }

    #[test]
    fn either_fail() {
        let state = State::new();
        let goal = either(Fail, Fail);
        let result = goal.apply(state);
        assert_eq!(result.into_states().count(), 0);
    }
}
