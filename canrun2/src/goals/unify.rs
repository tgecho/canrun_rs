use std::fmt::Debug;

use crate::core;
use crate::core::{State, Value};

use super::Goal;

/**
Create a [goal](crate::goals::Goal) that attempts to
[unify](crate::core::Unify) two values with each other. Create with [`unify`].
 */
#[derive(Debug)]
pub struct Unify<T: core::Unify> {
    a: Value<T>,
    b: Value<T>,
}

impl<T: core::Unify> Goal for Unify<T> {
    fn apply(&self, state: State) -> Option<State> {
        state.unify(&self.a, &self.b)
    }
}

/**
Create a [goal](crate::goals::Goal) that attempts to
[unify](crate::core::Unify) two values with each other.

If one of the values is an unbound [`LVar`](crate::LVar), it will be
bound to the other value. If both values are able to be resolved, they will
be compared with [`Unify::unify`](crate::Unify::unify). If this unification fails, the goal will fail.

# Examples

Unifying a fresh `LVar` will bind it to the other value:
```
use canrun2::{unify, LVar, Query};

let x = LVar::new();
let goal = unify(1, x);
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![1])
```

Attempting to unify two unequal values will fail:
```
# use canrun2::{unify, LVar, Query};
# let x: LVar<usize> = LVar::new();
let goal = unify(1, 2);
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![])
```
*/
pub fn unify<T, A, B>(a: A, b: B) -> Unify<T>
where
    T: core::Unify,
    A: Into<Value<T>>,
    B: Into<Value<T>>,
{
    Unify {
        a: a.into(),
        b: b.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::LVar;

    use super::*;

    #[test]
    fn deeply_nested_vars() {
        let x = LVar::new();
        let goal = unify(x, 1);
        let result = goal.apply(State::new());
        assert_eq!(result.unwrap().resolve(&x.into()), 1.into());
    }
}
