use super::Goal;
use crate::domains::Domain;
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
    use crate::domains::example::I32;
    use crate::goal::unify::unify;
    use crate::goal::Goal;
    use crate::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let y = var();
        let goal: Goal<I32> = both(unify(x, y), unify(y, 7));
        let result = util::goal_resolves_to(goal, (x, y));
        assert_eq!(result, vec![(7, 7)]);
    }

    #[test]
    fn fails() {
        let x = var();
        let goal: Goal<I32> = both(unify(x, 5), unify(x, 7));
        let result = util::goal_resolves_to(goal.clone(), x);
        assert_eq!(result, vec![]);
    }
}
