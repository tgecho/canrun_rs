use super::super::domain::{Domain, IntoDomainVal};
use super::super::state::State;
use super::Goal;
use std::rc::Rc;

pub(crate) fn run<'a, D>(
    state: State<'a, D>,
    a: Goal<'a, D>,
    b: Goal<'a, D>,
) -> Option<State<'a, D>>
where
    D: Domain<'a>,
{
    state.fork(Rc::new(move |s| {
        let a = a.clone().apply(s.clone()).into_iter();
        let b = b.clone().apply(s).into_iter();
        Box::new(a.chain(b))
    }))
}

pub fn either<'a, D>(a: Goal<'a, D>, b: Goal<'a, D>) -> Goal<'a, D>
where
    D: Domain<'a>,
{
    Goal::Either(Box::new(a), Box::new(b))
}

#[cfg(test)]
mod tests {
    use super::super::unify::unify;
    use super::either;
    use crate::core::tests::util;
    use crate::core::value::{val, var};

    #[test]
    fn either_both_succeeds() {
        let x = var();
        let goal = either(unify(x.clone(), val(5)), unify(x.clone(), val(7)));
        let results = util::goal_resolves_to(goal, &x);
        assert_eq!(results, vec![5, 7]);
    }

    #[test]
    fn either_one_succeeds() {
        let x = var();
        let bad = unify(val(6), val(5));

        let first = util::goal_resolves_to(either(unify(x.clone(), val(1)), bad.clone()), &x);
        assert_eq!(first, vec![1]);

        let second = util::goal_resolves_to(either(bad, unify(x.clone(), val(2))), &x);
        assert_eq!(second, vec![2]);
    }

    #[test]
    fn either_both_fail() {
        let x = var();
        let goal = either(unify(val(6), val(5)), unify(val(1), val(2)));
        let results = util::goal_resolves_to(goal, &x);
        assert_eq!(results, vec![]);
    }
}
