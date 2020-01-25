use super::Goal;
use super::{GoalIter, Pursue};
use crate::{Cell, State};

pub fn equal<T: Eq + Clone>(a: Cell<T>, b: Cell<T>) -> Goal<T> {
    Goal::Equal(EqualGoal { a, b })
}

impl<T: Eq + Clone + 'static> Pursue<T> for EqualGoal<T> {
    fn run<'a>(self, state: &'a State<T>) -> GoalIter<T> {
        Box::new(state.unify(&self.a, &self.b).into_iter())
    }
}

#[derive(Clone)]
pub struct EqualGoal<T: Eq + Clone> {
    pub a: Cell<T>,
    pub b: Cell<T>,
}

#[cfg(test)]
mod tests {
    use super::equal;
    use crate::{Cell, LVar, State};
    #[test]
    fn basic_equal() {
        let state: State<u32> = State::new();
        let x = LVar::new();
        let goal = equal(Cell::Var(x), Cell::Value(5));
        let mut result = goal.run(&state);
        assert_eq!(result.nth(0).unwrap().resolve_var(x), Cell::Value(5));
    }
}
