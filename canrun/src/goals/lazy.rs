use super::{Goal, GoalEnum};
use crate::domains::Domain;
use crate::state::State;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Lazy<'a, D: Domain<'a>>(Rc<dyn Fn() -> Goal<'a, D> + 'a>);

impl<'a, D: Domain<'a>> Lazy<'a, D> {
    pub(crate) fn run(self, state: State<'a, D>) -> Option<State<'a, D>>
    where
        D: Domain<'a>,
    {
        let func = self.0;
        let goal = func();
        goal.apply(state)
    }
}

/// Create a [goal](crate::goals::Goal) that is generated via callback just as
/// it is about to be evaluated.
///
/// The primary uses for this function involve introducing new internal vars.
/// The passed in callback function should return a valid goal to be evaluated.
///
/// # Examples
///
/// ```
/// use canrun::{Goal, lazy, both, unify, var};
/// use canrun::example::I32;
///
/// let x = var();
/// let goal: Goal<I32> = lazy(|| {
///     let y = var();
///     both(unify(y, 1), unify(x, y))
/// });
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![1])
/// ```
pub fn lazy<'a, D, F>(func: F) -> Goal<'a, D>
where
    D: Domain<'a>,
    F: Fn() -> Goal<'a, D> + 'a,
{
    Goal(GoalEnum::Lazy(Lazy(Rc::new(func))))
}

impl<'a, D: Domain<'a>> fmt::Debug for Lazy<'a, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lazy ??")
    }
}

#[cfg(test)]
mod tests {
    use super::lazy;
    use crate::example::I32;
    use crate::goals::unify::unify;
    use crate::goals::Goal;
    use crate::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let goal: Goal<I32> = lazy(|| unify(x, 1));
        let results = util::goal_resolves_to(goal, x);
        assert_eq!(results, vec![1]);
    }

    #[test]
    fn debug_impl() {
        let goal: Goal<I32> = lazy(Goal::succeed);
        assert_ne!(format!("{:?}", goal), "")
    }
}
