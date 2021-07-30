use crate::goals::{Goal, GoalEnum};
use crate::state::constraints::{Constraint, ResolveFn, TwoOfThree, VarWatch};
use crate::state::State;
use crate::value::{IntoVal, Val};
use crate::DomainType;
use crate::UnifyIn;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

/** Create a [projection goal](super) that allows deriving one resolved value
from the other two.

Functions must be provided to derive from any combination of two values.
Whichever two are resolved first will be used to derive the other.

```
use canrun::{Goal, all, unify, var, map_2};
use canrun::example::I32;

let (x, y, z) = (var(), var(), var());
let goal: Goal<I32> = all![
    unify(1, x),
    unify(2, y),
    map_2(x, y, z, |x, y| x + y, |x, z| z - x, |y, z| z - y),
];
let result: Vec<_> = goal.query(z).collect();
assert_eq!(result, vec![3])
```
*/
pub fn map_2<'a, A, AV, B, BV, C, CV, D, ABtoC, ACtoB, BCtoA>(
    a: AV,
    b: BV,
    c: CV,
    ab_to_c: ABtoC,
    ac_to_b: ACtoB,
    bc_to_a: BCtoA,
) -> Goal<'a, D>
where
    A: UnifyIn<'a, D> + Debug + 'a,
    AV: IntoVal<A>,
    B: UnifyIn<'a, D> + Debug + 'a,
    BV: IntoVal<B>,
    C: UnifyIn<'a, D> + Debug + 'a,
    CV: IntoVal<C>,
    D: DomainType<'a, A> + DomainType<'a, B> + DomainType<'a, C>,
    ABtoC: Fn(&A, &B) -> C + 'a,
    ACtoB: Fn(&A, &C) -> B + 'a,
    BCtoA: Fn(&B, &C) -> A + 'a,
{
    Goal(GoalEnum::Constraint(Rc::new(Map2 {
        a: a.into_val(),
        b: b.into_val(),
        c: c.into_val(),
        ab_to_c: Rc::new(ab_to_c),
        ac_to_b: Rc::new(ac_to_b),
        bc_to_a: Rc::new(bc_to_a),
    })))
}

pub struct Map2<'a, A: Debug, B: Debug, C: Debug> {
    a: Val<A>,
    b: Val<B>,
    c: Val<C>,
    ab_to_c: Rc<dyn Fn(&A, &B) -> C + 'a>,
    ac_to_b: Rc<dyn Fn(&A, &C) -> B + 'a>,
    bc_to_a: Rc<dyn Fn(&B, &C) -> A + 'a>,
}

impl<'a, A: Debug, B: Debug, C: Debug> Debug for Map2<'a, A, B, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Map2 {:?} {:?} {:?}", self.a, self.b, self.c)
    }
}

impl<'a, A, B, C, Dom> Constraint<'a, Dom> for Map2<'a, A, B, C>
where
    A: UnifyIn<'a, Dom> + Debug + 'a,
    B: UnifyIn<'a, Dom> + Debug + 'a,
    C: UnifyIn<'a, Dom> + Debug + 'a,
    Dom: DomainType<'a, A> + DomainType<'a, B> + DomainType<'a, C> + 'a,
{
    fn attempt(&self, state: &State<'a, Dom>) -> Result<ResolveFn<'a, Dom>, VarWatch> {
        use TwoOfThree::*;
        let resolved = TwoOfThree::resolve(&self.a, &self.b, &self.c, state)?;
        // let a = state.resolve_val(&self.a).clone();
        // let b = state.resolve_val(&self.b).clone();
        // let c = state.resolve_val(&self.c).clone();
        match resolved {
            AB(a, b, c) => {
                let f = self.ab_to_c.clone();
                Ok(Box::new(move |state| {
                    state.unify(&f(&*a, &*b).into_val(), &c)
                }))
            }
            BC(a, b, c) => {
                let f = self.bc_to_a.clone();
                Ok(Box::new(move |state| {
                    state.unify(&f(&*b, &*c).into_val(), &a)
                }))
            }
            AC(a, b, c) => {
                let f = self.ac_to_b.clone();
                Ok(Box::new(move |state| {
                    state.unify(&f(&*a, &*c).into_val(), &b)
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::map_2;
    use crate::example::I32;
    use crate::goals::unify::unify;
    use crate::goals::Goal;
    use crate::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let y = var();
        let z = var();
        let goals: Vec<Goal<I32>> = vec![
            unify(1, x),
            unify(2, y),
            unify(3, z),
            map_2(x, y, z, |x, y| x + y, |x, z| z - x, |y, z| z - y),
        ];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![(1, 2, 3)]);
    }

    #[test]
    fn debug_impl() {
        let goal: Goal<I32> = map_2(1, 2, 3, |x, y| x + y, |x, z| z - x, |y, z| z - y);
        assert_ne!(format!("{:?}", goal), "");
    }
}
