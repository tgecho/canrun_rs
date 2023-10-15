use crate::constraints::TwoOfThree;
use crate::goals::Goal;
use crate::{
    constraints::{Constraint, ResolveFn},
    {LVarList, State, Unify, Value},
};
use std::fmt::{self, Debug};
use std::rc::Rc;

/** Create a [projection goal](super) that allows deriving one resolved value
from the other two.

Functions must be provided to derive from any combination of two values.
Whichever two are resolved first will be used to derive the other.

```
use canrun::{LVar, Query};
use canrun::goals::{map_2, all, unify};

let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
let goal = all![
    unify(1, x),
    unify(2, y),
    map_2(x, y, z, |x, y| x + y, |x, z| z - x, |y, z| z - y),
];
let result: Vec<_> = goal.query(z).collect();
assert_eq!(result, vec![3])
```
*/
pub fn map_2<A, B, C>(
    a: impl Into<Value<A>>,
    b: impl Into<Value<B>>,
    c: impl Into<Value<C>>,
    ab_to_c: impl Fn(&A, &B) -> C + 'static,
    ac_to_b: impl Fn(&A, &C) -> B + 'static,
    bc_to_a: impl Fn(&B, &C) -> A + 'static,
) -> Map2<A, B, C>
where
    A: Unify,
    B: Unify,
    C: Unify,
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

/** A [projection goal](super) that allows deriving one resolved value
from the other two. Create with [`map_2`].
*/
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
    fn attempt(&self, state: &State) -> Result<ResolveFn, LVarList> {
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
    use crate::goals::unify;

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
