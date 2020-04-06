use super::Unify;
use crate::domains::DomainType;
use crate::state::State;
use crate::value::Val;
use std::rc::Rc;

impl<'a, T, D> Unify<'a, Vec<Val<T>>> for D
where
    D: Unify<'a, T> + DomainType<'a, Vec<Val<T>>>,
{
    fn unify_resolved(
        state: State<'a, Self>,
        a: Rc<Vec<Val<T>>>,
        b: Rc<Vec<Val<T>>>,
    ) -> Option<State<'a, Self>> {
        if a.len() == b.len() {
            a.iter()
                .zip(b.iter())
                .try_fold(state, |s: State<'a, D>, (a, b)| {
                    s.unify(a.clone(), b.clone())
                })
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! lvec {
    ($($item:expr),*) => {
        vec![$(canrun::value::IntoVal::into_val($item)),*]
    };
}

#[cfg(test)]
mod tests {
    use crate as canrun;
    use crate::domains::example::VecI32;
    use crate::goal::unify;
    use crate::goal::Goal;
    use crate::lvec;
    use crate::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let goals: Vec<Goal<VecI32>> = vec![unify(x, lvec![1, 2]), unify(x, lvec![1, 2])];
        util::all_permutations_resolve_to(goals, x, vec![lvec![1, 2]]);
    }

    #[test]
    fn fails() {
        let x = var();
        let goals: Vec<Goal<VecI32>> = vec![unify(x, lvec![1, 3]), unify(x, lvec![1, 2])];
        util::all_permutations_resolve_to(goals, x, vec![]);
    }

    #[test]
    fn nested_var() {
        let x = var();
        let y = var::<i32>();
        let goals: Vec<Goal<VecI32>> = vec![unify(x, lvec![1, y]), unify(x, lvec![1, 2])];
        util::all_permutations_resolve_to(goals, y, vec![2]);
    }
}
