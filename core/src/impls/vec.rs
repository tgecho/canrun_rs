use crate::domains::DomainType;
use crate::state::State;
use crate::value::Val;
use crate::{ReifyIn, ResolvedState, UnifyIn};
use std::fmt::Debug;
use std::rc::Rc;

impl<'a, T, D> UnifyIn<'a, D> for Vec<Val<T>>
where
    T: UnifyIn<'a, D>,
    D: DomainType<'a, T> + DomainType<'a, Vec<Val<T>>>,
{
    fn unify_resolved(
        state: State<'a, D>,
        a: Rc<Vec<Val<T>>>,
        b: Rc<Vec<Val<T>>>,
    ) -> Option<State<'a, D>> {
        if a.len() == b.len() {
            a.iter()
                .zip(b.iter())
                .try_fold(state, |s: State<'a, D>, (a, b)| s.unify(a, b))
        } else {
            None
        }
    }
}

impl<'a, D, T> ReifyIn<'a, D> for Vec<Val<T>>
where
    T: ReifyIn<'a, D> + Debug,
    D: DomainType<'a, T> + 'a,
{
    type Reified = Vec<T::Reified>;
    fn reify_in(&self, state: &ResolvedState<D>) -> Option<Self::Reified> {
        self.into_iter().map(|v: &Val<T>| state.reify(v)).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::domains::example::VecI32;
    use crate::goal::unify;
    use crate::goal::Goal;
    use crate::util;
    use crate::value::{val, var};

    #[test]
    fn succeeds() {
        let x = var();
        let goals: Vec<Goal<VecI32>> = vec![
            unify(x, vec![val!(1), val!(2)]),
            unify(x, vec![val!(1), val!(2)]),
        ];
        util::assert_permutations_resolve_to(goals, x, vec![vec![1, 2]]);
    }

    #[test]
    fn fails() {
        let x = var();
        let goals: Vec<Goal<VecI32>> = vec![
            unify(x, vec![val!(1), val!(3)]),
            unify(x, vec![val!(1), val!(2)]),
        ];
        util::assert_permutations_resolve_to(goals, x, vec![]);
    }

    #[test]
    fn nested_var() {
        let x = var();
        let y = var::<i32>();
        let goals: Vec<Goal<VecI32>> = vec![
            unify(x, vec![val!(1), val!(y)]),
            unify(x, vec![val!(1), val!(2)]),
        ];
        util::assert_permutations_resolve_to(goals, y, vec![2]);
    }
}
