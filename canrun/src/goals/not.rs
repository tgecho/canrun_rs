use std::rc::Rc;

use super::Goal;
use crate::{
    constraints::{Constraint, ResolveFn},
    LVarList, State, StateIterator,
};

/**
A [Goal](crate::goals::Goal) that only succeeds if the sub-goal is proved to always fail.

See [`not()`] for more details.
*/
#[derive(Debug)]
pub enum Not {
    /// A `Not` with a sub-goal that failed quickly at creation time
    Fail,
    /// A `Not` goal that needs further evaluation to see if it will succeed.
    Maybe(Rc<NotConstraint>),
}

/**
Create a [Goal](crate::goals::Goal) that only succeeds if the sub-goal is proved to always fail.

This is implented using my interpretation of
[Negation as failure](https://en.wikipedia.org/wiki/Negation_as_failure). When created, it will
exhaustively run the sub-goal. If every possible iteration fails, it assumes that no
additional facts can change the result and so marks the outer goal as a success. If any
result state is a success, it adds a constraint that will continue checking until the
sub-goal has no unresolved variables or open constraints. At this point, if it succeeds
at least once, the outer `Not` will fail.

# Examples
```
use canrun::{State, Query, LVar, all, any, unify, not};

let x = LVar::new();
let goal = all![
    any![unify(x, 1), unify(x, 2)],
    not(unify(x, 1)),
];
let results: Vec<_> = goal.query(x).collect();
assert_eq!(results, vec![2]);
```

# Caveats
This is a somewhat recurring complication in the logic programming world, and I can't claim
a very deep understanding of the space. I have not yet found this approach to yield incorrect
results, but, well... proving a negative is hard!

# Performance considerations
This goal will do a speculative fork of the outer state in an attempt to search for success
states. It will short circuit as soon as one is found, but this could be a lot of computation
depending on the complexity of the outer state.

A `not()` that depends on unresolved [variables](crate::LVar) should work correctly, but
will require adding a constraint watch on these variables. If they are only resolved
within branches of a fork, this could involve a lot of repeating forking before the `not()`
is able to conclusively prove or disprove the sub-goal.

All of this is not to discourage usage, but just to say that you should try to keep them
relatively simple and as precise as possible.
*/
pub fn not(goal: impl Goal) -> Not {
    // We run the subgoal in isolation right up front for two reasons...
    let mut inner_states = goal.apply(State::new()).into_states().peekable();
    if inner_states.peek().is_none() {
        // if it fails right away, there shouldn't be anything down the line
        // that should be able to make it pass later so we can skip any
        // additional checking.
        Not::Fail
    } else {
        // if it succeeds
        Not::Maybe(Rc::new(NotConstraint {
            goal: Rc::new(goal),
            // note that we used .into_states() to make sure that we were
            // actually evaluating any inner forks and get an accurate
            // accounting of the vars involved
            vars: LVarList::flatten(inner_states.map(|s| s.vars())),
        }))
    }
}

impl Goal for Not {
    fn apply(&self, state: State) -> Option<State> {
        match self {
            Not::Fail => Some(state),
            Not::Maybe(constraint) => {
                let vars = constraint.vars.without_resolved_in(&state);
                state.constrain(Rc::new(NotConstraint {
                    goal: constraint.goal.clone(),
                    vars,
                }))
            }
        }
    }
}

/** A [`Not`] goal that needs to keep evaluating the state as variables are resolved. */
#[derive(Debug)]
pub struct NotConstraint {
    goal: Rc<dyn Goal>,
    vars: LVarList,
}

fn any_succeed(state: Option<State>) -> bool {
    state.into_states().next().is_some()
}

impl Constraint for NotConstraint {
    fn attempt(&self, state: &State) -> Result<ResolveFn, LVarList> {
        // If the internal goal succeeded...
        if any_succeed(self.goal.apply(state.clone())) {
            // This is the list of vars in the goal that are not resolved as of
            // the current state, before the goal may or may not have forked.
            let open_vars = self.vars.without_resolved_in(state);
            if open_vars.is_empty() {
                // There are no unresolved variables. We can fail now.
                Ok(Box::new(|_| None))
            } else {
                // There are unresolved variables. We need to wait.
                Err(open_vars)
            }
        } else {
            // The internal goal failed. So the not() succeeds!
            Ok(Box::new(Some))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        any, goal_vec,
        goals::{assert_1, fail::Fail, succeed::Succeed},
        unify, LVar,
    };

    #[test]
    fn succeeds_when_child_fails() {
        let state = State::new();
        let goal = not(Fail);
        let result = goal.apply(state);
        assert!(result.is_some());
    }

    #[test]
    fn fails_when_child_succeeds() {
        let state = State::new();
        let goal = not(Succeed);
        let result = goal.apply(state);
        assert!(result.is_none());
    }

    #[test]
    fn succeeds_with_unify() {
        let x = LVar::new();
        let goals = goal_vec![not(unify(x, 2)), unify(x, 1)];
        goals.assert_permutations_resolve_to(&x, vec![1]);
    }

    #[test]
    fn succeeds_with_constraints() {
        let x = LVar::new();
        let goals = goal_vec![not(assert_1(x, |x| *x == 2)), unify(x, 1)];
        goals.assert_permutations_resolve_to(&x, vec![1]);
    }

    #[test]
    fn fails_with_unify() {
        let x = LVar::new();
        let goals = goal_vec![not(unify(x, 1)), unify(x, 1)];
        goals.assert_permutations_resolve_to(&x, vec![]);
    }

    #[test]
    fn fails_with_constraints() {
        let x = LVar::new();
        let goals = goal_vec![not(assert_1(x, |x| *x == 1)), unify(x, 1)];
        goals.assert_permutations_resolve_to(&x, vec![]);
    }

    #[test]
    fn succeeds_with_forking_goals() {
        let x = LVar::new();
        let goals = goal_vec![unify(x, 1), not(any![unify(x, 2), unify(x, 3)])];
        goals.assert_permutations_resolve_to(&x, vec![1]);
    }

    #[test]
    fn fails_with_forking_goals() {
        let x = LVar::new();
        let goals = goal_vec![not(any![unify(x, 1), unify(x, 2)]), unify(x, 1)];
        goals.assert_permutations_resolve_to(&1, vec![]);
    }

    #[test]
    fn succeeds_with_adjacent_forking_goals() {
        let x = LVar::new();
        let goals = goal_vec![
            any![unify(x, 1), unify(x, 2)],
            not(any![unify(x, 1), unify(x, 3)])
        ];
        goals.assert_permutations_resolve_to(&x, vec![2]);
    }

    #[test]
    fn fails_with_adjacent_forking_goals() {
        let x = LVar::new();
        let y = LVar::new();
        let goals = goal_vec![
            not(any![unify(x, 1), unify(y, 1)]),
            any![unify(x, 1), unify(x, 1)],
            unify(y, 1)
        ];
        goals.assert_permutations_resolve_to(&x, vec![]);
    }
}
