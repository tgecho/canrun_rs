use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

use crate::goals::Goal;
use crate::{
    constraints::{Constraint, ResolveFn},
    {LVarList, State, Unify, Value},
};
use crate::{ReadyState, Reify};

/** A [projection goal](super) that allows creating a new goal based on
the fully [reified](crate::core::Reify) value. Create with [`reified`].
*/
#[allow(clippy::type_complexity)]
pub struct Reified<A: Unify + Reify> {
    a: Value<A>,
    f: Rc<dyn Fn(<A as Reify>::Reified) -> Box<dyn Goal>>,
}

/** Create a [projection goal](super) that allows creating a new goal based on
the fully [reified](crate::core::Reify) value.

The key distinction between this and the [`project_1`](super::project_1::project_1) goal
is that this will fully reify nested structures, where [`project_1`](super::project_1::project_1)
will only resolve the top level value.

```
use canrun::{LVar, Query};
use canrun::goals::{reified, both, unify, Succeed, Fail};

let x = LVar::new();
let goal = both(unify(1, x), reified(x, |x| if x < 2 { Box::new(Succeed) } else { Box::new(Fail) }));
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![1])
```
*/
pub fn reified<A, IA, F>(a: IA, func: F) -> Reified<A>
where
    A: Unify + Reify,
    IA: Into<Value<A>>,
    F: Fn(<A as Reify>::Reified) -> Box<dyn Goal> + 'static,
{
    Reified {
        a: a.into(),
        f: Rc::new(func),
    }
}

impl<A: Unify + Reify> Clone for Reified<A> {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            f: self.f.clone(),
        }
    }
}

impl<A: Unify + Reify> Goal for Reified<A> {
    fn apply(&self, state: State) -> Option<State> {
        state.constrain(Rc::new(self.clone()))
    }
}

impl<A: Unify + Reify> Constraint for Reified<A> {
    fn attempt(&self, state: &State) -> Result<ResolveFn, LVarList> {
        // Normally a `ReadyState` is created with `State::ready()` and can only
        // be obtained from a `State` without open forks or constraints.
        // However, in this case we want to prune states as soon as possible, so
        // if we have enough resolved values we should go ahead and run the
        // `reified` callback as soon as we can.
        let ready_state = ReadyState::new(state.values.clone());
        let a = self.a.reify_in(&ready_state)?;
        let goal = (self.f)(a);
        Ok(Box::new(move |state| goal.apply(state)))
    }
}

impl<A: Unify + Reify + Debug> Debug for Reified<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Reified {:?}", self.a)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::LVar,
        goal_vec,
        goals::{fail::Fail, reified, succeed::Succeed, unify},
    };

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let y = LVar::new();
        let goals = goal_vec![
            unify(1, x),
            unify(2, y),
            reified(x, |x| {
                if x < 2 {
                    Box::new(Succeed)
                } else {
                    Box::new(Fail)
                }
            }),
            reified(y, |y| {
                if y < 3 {
                    Box::new(Succeed)
                } else {
                    Box::new(Fail)
                }
            }),
        ];
        goals.assert_permutations_resolve_to(&(x, y), vec![(1, 2)]);
    }

    #[test]
    fn fails() {
        let x = LVar::new();
        let goals = goal_vec![
            unify(1, x),
            reified(x, |x| {
                if x < 1 {
                    Box::new(Succeed)
                } else {
                    Box::new(Fail)
                }
            }),
        ];
        goals.assert_permutations_resolve_to(&x, vec![]);
    }
}
