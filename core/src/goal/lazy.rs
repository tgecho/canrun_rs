use super::Goal;
use crate::domain::Domain;
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

pub fn lazy<'a, D, F>(func: F) -> Goal<'a, D>
where
    D: Domain<'a>,
    F: Fn() -> Goal<'a, D> + 'a,
{
    Goal::Lazy(Lazy(Rc::new(func)))
}

impl<'a, D: Domain<'a>> fmt::Debug for Lazy<'a, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lazy ??")
    }
}

#[cfg(test)]
mod tests {
    use super::lazy;
    use crate::goal::unify::unify;
    use crate::goal::Goal;
    use crate::tests::domains::Numbers;
    use crate::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let goal: Goal<Numbers> = lazy(|| unify(x, 1));
        let results = util::goal_resolves_to(goal, x);
        assert_eq!(results, vec![1]);
    }
}
