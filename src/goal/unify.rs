use super::Goal;
use crate::core::domain::{Domain, IntoDomainVal};
use crate::core::state::State;

pub(super) fn run<'a, D>(state: State<'a, D>, a: D::Value, b: D::Value) -> Option<State<'a, D>>
where
    D: Domain<'a>,
{
    D::unify_domain_values(state, a, b)
}

pub fn unify<'a, V, D>(a: V, b: V) -> Goal<'a, D>
where
    D: Domain<'a>,
    V: IntoDomainVal<'a, D>,
{
    Goal::Unify(a.into_domain_val(), b.into_domain_val())
}

#[cfg(test)]
mod tests {
    use super::unify;
    use crate::core::tests::util;
    use crate::value::{val, var};

    #[test]
    fn basic_unify_succeeds() {
        let x = var();
        let goal = unify(x.clone(), val(5));
        let result = util::goal_resolves_to(goal, &x);
        assert_eq!(result, vec![5]);
    }
}
