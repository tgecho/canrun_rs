use crate::state;
use crate::{Can, CanT, Goal, State, StateIter};

#[derive(Clone, PartialEq, Debug)]
pub struct Constraint<T: CanT> {
    pub left: Can<T>,
    pub right: Can<T>,
    pub func: fn(T, T) -> bool,
}

impl<'a, T: CanT + 'a> Constraint<T> {
    pub fn run(self, state: State<T>) -> StateIter<'a, T> {
        match (self.left.clone(), self.right.clone()) {
            (Can::Var(left), _) => Box::new(
                state
                    .add_constraint(left, self)
                    .check_constraint(left.can()),
            ),
            (_, Can::Var(right)) => Box::new(
                state
                    .add_constraint(right, self)
                    .check_constraint(right.can()),
            ),
            (Can::Val(left), Can::Val(right)) => Box::new(self.evaluate(left, right).run(state)),
            _ => state::empty_iter(),
        }
    }

    pub fn evaluate(self, left: T, right: T) -> Goal<T> {
        let func = self.func;
        if func(left, right) {
            Goal::Succeed
        } else {
            Goal::Fail
        }
    }
}

pub fn constrain<T: CanT>(a: Can<T>, b: Can<T>, func: fn(T, T) -> bool) -> Goal<T> {
    Goal::Constrain(Constraint {
        left: a,
        right: b,
        func,
    })
}
