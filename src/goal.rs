use crate::state::{Cell, State};

// Box<dyn Iterator<Item = State<T>>>
pub trait Goal<T>
where
    T: Eq + Clone,
{
    fn run<'a>(self, state: &State<T>) -> Vec<State<T>>;
}

pub struct EqualGoal<T: Eq + Clone> {
    a: Cell<T>,
    b: Cell<T>,
}

impl<T: Eq + Clone> Goal<T> for EqualGoal<T> {
    fn run<'a>(self, state: &'a State<T>) -> Vec<State<T>> {
        match state.unify(&self.a, &self.b) {
            Some(state) => vec![state],
            None => vec![],
        }
    }
}

pub fn equal<T: Eq + Clone>(a: Cell<T>, b: Cell<T>) -> impl Goal<T> {
    EqualGoal { a, b }
}

#[cfg(test)]
mod tests {
    use super::{equal, Goal};
    use crate::lvar::LVar;
    use crate::state::{Cell, State};
    #[test]
    fn basic_equal() {
        let state: State<u32> = State::new();
        let x = LVar::new();
        let goal = equal(Cell::Var(x), Cell::Value(5));
        let result = goal.run(&state);
        assert_eq!(result[0].resolve_var(x), Cell::Value(5));
    }
}
