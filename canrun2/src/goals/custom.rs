use std::fmt;
use std::rc::Rc;

use crate::State;

use super::Goal;

/// A [Goal](crate::goals::Goal) that gives access to the underlying
/// [`State`](crate::core::State) struct. Create with [`custom`].
#[derive(Clone)]
pub struct Custom(Rc<dyn Fn(State) -> Option<State>>);

impl Goal for Custom {
    fn apply(&self, state: State) -> Option<State> {
        (self.0)(state)
    }
}

/**
Create a [goal](crate::goals::Goal) that gives access to the underlying
[`State`](crate::core::State) struct.

Similar to [`lazy`](crate::goals::lazy()), the passed in callback is given
access to the state so it can call the lower level [State] manipulation
methods. This should approach should be used sparingly. Ideally most logic
should be composable out of lower level primitive goals.

Because the [State] methods return an `Option<[State]>` the
[question mark operator `?`](https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html)
can be used to allow chaining operations on the [State].

# Examples

```
use canrun2::{custom, LVar, Query};

let x = LVar::new();
let goal = custom(move |state| {
    let y = LVar::new();
    state.unify(&y.into(), &1.into())?
         .unify(&x.into(), &y.into())
});
let result: Vec<_> = goal.query(x).collect();
assert_eq!(result, vec![1])
```
*/
pub fn custom<F>(func: F) -> Custom
where
    F: Fn(State) -> Option<State> + 'static,
{
    Custom(Rc::new(func))
}

impl fmt::Debug for Custom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Custom(<fn>)")
    }
}

#[cfg(test)]
mod tests {
    use super::custom;
    use crate::{LVar, Query};

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let goal = custom(move |s| s.unify(&x.into(), &1.into()));
        let results: Vec<_> = goal.query(x).collect();
        assert_eq!(results, vec![1]);
    }

    #[test]
    fn debug_impl() {
        let goal = custom(|_| None);
        assert_eq!(format!("{goal:?}"), "Custom(<fn>)");
    }
}
