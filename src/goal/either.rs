use super::Goal;
use super::{GoalIter, Pursue};
use crate::State;
use itertools::Itertools;

pub fn either<T: Eq + Clone>(a: Goal<T>, b: Goal<T>) -> Goal<T> {
    Goal::Either(EitherGoal {
        a: Box::new(a),
        b: Box::new(b),
    })
}

#[derive(Clone)]
pub struct EitherGoal<T: Eq + Clone + 'static> {
    pub a: Box<Goal<T>>,
    pub b: Box<Goal<T>>,
}

impl<T: Eq + Clone + 'static> Pursue<T> for EitherGoal<T> {
    fn run<'a>(self, state: &'a State<T>) -> GoalIter<T> {
        Box::new(self.a.run(&state).interleave(self.b.run(&state)))
    }
}

#[cfg(test)]
mod tests {
    use super::either;
    use crate::goal::equal;
    use crate::{Cell, LVar, State};
    #[test]
    fn basic_either() {
        let state: State<usize> = State::new();
        let x = LVar::new();
        let xv = Cell::Var(x);
        let goal = either(equal(xv.clone(), Cell::Value(5)), equal(xv, Cell::Value(6)));
        let mut results = goal.run(&state).map(|s| s.resolve_var(x));
        assert_eq!(results.nth(0).unwrap(), Cell::Value(5));
        assert_eq!(results.nth(0).unwrap(), Cell::Value(6));
    }
}
