use crate::{Can, CanT, Goal, State, StateIter};

#[derive(Clone, PartialEq, Debug)]
pub struct Constraint<T: CanT> {
    pub left: Can<T>,
    pub right: Can<T>,
    pub func: fn(T, T) -> bool,
}

impl<'a, T: CanT + 'a> Constraint<T> {
    pub fn run(self, state: State<T>) -> StateIter<'a, T> {
        let constrained = match (self.left.clone(), self.right.clone()) {
            (Can::Var(left), _) => state.add_constraint(left, self),
            (_, Can::Var(right)) => state.add_constraint(right, self),
            (Can::Val(left), Can::Val(right)) => self.evaluate(left, right, &state),
            _ => None,
        };
        Box::new(constrained.into_iter())
    }

    pub fn evaluate(self, left: T, right: T, state: &State<T>) -> Option<State<T>> {
        let func = self.func;
        if func(left, right) {
            Some(state.clone())
        } else {
            None
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
