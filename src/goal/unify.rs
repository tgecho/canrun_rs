use super::Goal;
use crate::domain::{Domain, DomainType, IntoDomainVal};
use crate::state::State;

pub(super) fn run<'a, D>(state: State<'a, D>, a: D::Value, b: D::Value) -> Option<State<'a, D>>
where
    D: Domain<'a>,
{
    D::unify_domain_values(state, a, b)
}

pub fn unify<'a, T, A, B, D>(a: A, b: B) -> Goal<'a, D>
where
    D: Domain<'a> + DomainType<'a, T>,
    A: IntoDomainVal<'a, D>,
    B: IntoDomainVal<'a, D>,
{
    Goal::Unify(a.into_domain_val(), b.into_domain_val())
}

#[cfg(test)]
mod tests {
    use super::unify;
    use crate::tests::util;
    use crate::value::var;

    #[test]
    fn basic_unify_succeeds() {
        let x = var();
        let goal = unify(x, 5);
        let result = util::goal_resolves_to(goal, &x);
        assert_eq!(result, vec![5]);
    }
}
