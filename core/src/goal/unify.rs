use super::Goal;
use crate::domain::{Domain, IntoDomainVal};
use crate::state::State;
use crate::value::IntoVal;

pub(super) fn run<'a, D>(state: State<'a, D>, a: D::Value, b: D::Value) -> Option<State<'a, D>>
where
    D: Domain<'a>,
{
    D::unify_domain_values(state, a, b)
}

pub fn unify<'a, T, A, B, D>(a: A, b: B) -> Goal<'a, D>
where
    D: Domain<'a> + IntoDomainVal<'a, T>,
    A: IntoVal<T>,
    B: IntoVal<T>,
{
    Goal::Unify(
        D::into_domain_val(a.into_val()),
        D::into_domain_val(b.into_val()),
    )
}

#[cfg(test)]
mod tests {
    use super::unify;
    use crate::goal::Goal;
    use crate::tests::domains::Numbers;
    use crate::util;
    use crate::value::var;

    #[test]
    fn basic_unify_succeeds() {
        let x = var();
        let goal: Goal<Numbers> = unify(x, 5);
        let result = util::goal_resolves_to(goal, x);
        assert_eq!(result, vec![5]);
    }
}
