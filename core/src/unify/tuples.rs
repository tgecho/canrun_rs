use super::Unify;
use crate::domain::DomainType;
use crate::state::State;
use crate::value::Val;
use std::rc::Rc;

impl<'a, A, B, D> Unify<'a, (Val<A>, Val<B>)> for State<'a, D>
where
    D: DomainType<'a, A> + DomainType<'a, B> + DomainType<'a, (Val<A>, Val<B>)>,
    Self: Unify<'a, A> + Unify<'a, B>,
{
    fn unify_resolved(self, l: Rc<(Val<A>, Val<B>)>, r: Rc<(Val<A>, Val<B>)>) -> Option<Self> {
        Some(
            self.unify(l.0.clone(), r.0.clone())?
                .unify(l.1.clone(), r.1.clone())?,
        )
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
        let goals: Vec<Goal<Numbers2>> =
            vec![unify(x, (val(1), val(2))), unify(x, (val(1), val(2)))];
        util::all_permutations_resolve_to(goals, x, vec![(val(1), val(2))]);
    }

    #[test]
    fn fails() {
        let x = var();
        let goals: Vec<Goal<Numbers2>> =
            vec![unify(x, (val(1), val(3))), unify(x, (val(1), val(2)))];
        util::all_permutations_resolve_to(goals, x, vec![]);
    }
}
