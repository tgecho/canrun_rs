use crate::core::Value;
use crate::core::{Constraint, OneOfTwo, ResolveFn, State, Unify, VarWatch};
use crate::goals::Goal;
use std::fmt::{self, Debug};
use std::rc::Rc;

pub fn map_1<A, AV, B, BV, AtoB, BtoA>(a: AV, b: BV, a_to_b: AtoB, b_to_a: BtoA) -> Map1<A, B>
where
    A: Unify,
    B: Unify,
    AV: Into<Value<A>>,
    BV: Into<Value<B>>,
    AtoB: Fn(&A) -> B + 'static,
    BtoA: Fn(&B) -> A + 'static,
{
    Map1 {
        a: a.into(),
        b: b.into(),
        a_to_b: Rc::new(a_to_b),
        b_to_a: Rc::new(b_to_a),
    }
}

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
    fn attempt(&self, state: &State) -> Result<ResolveFn, VarWatch> {
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
    use crate::goals::unify::unify;

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
}
