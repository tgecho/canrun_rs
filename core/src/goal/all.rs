use super::Goal;
use crate::domains::Domain;
use crate::state::State;

pub(crate) fn run<'a, D>(state: State<'a, D>, goals: Vec<Goal<'a, D>>) -> Option<State<'a, D>>
where
    D: Domain<'a>,
{
    goals.into_iter().try_fold(state, |s, g| g.apply(s))
}

#[macro_export]
macro_rules! all {
    ($($item:expr),*) => {
        canrun::goal::Goal::All(vec![$($item),*])
    };
}
pub use all;

#[cfg(test)]
mod tests {
    use super::all;
    use crate as canrun;
    use crate::domains::example::I32;
    use crate::goal::unify::unify;
    use crate::goal::Goal;
    use crate::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let y = var();
        let goal: Goal<I32> = all![unify(x, 5), unify(y, 7)];
        let result = util::goal_resolves_to(goal, (x, y));
        assert_eq!(result, vec![(5, 7)]);
    }

    #[test]
    fn fails() {
        let x = var();
        let goal: Goal<I32> = all![unify(x, 5), unify(x, 7)];
        let result = util::goal_resolves_to(goal.clone(), x);
        assert_eq!(result, vec![]);
    }
}
