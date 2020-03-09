use crate::constraint::{constrain, Constraint};
use crate::{
    Can,
    Can::{Val, Var},
    CanT, Goal,
};

pub fn constrain_1<'a, T, F>(a: Can<T>, func: F) -> Goal<'a, T>
where
    T: CanT + 'a,
    F: Fn(T) -> bool + 'a,
{
    constrain(Constraint::one(a, move |a| match a {
        Var(a) => Err(vec![a]),
        Val(a) => Ok(if func(a) { Goal::Succeed } else { Goal::Fail }),
        _ => Ok(Goal::Fail),
    }))
}

pub fn constrain_2<'a, T, F>(a: Can<T>, b: Can<T>, func: F) -> Goal<'a, T>
where
    T: CanT + 'a,
    F: Fn(T, T) -> bool + 'a,
{
    constrain(Constraint::two(a, b, move |a, b| match (a, b) {
        (Val(a), Val(b)) => Ok(if func(a, b) {
            Goal::Succeed
        } else {
            Goal::Fail
        }),
        (Var(a), Var(b)) => Err(vec![a, b]),
        (Var(a), _) => Err(vec![a]),
        (_, Var(b)) => Err(vec![b]),
        _ => Ok(Goal::Fail),
    }))
}

#[cfg(test)]
mod tests {
    use super::{constrain_1, constrain_2};
    use crate::util::test;
    use crate::{var, Can, Equals};

    #[test]
    fn should_succeed_constrain_1() {
        let x = var();
        let goals = vec![x.equals(2), constrain_1(x.can(), |x| x > 1)];
        test::all_permutations_resolve_to(goals, &vec![x], vec![vec![Can::Val(2)]]);
    }

    #[test]
    fn should_fail_constrain_1() {
        let x = var();
        let goals = vec![x.equals(1), constrain_1(x.can(), |x| x > 1)];
        test::all_permutations_resolve_to(goals, &vec![x], vec![]);
    }
    #[test]
    fn should_succeed_constrain_2() {
        let (x, y) = (var(), var());
        let goals = vec![
            x.equals(1),
            y.equals(2),
            constrain_2(x.can(), y.can(), |x, y| x < y),
        ];
        test::all_permutations_resolve_to(goals, &vec![x, y], vec![vec![Can::Val(1), Can::Val(2)]]);
    }
    #[test]
    fn should_fail_constrain_2() {
        let (x, y) = (var(), var());
        let goals = vec![
            x.equals(1),
            y.equals(2),
            constrain_2(x.can(), y.can(), |x, y| x > y),
        ];
        test::all_permutations_resolve_to(goals, &vec![x, y], vec![]);
    }

    #[test]
    fn should_fail_with_multiple_constraints() {
        let (x, y) = (var(), var());
        let goals = vec![
            constrain_2(x.can(), y.can(), |x, y| x < y),
            constrain_2(x.can(), y.can(), |x, y| x > y),
            x.equals(1),
            y.equals(2),
        ];
        let expected = vec![];
        test::all_permutations_resolve_to(goals, &vec![x, y], expected);
    }

    #[test]
    fn should_fail_with_multi_stepped_vars() {
        let (x, y, z, w) = (var(), var(), var(), var());
        let goals = vec![
            constrain_2(x.can(), y.can(), |x, y| x > y),
            z.equals(1),
            w.equals(2),
            x.equals(z.can()),
            y.equals(w.can()),
        ];
        let expected = vec![];
        test::all_permutations_resolve_to(goals, &vec![x, y], expected);
    }

    #[test]
    fn should_succeed_with_multi_stepped_vars() {
        let (x, y, z, w) = (var(), var(), var(), var());
        let goals = vec![
            constrain_2(x.can(), y.can(), |x, y| x < y),
            z.equals(1),
            w.equals(2),
            x.equals(z.can()),
            y.equals(w.can()),
        ];
        let expected = vec![vec![Can::Val(1), Can::Val(2)]];
        test::all_permutations_resolve_to(goals, &vec![x, y], expected);
    }
}
