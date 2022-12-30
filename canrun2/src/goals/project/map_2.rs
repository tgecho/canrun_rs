use crate::core::Value;
use crate::core::{Constraint, ResolveFn, State, TwoOfThree, Unify, VarWatch};
use crate::goals::Goal;
use std::fmt::{self, Debug};
use std::rc::Rc;

pub fn map_2<A, IA, B, IB, C, IC, ABtoC, ACtoB, BCtoA>(
    a: IA,
    b: IB,
    c: IC,
    ab_to_c: ABtoC,
    ac_to_b: ACtoB,
    bc_to_a: BCtoA,
) -> Map2<A, B, C>
where
    A: Unify,
    B: Unify,
    C: Unify,
    IA: Into<Value<A>>,
    IB: Into<Value<B>>,
    IC: Into<Value<C>>,
    ABtoC: Fn(&A, &B) -> C + 'static,
    ACtoB: Fn(&A, &C) -> B + 'static,
    BCtoA: Fn(&B, &C) -> A + 'static,
{
    Map2 {
        a: a.into(),
        b: b.into(),
        c: c.into(),
        ab_to_c: Rc::new(ab_to_c),
        ac_to_b: Rc::new(ac_to_b),
        bc_to_a: Rc::new(bc_to_a),
    }
}

#[allow(clippy::type_complexity)]
pub struct Map2<A: Unify, B: Unify, C: Unify> {
    a: Value<A>,
    b: Value<B>,
    c: Value<C>,
    ab_to_c: Rc<dyn Fn(&A, &B) -> C>,
    ac_to_b: Rc<dyn Fn(&A, &C) -> B>,
    bc_to_a: Rc<dyn Fn(&B, &C) -> A>,
}

impl<A: Unify, B: Unify, C: Unify> Debug for Map2<A, B, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Map2 {:?} {:?} {:?}", self.a, self.b, self.c)
    }
}

impl<A: Unify, B: Unify, C: Unify> Clone for Map2<A, B, C> {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            c: self.c.clone(),
            ab_to_c: self.ab_to_c.clone(),
            ac_to_b: self.ac_to_b.clone(),
            bc_to_a: self.bc_to_a.clone(),
        }
    }
}

impl<A: Unify, B: Unify, C: Unify> Goal for Map2<A, B, C> {
    fn apply(&self, state: State) -> Option<State> {
        state.constrain(Rc::new(self.clone()))
    }
}

impl<A: Unify, B: Unify, C: Unify> Constraint for Map2<A, B, C> {
    fn attempt(&self, state: &State) -> Result<ResolveFn, VarWatch> {
        let resolved = TwoOfThree::resolve(&self.a, &self.b, &self.c, state)?;
        match resolved {
            TwoOfThree::AB(a, b, c) => {
                let f = self.ab_to_c.clone();
                Ok(Box::new(move |state| {
                    state.unify(&Value::new(f(&*a, &*b)), &c)
                }))
            }
            TwoOfThree::BC(a, b, c) => {
                let f = self.bc_to_a.clone();
                Ok(Box::new(move |state| {
                    state.unify(&Value::new(f(&*b, &*c)), &a)
                }))
            }
            TwoOfThree::AC(a, b, c) => {
                let f = self.ac_to_b.clone();
                Ok(Box::new(move |state| {
                    state.unify(&Value::new(f(&*a, &*c)), &b)
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::map_2;
    use crate::core::{LVar, Query};
    use crate::goals::both::both;
    use crate::goals::unify::unify;

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let y = LVar::new();
        let z = LVar::new();
        let goal = both(
            both(both(unify(1, x), unify(2, y)), unify(3, z)),
            map_2(x, y, z, |x, y| x + y, |x, z| z - x, |y, z| z - y),
        );
        assert_eq!(goal.query((x, y)).collect::<Vec<_>>(), vec![(1, 2)]);
    }
}
