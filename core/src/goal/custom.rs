use super::Goal;
use crate::domain::Domain;
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

pub fn custom<'a, D, F>(func: F) -> Goal<'a, D>
where
    D: Domain<'a>,
    F: Fn(State<'a, D>) -> Option<State<'a, D>> + 'a,
{
    Goal::Custom(Custom(Rc::new(func)))
}

impl<'a, D: Domain<'a>> fmt::Debug for Custom<'a, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Custom ??")
    }
}

#[cfg(test)]
mod tests {
    use super::custom;
    use crate::goal::Goal;
    use crate::tests::domains::Numbers;
    use crate::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let goal: Goal<Numbers> = custom(|s| s.unify(x, 1));
        let results = util::goal_resolves_to(goal, x);
        assert_eq!(results, vec![1]);
    }
}
