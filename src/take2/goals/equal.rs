use super::{Goal, StateIter};
use crate::take2::state::Unify;
use crate::take2::val::Val;

#[derive(Clone)]
struct Equal<T> {
    a: Val<T>,
    b: Val<T>,
}

impl<'a, T> Goal<'a, T> for Equal<T> {
    fn run<S: Unify<'a, T> + 'a>(self, state: S) -> StateIter<'a, S> {
        state.unify(self.a, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::Equal;
    use crate::take2::domain::Just;
    use crate::take2::goals::AddGoal;
    use crate::take2::state::State;
    use crate::take2::val::r;

    #[test]
    fn basic_equal() {
        let mut state: State<Just<i32>> = State::new();
        state.add_goal(Equal { a: r(5), b: r(5) });

        // state.either(|s| s.equal(r(6), r(7)), |s| s.equal(r(6), r(7)))
        // let goal = x.equals(5);
        // let mut result = goal.run(state);
        // assert_eq!(result.nth(0).unwrap().resolve_var(x).unwrap(), Can::Val(5));
    }
}
