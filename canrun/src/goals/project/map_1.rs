use crate::constraints::OneOfTwo;
use crate::goals::Goal;
use crate::{
    constraints::{Constraint, ResolveFn},
    {LVarList, State, Unify, Value},
};
use std::fmt::{self, Debug};
use std::rc::Rc;

/** Create a [projection goal](super) that allows deriving one resolved value
from the other.

Functions must be provided to derive in both directions. Whichever value is
resolved first will be used to derive the other.

```
use canrun::{LVar, Query};
use canrun::goals::{map_1, all, unify};

let (x, y) = (LVar::new(), LVar::new());
let goal = all![
    unify(1, x),
    map_1(x, y, |x| x + 1, |y| y - 1),
];
let result: Vec<_> = goal.query(y).collect();
assert_eq!(result, vec![2])
```
*/
pub fn map_1<A, B>(
    a: impl Into<Value<A>>,
    b: impl Into<Value<B>>,
    a_to_b: impl Fn(&A) -> B + 'static,
    b_to_a: impl Fn(&B) -> A + 'static,
) -> Map1<A, B>
where
    A: Unify,
    B: Unify,
{
    Map1 {
        a: a.into(),
        b: b.into(),
        a_to_b: Rc::new(a_to_b),
        b_to_a: Rc::new(b_to_a),
    }
}

/** A [projection goal](super) that allows deriving one resolved value
from the other. Create with [`map_1`].
*/
pub struct Map1<A: Unify, B: Unify> {
    a: Value<A>,
    b: Value<B>,
    a_to_b: Rc<dyn Fn(&A) -> B>,
    b_to_a: Rc<dyn Fn(&B) -> A>,
}

impl<A: Unify, B: Unify> Debug for Map1<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Map1 {:?} {:?}", self.a, self.b)
    }
}

impl<A: Unify, B: Unify> Clone for Map1<A, B> {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            a_to_b: self.a_to_b.clone(),
            b_to_a: self.b_to_a.clone(),
        }
    }
}

impl<A: Unify, B: Unify> Goal for Map1<A, B> {
    fn apply(&self, state: State) -> Option<State> {
        state.constrain(Rc::new(self.clone()))
    }
}

impl<A: Unify, B: Unify> Constraint for Map1<A, B> {
    fn attempt(&self, state: &State) -> Result<ResolveFn, LVarList> {
        let resolved = OneOfTwo::resolve(&self.a, &self.b, state)?;
        match resolved {
            OneOfTwo::A(a, b) => {
                let f = self.a_to_b.clone();
                Ok(Box::new(move |state| {
                    state.unify(&Value::new(f(a.as_ref())), &b)
                }))
            }
            OneOfTwo::B(a, b) => {
                let f = self.b_to_a.clone();
                Ok(Box::new(move |state| {
                    state.unify(&Value::new(f(b.as_ref())), &a)
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::map_1;
    use crate::core::LVar;
    use crate::core::Query;
    use crate::goals::both::both;
    use crate::goals::unify;

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let y = LVar::new();
        let goal = both(
            both(unify(1, x), unify(2, y)),
            map_1(x, y, |x| x + 1, |y| y - 1),
        );
        assert_eq!(goal.query((x, y)).collect::<Vec<_>>(), vec![(1, 2)]);
    }

    #[test]
    fn debug() {
        let x = LVar::new();
        let y = LVar::new();
        let goal = map_1(x, y, |x| x + 1, |y| y - 1);
        assert_ne!(format!("{goal:?}"), "");
    }
}
