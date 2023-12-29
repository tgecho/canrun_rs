use std::fmt::Debug;
use std::rc::Rc;

use crate::core::State;

use super::Goal;

/**
A [Goal](crate::goals::Goal) that is generated via callback just as
it is about to be evaluated. Create with [`lazy`].
 */
pub struct Lazy<G: Goal> {
    fun: Rc<dyn Fn() -> G>,
}

impl<G: Goal> Debug for Lazy<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lazy")
            .field("fun", &"Rc<dyn Fn() -> impl Goal>")
            .finish()
    }
}

/**
Create a [goal](crate::goals::Goal) that is generated via callback just as
it is about to be evaluated.

The primary uses for this function involve introducing new internal vars.
The passed in callback function should return a valid goal to be evaluated.

# Examples

```
use canrun::{lazy, both, unify, LVar, Query};

let x = LVar::new();
let goal = lazy(move || {
    let y = LVar::new();
    Box::new(both(unify(y, 1), unify(x, y)))
});
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![1])
```
*/
pub fn lazy<F, G>(fun: F) -> Lazy<G>
where
    G: Goal,
    F: (Fn() -> G) + 'static,
{
    Lazy { fun: Rc::new(fun) }
}

impl<G: Goal> Goal for Lazy<G> {
    fn apply(&self, state: State) -> Option<State> {
        let fun = &self.fun;
        let goal = fun();
        goal.apply(state)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{LVar, Value},
        goals::{succeed::Succeed, unify},
    };

    use super::*;

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let goal = lazy(move || Box::new(unify(x, 1)));
        let result = goal.apply(State::new());
        assert_eq!(result.unwrap().resolve(&x.into()), Value::new(1));
    }

    #[test]
    fn debug_impl() {
        let goal = lazy(|| Box::new(Succeed));
        assert_ne!(format!("{goal:?}"), "");
    }
}
