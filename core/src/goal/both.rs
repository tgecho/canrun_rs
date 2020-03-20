use super::Goal;
use crate::domain::Domain;
use crate::state::State;

pub(crate) fn run<'a, D>(
    state: State<'a, D>,
    a: Goal<'a, D>,
    b: Goal<'a, D>,
) -> Option<State<'a, D>>
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
    use super::both;
    use crate::domain::one::OfOne;
    use crate::goal::unify::unify;
    use crate::goal::Goal;
    use crate::tests::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let y = var();
        let goal: Goal<OfOne<i32>> = both(unify(x, 5), unify(y, 7));
        let result = util::goal_resolves_to(goal, (x, y));
        assert_eq!(result, vec![(5, 7)]);
    }

    #[test]
    fn fails() {
        let x = var();
        let goal: Goal<OfOne<i32>> = both(unify(x, 5), unify(x, 7));
        let result = util::goal_resolves_to(goal.clone(), x);
        assert_eq!(result, vec![]);
    }
}
