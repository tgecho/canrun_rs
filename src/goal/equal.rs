use super::Goal;
use crate::state::{Cell, State};

pub fn equal<T: Eq + Clone>(a: Cell<T>, b: Cell<T>) -> impl Goal<T> {
    EqualGoal { a, b }
}

#[derive(Clone)]
struct EqualGoal<T: Eq + Clone> {
    a: Cell<T>,
    b: Cell<T>,
}

impl<T: Eq + Clone> Goal<T> for EqualGoal<T> {
    fn run<'a>(self, state: &'a State<T>) -> Box<dyn Iterator<Item = State<T>> + 'a> {
        Box::new(EqualGoalIter {
            a: self.a,
            b: self.b,
            state,
            consumed: false,
        })
    }
}

struct EqualGoalIter<'a, T: Eq + Clone> {
    a: Cell<T>,
    b: Cell<T>,
    state: &'a State<T>,
    consumed: bool,
}

impl<'a, T: Eq + Clone> Iterator for EqualGoalIter<'a, T> {
    type Item = State<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed {
            None
        } else {
            self.consumed = true;
            self.state.unify(&self.a, &self.b)
        }
    }
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
        let mut result = goal.run(&state);
        assert_eq!(result.nth(0).unwrap().resolve_var(x), Cell::Value(5));
    }
}
