use std::fmt::Debug;
use std::rc::Rc;

use crate::core::State;

use super::Goal;

type LazyFun = dyn Fn() -> Box<dyn Goal>;

/**
A [Goal](crate::goals::Goal) that is generated via callback just as
it is about to be evaluated. Create with [`lazy`].
 */
pub struct Lazy {
    fun: Rc<LazyFun>,
}

impl Debug for Lazy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lazy")
            .field("fun", &"Rc<dyn Fn() -> Box<dyn Goal>>")
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
pub fn lazy<F>(fun: F) -> Lazy
where
    F: (Fn() -> Box<dyn Goal>) + 'static,
{
    Lazy { fun: Rc::new(fun) }
}

impl Goal for Lazy {
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
        assert_ne!(format!("{:?}", goal), "")
    }
}
