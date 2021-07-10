use crate::goals::Goal;
use crate::state::constraints::{Constraint, OneOfTwo, ResolveFn, VarWatch};
use crate::DomainType;
use crate::State;
use crate::UnifyIn;
use crate::{IntoVal, Val};
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

/** Create a [projection goal](super) that allows deriving one resolved value
from the other.

Functions must be provided to derive in both directions. Whichever value is
resolved first will be used to derive the other.

```
use canrun::{Goal, all, unify, var, map_1};
use canrun::example::I32;

let (x, y) = (var(), var());
let goal: Goal<I32> = all![
    unify(1, x),
    map_1(x, y, |x| x + 1, |y| y - 1),
];
let result: Vec<_> = goal.query(y).collect();
assert_eq!(result, vec![2])
```
*/
pub fn map_1<'a, A, AV, B, BV, D, AtoB, BtoA>(
    a: AV,
    b: BV,
    a_to_b: AtoB,
    b_to_a: BtoA,
) -> Goal<'a, D>
where
    A: UnifyIn<'a, D> + Debug + 'a,
    B: UnifyIn<'a, D> + Debug + 'a,
    AV: IntoVal<A>,
    BV: IntoVal<B>,
    D: DomainType<'a, A> + DomainType<'a, B>,
    AtoB: Fn(&A) -> B + 'a,
    BtoA: Fn(&B) -> A + 'a,
{
    Goal::constraint(Map1 {
        a: a.into_val(),
        b: b.into_val(),
        a_to_b: Rc::new(a_to_b),
        b_to_a: Rc::new(b_to_a),
    })
}

pub struct Map1<'a, A: Debug, B: Debug> {
    a: Val<A>,
    b: Val<B>,
    a_to_b: Rc<dyn Fn(&A) -> B + 'a>,
    b_to_a: Rc<dyn Fn(&B) -> A + 'a>,
}

impl<'a, A: Debug, B: Debug> Debug for Map1<'a, A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Map1 {:?} {:?}", self.a, self.b)
    }
}

impl<'a, A, B, Dom> Constraint<'a, Dom> for Map1<'a, A, B>
where
    A: UnifyIn<'a, Dom> + Debug + 'a,
    B: UnifyIn<'a, Dom> + Debug + 'a,
    Dom: DomainType<'a, A> + DomainType<'a, B> + 'a,
{
    fn attempt(&self, state: &State<'a, Dom>) -> Result<ResolveFn<'a, Dom>, VarWatch> {
        let resolved = OneOfTwo::resolve(&self.a, &self.b, state)?;
        match resolved {
            OneOfTwo::A(a, b) => {
                let f = self.a_to_b.clone();
                Ok(Box::new(move |state| state.unify(&f(&*a).into_val(), &b)))
            }
            OneOfTwo::B(a, b) => {
                let f = self.b_to_a.clone();
                Ok(Box::new(move |state| state.unify(&f(&*b).into_val(), &a)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::map_1;
    use crate::example::I32;
    use crate::goals::unify::unify;
    use crate::goals::Goal;
    use crate::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let y = var();
        let goals: Vec<Goal<I32>> =
            vec![unify(1, x), unify(2, y), map_1(x, y, |x| x + 1, |y| y - 1)];
        util::assert_permutations_resolve_to(goals, (x, y), vec![(1, 2)]);
    }
}
