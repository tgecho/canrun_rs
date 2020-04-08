use super::{Goal, GoalEnum};
use crate::domains::Domain;
use crate::state::State;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Custom<'a, D: Domain<'a>>(Rc<dyn Fn(State<'a, D>) -> Option<State<'a, D>> + 'a>);

impl<'a, D: Domain<'a>> Custom<'a, D> {
    pub(crate) fn run(self, state: State<'a, D>) -> Option<State<'a, D>>
    where
        D: Domain<'a>,
    {
        let func = self.0;
        func(state)
    }
}

// TODO: Add more illustrative examples

/// Create a [goal](crate::goal::Goal) that gives access to the underlying
/// [State](crate::state::State) struct.
///
/// Similar to [lazy](crate::goal::lazy()), the passed in callback is given access to
/// the state so it can call the lower level [State] manipulation methods. This
/// should approach should be used sparingly. Ideally most logic should be
/// composable out of lower level primitive goals.
///
/// Because the [State] methods return an `Option<[State]>` the
/// [question mark operator `?`](https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html)
/// can be used to allow chaining operations on the [State].
///
/// # Examples
///
/// ```
/// use canrun::{Goal, custom, val, var};
/// use canrun::domains::example::I32;
///
/// let x = var();
/// let goal: Goal<I32> = custom(|state| {
///     let y = var();
///     state.unify(&val!(y), &val!(1))?
///          .unify(&val!(x), &val!(y))
/// });
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![1])
/// ```
pub fn custom<'a, D, F>(func: F) -> Goal<'a, D>
where
    D: Domain<'a>,
    F: Fn(State<'a, D>) -> Option<State<'a, D>> + 'a,
{
    Goal(GoalEnum::Custom(Custom(Rc::new(func))))
}

impl<'a, D: Domain<'a>> fmt::Debug for Custom<'a, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Custom ??")
    }
}

#[cfg(test)]
mod tests {
    use super::custom;
    use crate::domains::example::I32;
    use crate::goal::Goal;
    use crate::util;
    use crate::value::{var, IntoVal};

    #[test]
    fn succeeds() {
        let x = var::<i32>();
        let goal: Goal<I32> = custom(|s| s.unify(&x.into_val(), &1.into_val()));
        let results = util::goal_resolves_to(goal, x);
        assert_eq!(results, vec![1]);
    }
}
