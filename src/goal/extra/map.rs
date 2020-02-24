use crate::constraint::{constrain, Constraint};
use crate::{
    equal, Can,
    Can::{Val, Var},
    CanT, Goal,
};

pub fn map_2<'a, T, AB, BA>(a: Can<T>, b: Can<T>, ab: AB, ba: BA) -> Goal<'a, T>
where
    T: CanT + 'a,
    AB: Fn(T) -> T + 'a,
    BA: Fn(T) -> T + 'a,
{
    constrain(Constraint::two(a, b, move |a, b| match (a, b) {
        (Val(a), b) => Ok(equal(ab(a), b)),
        (a, Val(b)) => Ok(equal(a, ba(b))),
        (Var(a), Var(b)) => Err(vec![a, b]),
        _ => Ok(Goal::Fail),
    }))
}

pub fn map_3<'a, T, AB, BC, AC>(
    a: Can<T>,
    b: Can<T>,
    c: Can<T>,
    ab: AB,
    bc: BC,
    ac: AC,
) -> Goal<'a, T>
where
    T: CanT + 'a,
    AB: Fn(T, T) -> T + 'a,
    BC: Fn(T, T) -> T + 'a,
    AC: Fn(T, T) -> T + 'a,
{
    constrain(Constraint::three(a, b, c, move |a, b, c| match (a, b, c) {
        (Val(a), Val(b), c) => Ok(equal(ab(a, b), c)),
        (a, Val(b), Val(c)) => Ok(equal(a, bc(b, c))),
        (Val(a), b, Val(c)) => Ok(equal(ac(a, c), b)),
        (Var(a), Var(b), Var(c)) => Err(vec![a, b, c]),
        (Var(a), Var(b), _) => Err(vec![a, b]),
        (_, Var(b), Var(c)) => Err(vec![b, c]),
        (Var(a), _, Var(c)) => Err(vec![a, c]),
        _ => Ok(Goal::Fail),
    }))
}

#[cfg(test)]
mod tests {
    use super::{map_2, map_3};
    use crate::util::test;
    use crate::{var, Can, Equals, Goal};

    fn incr(n: usize) -> usize {
        n + 1
    }
    fn decr(n: usize) -> usize {
        n - 1
    }
    #[test]
    fn should_succeed_map_2() {
        let (x, y) = (var(), var());
        let expected = vec![vec![Can::Val(1), Can::Val(2)]];

        let goals = vec![
            x.equals(1),
            y.equals(2),
            map_2(x.can(), y.can(), incr, decr),
        ];
        test::all_permutations_resolve_to(goals, &vec![x, y], expected.clone());

        let goals = vec![x.equals(1), map_2(x.can(), y.can(), incr, decr)];
        test::all_permutations_resolve_to(goals, &vec![x, y], expected.clone());

        let goals = vec![y.equals(2), map_2(x.can(), y.can(), incr, decr)];
        test::all_permutations_resolve_to(goals, &vec![x, y], expected);
    }
    #[test]
    fn should_fail_map_2() {
        let (x, y) = (var(), var());
        let goals = vec![
            x.equals(2),
            y.equals(1),
            map_2(x.can(), y.can(), incr, decr),
        ];
        test::all_permutations_resolve_to(goals, &vec![x, y], vec![]);
    }

    fn add<'a>(a: Can<usize>, b: Can<usize>, c: Can<usize>) -> Goal<'a, usize> {
        map_3(a, b, c, |a, b| a + b, |a, c| c - a, |b, c| c - b)
    }

    #[test]
    fn should_succeed_add() {
        let (x, y, z) = (var(), var(), var());
        let scenarios = vec![
            vec![
                add(x.can(), y.can(), z.can()),
                x.equals(1),
                y.equals(2),
                z.equals(3),
            ],
            vec![add(x.can(), y.can(), z.can()), y.equals(2), z.equals(3)],
            vec![add(x.can(), y.can(), z.can()), x.equals(1), z.equals(3)],
            vec![add(x.can(), y.can(), z.can()), x.equals(1), y.equals(2)],
        ];
        for goals in scenarios {
            let expected = vec![vec![Can::Val(1), Can::Val(2), Can::Val(3)]];
            test::all_permutations_resolve_to(goals, &vec![x, y, z], expected);
        }
    }
}
