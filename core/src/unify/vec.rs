use super::Unify;
use crate::domain::DomainType;
use crate::state::State;
use crate::value::Val;
use std::rc::Rc;

impl<'a, T, D> Unify<'a, Vec<Val<T>>> for State<'a, D>
where
    D: DomainType<'a, T> + DomainType<'a, Vec<Val<T>>>,
    Self: Unify<'a, T>,
{
    fn unify_resolved(self, a: Rc<Vec<Val<T>>>, b: Rc<Vec<Val<T>>>) -> Option<Self> {
        if a.len() == b.len() {
            a.iter()
                .zip(b.iter())
                .try_fold(self, |s: State<'a, D>, (a, b)| {
                    s.unify(a.clone(), b.clone())
                })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::goal::unify;
    use crate::goal::Goal;
    use crate::tests::domains::Numbers2;
    use crate::util;
    use crate::value::{val, var};

    #[test]
    fn succeeds() {
        let x = var();
        let goals: Vec<Goal<Numbers2>> = vec![
            unify(x, vec![val(1), val(2)]),
            unify(x, vec![val(1), val(2)]),
        ];
        util::all_permutations_resolve_to(goals, x, vec![vec![val(1), val(2)]]);
    }

    #[test]
    fn fails() {
        let x = var();
        let goals: Vec<Goal<Numbers2>> = vec![
            unify(x, vec![val(1), val(3)]),
            unify(x, vec![val(1), val(2)]),
        ];
        util::all_permutations_resolve_to(goals, x, vec![]);
    }

    #[test]
    fn nested_var() {
        let x = var();
        let y = var();
        let goals: Vec<Goal<Numbers2>> = vec![
            unify(x, vec![val(1), y.into_val()]),
            unify(x, vec![val(1), val(2)]),
        ];
        util::all_permutations_resolve_to(goals, y, vec![2]);
    }
}
