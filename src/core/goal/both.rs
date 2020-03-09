use super::super::domain::{Domain, IntoDomainVal};
use super::super::state::State;
use super::Goal;

pub fn run<'a, D>(state: State<'a, D>, a: Goal<'a, D>, b: Goal<'a, D>) -> Option<State<'a, D>>
where
    D: Domain<'a>,
{
    a.apply(state).and_then(|s| b.apply(s))
}

pub fn both<'a, D>(a: Goal<'a, D>, b: Goal<'a, D>) -> Goal<'a, D>
where
    D: Domain<'a>,
{
    Goal::Both(Box::new(a), Box::new(b))
}

#[cfg(test)]
mod tests {
    use super::super::unify::unify;
    use super::both;
    use crate::core::tests::util;
    use crate::core::value::{val, var};

    #[test]
    fn basic_both_succeeds() {
        let x = var();
        let y = var();
        let goal = both(unify(x.clone(), val(5)), unify(y.clone(), val(7)));
        let result = util::goal_resolves_to(goal, (&x, &y));
        assert_eq!(result, vec![(5, 7)]);
    }

    #[test]
    fn basic_both_fails() {
        let x = var();
        let goal = both(unify(x.clone(), val(5)), unify(x.clone(), val(7)));
        let result = util::goal_resolves_to(goal.clone(), &x);
        assert_eq!(result, vec![]);
    }
}
