use super::Goal;
use crate::domain::Domain;
use crate::state::IterResolved;
use crate::state::State;

pub(crate) fn run<'a, D>(state: State<'a, D>, goal: Goal<'a, D>) -> Option<State<'a, D>>
where
    D: Domain<'a>,
{
    let mut iter = goal.apply(state.clone()).resolved_iter();
    if iter.next().is_none() {
        Some(state)
    } else {
        None
    }
}

pub fn not<'a, D>(goal: Goal<'a, D>) -> Goal<'a, D>
where
    D: Domain<'a>,
{
    Goal::Not(Box::new(goal))
}

#[cfg(test)]
mod tests {
    use super::not;
    use crate::domain::one::OfOne;
    use crate::goal::both::both;
    use crate::goal::unify::unify;
    use crate::goal::Goal;
    use crate::tests::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let goal: Goal<OfOne<i32>> = both(unify(x, 1), not(unify(x, 2)));
        let results = util::goal_resolves_to(goal, &x);
        assert_eq!(results, vec![1]);
    }

    #[test]
    fn fails() {
        let x = var();
        let goal: Goal<OfOne<i32>> = both(unify(x, 5), not(unify(x, 5)));
        let results = util::goal_resolves_to(goal, &x);
        assert_eq!(results, vec![]);
    }
}
