use std::{iter::repeat, rc::Rc};

use super::Goal;
use crate::core::{Fork, State, StateIter};

/**
A [`Goal`] that yields a state for every successful
sub-goal.

See the [`any!`](any) macro for a more ergonomic way to
construct static `Any` goals.

Also implements [`From<Vec<Rc<dyn Goal>>>`](From) and [`FromIterator<Rc<dyn Goal>>`](FromIterator)/

# Example
```
use canrun::{any, unify, LVar, Query};
use canrun::goals::{Goal, Any};
use std::rc::Rc;

let x = LVar::new();
let goals: Vec<Rc<dyn Goal>> = vec![
    Rc::new(unify(x, 1)),
    Rc::new(unify(x, 2)),
    Rc::new(unify(x, 3)),
];
let goal = Any::from(goals);
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![1, 2, 3])
```
*/
#[derive(Debug, Clone)]
pub struct Any {
    goals: Vec<Rc<dyn Goal>>,
}

impl From<Vec<Rc<dyn Goal>>> for Any {
    fn from(goals: Vec<Rc<dyn Goal>>) -> Self {
        Any { goals }
    }
}

impl FromIterator<Rc<dyn Goal>> for Any {
    fn from_iter<T: IntoIterator<Item = Rc<dyn Goal>>>(iter: T) -> Self {
        Any {
            goals: iter.into_iter().collect(),
        }
    }
}

impl Goal for Any {
    fn apply(&self, state: State) -> Option<State> {
        state.fork(self.clone())
    }
}

impl Fork for Any {
    fn fork(&self, state: &State) -> StateIter {
        let goals = self.goals.clone().into_iter();
        let states = repeat(state.clone());
        dbg!(self.goals.len());
        Box::new(goals.zip(states).flat_map(|(g, s)| g.apply(s).into_iter()))
    }
}

/**
Create a [goal](crate::goals::Goal) that yields a state for every successful
sub-goal.

This is essentially an "OR" operation on a vector of goals. It may yield
from zero to as many resolved [states](crate::core::State) as there
are sub-goals.

# Examples

Each successful goal will yield a different result:
```
use canrun::{any, unify, LVar, Query};

let x = LVar::new();
let goal = any![unify(x, 1), unify(x, 2), unify(x, 3)];
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![1, 2, 3])
```

One failing goal will not cause the other to fail:
```
# use canrun::{any, unify, LVar, Query};
# let x = LVar::new();
let goal = any!(unify(1, 2), unify(x, 2), unify(x, 3));
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![2, 3])
```

All goals can fail, leading to no results:
```
# use canrun::{any, unify, LVar, Query};
# let x: LVar<usize> = LVar::new();
let goal = any!(unify(6, 5), unify(42, 0), unify(1, 2));
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![]) // Empty result
```
*/
#[macro_export]
macro_rules! any {
    ($($item:expr),* $(,)?) => {
        {
            let goals: Vec<std::rc::Rc<dyn $crate::goals::Goal>> = vec![$(std::rc::Rc::new($item)),*];
            $crate::goals::Any::from(goals)
        }
    };
}
pub use any;

#[cfg(test)]
mod tests {
    use crate::{
        core::LVar,
        core::Query,
        goals::{both::both, fail::Fail, unify},
    };

    use super::any;

    #[test]
    fn both_succeed() {
        let x = LVar::new();
        let goal = any![unify(x, 5), unify(x, 7)];
        let result = goal.query(x).collect::<Vec<_>>();
        assert_eq!(result, vec![5, 7]);
    }

    #[test]
    fn one_succeeds() {
        let x = LVar::new();
        let goal = any![unify(x, 5), both(Fail, unify(x, 7))];
        let result = goal.query(x).collect::<Vec<_>>();
        assert_eq!(result, vec![5]);
    }

    #[test]
    fn all_fail() {
        let x = LVar::new();
        let goal = any![both(Fail, unify(x, 5)), both(Fail, unify(x, 7))];
        let result = goal.query(x).collect::<Vec<_>>();
        assert_eq!(result, vec![]);
    }
}
