use crate::domains::DomainType;
use crate::state::constraints::{resolve_2, Constraint, ResolveFn, VarWatch};
use crate::value::{IntoVal, Val};
use crate::{Goal, State};
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

pub struct Project2<'a, A, B, D>
where
    A: Debug,
    B: Debug,
    D: DomainType<'a, A> + DomainType<'a, B>,
{
    a: Val<A>,
    b: Val<B>,
    #[allow(clippy::type_complexity)]
    f: Rc<dyn Fn(Rc<A>, Rc<B>) -> Goal<'a, D> + 'a>,
}

/** Create a [projection goal](super) that allows creating a new goal based on
the resolved values.

```
use canrun::{Goal, all, unify, var, project_2};
use canrun::example::I32;

let (x, y) = (var(), var());
let goal: Goal<I32> = all![
    unify(1, x),
    unify(2, y),
    project_2(x, y, |x, y| if x < y { Goal::succeed() } else { Goal::fail() }),
];
let result: Vec<_> = goal.query((x, y)).collect();
assert_eq!(result, vec![(1, 2)])
```
*/
pub fn project_2<'a, A, AV, B, BV, D, F>(a: AV, b: BV, func: F) -> Goal<'a, D>
where
    A: Debug + 'a,
    AV: IntoVal<A>,
    B: Debug + 'a,
    BV: IntoVal<B>,
    D: DomainType<'a, A> + DomainType<'a, B>,
    F: Fn(Rc<A>, Rc<B>) -> Goal<'a, D> + 'a,
{
    Goal::constraint(Project2 {
        a: a.into_val(),
        b: b.into_val(),
        f: Rc::new(func),
    })
}

impl<'a, A, B, Dom> Constraint<'a, Dom> for Project2<'a, A, B, Dom>
where
    A: Debug,
    B: Debug,
    Dom: DomainType<'a, A> + DomainType<'a, B>,
{
    fn attempt(&self, state: &State<'a, Dom>) -> Result<ResolveFn<'a, Dom>, VarWatch> {
        let (a, b) = resolve_2(&self.a, &self.b, state)?;
        let goal = (self.f)(a, b);
        Ok(Box::new(move |state| goal.apply(state)))
    }
}

impl<'a, A, B, D> Debug for Project2<'a, A, B, D>
where
    A: Debug,
    B: Debug,
    D: DomainType<'a, A> + DomainType<'a, B>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Project2 {:?} {:?}", self.a, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::project_2;
    use crate::example::I32;
    use crate::goals::Goal;

    #[test]
    fn debug_impl() {
        let goal: Goal<I32> = project_2(
            1,
            2,
            |_, _| Goal::succeed(), // coverage-ignore
        );
        assert_ne!(format!("{:?}", goal), "");
    }
}
