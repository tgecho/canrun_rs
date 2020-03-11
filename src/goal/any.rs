use super::Goal;
use crate::domain::Domain;
use crate::state::State;
use std::iter::repeat;
use std::rc::Rc;

pub(crate) fn run<'a, D>(state: State<'a, D>, goals: Vec<Goal<'a, D>>) -> Option<State<'a, D>>
where
    D: Domain<'a>,
{
    state.fork(Rc::new(move |s| {
        Box::new(
            goals
                .clone()
                .into_iter()
                .zip(repeat(s))
                .flat_map(|(g, s)| g.apply(s).into_iter()),
        )
    }))
}

pub fn any<'a, D>(goals: Vec<Goal<'a, D>>) -> Goal<'a, D>
where
    D: Domain<'a>,
{
    Goal::Any(goals)
}

#[cfg(test)]
mod tests {
    use super::any;
    use crate::domain::one::OfOne;
    use crate::goal::unify::unify;
    use crate::goal::Goal;
    use crate::tests::util;
    use crate::value::var;

    #[test]
    fn both_succeeds() {
        let x = var();
        let goal: Goal<OfOne<i32>> = any(vec![unify(x, 5), unify(x, 7)]);
        let results = util::goal_resolves_to(goal, &x);
        assert_eq!(results, vec![5, 7]);
    }

    #[test]
    fn one_succeeds() {
        let x = var();
        let bad: Goal<OfOne<i32>> = unify(6, 5);

        let first = util::goal_resolves_to(any(vec![unify(x, 1), bad.clone()]), &x);
        assert_eq!(first, vec![1]);

        let second = util::goal_resolves_to(any(vec![bad, unify(x, 2)]), &x);
        assert_eq!(second, vec![2]);
    }

    #[test]
    fn both_fail() {
        let x = var();
        let goal: Goal<OfOne<i32>> = any(vec![unify(6, 5), unify(1, 2)]);
        let results = util::goal_resolves_to(goal, &x);
        assert_eq!(results, vec![]);
    }
}
