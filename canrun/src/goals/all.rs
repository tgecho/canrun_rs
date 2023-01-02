use super::Goal;
use crate::core::State;

/**
A [`Goal`] that only succeeds if all sub-goals succeed.

See the [`all!`](all) macro for a more ergonomic way to construct static `All` goals.

Also implements [`From<Vec<Box<dyn Goal>>>`](From) and [`FromIterator<Box<dyn Goal>>`](FromIterator).

# Example
```
use canrun::{unify, LVar, Query};
use canrun::goals::{Goal, All};

let x = LVar::new();
let y = LVar::new();
let goals: Vec<Box<dyn Goal>> = vec![
    Box::new(unify(y, x)),
    Box::new(unify(1, x)),
    Box::new(unify(y, 1)),
];
let goal = All::from(goals);
let result: Vec<_> = goal.query((x, y)).collect();
assert_eq!(result, vec![(1, 1)])
```
*/
#[derive(Debug)]
pub struct All {
    goals: Vec<Box<dyn Goal>>,
}

impl From<Vec<Box<dyn Goal>>> for All {
    fn from(goals: Vec<Box<dyn Goal>>) -> Self {
        All { goals }
    }
}

impl FromIterator<Box<dyn Goal>> for All {
    fn from_iter<T: IntoIterator<Item = Box<dyn Goal>>>(iter: T) -> Self {
        All {
            goals: iter.into_iter().collect(),
        }
    }
}

impl Goal for All {
    fn apply(&self, state: State) -> Option<State> {
        self.goals.iter().try_fold(state, |s, g| g.apply(s))
    }
}

/**
Create a [goal](crate::goals::Goal) that only succeeds if all sub-goals
succeed.

This is essentially an "AND" operation on a vector of goals. The resulting
state will be the result of the combining all of the sub-goals.

If the any goal fails, the rest of the goals will not be attempted.

# Example
```
use canrun::{unify, LVar, Query, all};

let x = LVar::new();
let y = LVar::new();
let goal = all![unify(y, x), unify(1, x), unify(y, 1)];
let result: Vec<_> = goal.query((x, y)).collect();
assert_eq!(result, vec![(1, 1)])
```
*/
#[macro_export]
macro_rules! all {
    ($($item:expr),* $(,)?) => {
        {
            let goals: Vec<Box<dyn $crate::goals::Goal>> = vec![$(Box::new($item)),*];
            $crate::goals::All::from(goals)
        }
    };
}
pub use all;

#[cfg(test)]
mod tests {
    use crate::{core::LVar, core::Query, goals::unify};

    use super::all;

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let y = LVar::new();
        let goal = all![unify(y, x), unify(y, 1)];
        let result = goal.query((x, y)).collect::<Vec<_>>();
        assert_eq!(result, vec![(1, 1)]);
    }

    #[test]
    fn fails() {
        let x = LVar::new();
        let goal = all![unify(x, 5), unify(x, 7)];
        let result = goal.query(x).collect::<Vec<_>>();
        assert_eq!(result, vec![]);
    }
}
