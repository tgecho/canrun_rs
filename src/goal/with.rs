// #![feature(clone_closures, copy_closures)]

use super::Goal;

type WithFn<T> = dyn Fn() -> Goal<T>;

pub fn with<T: Eq + Clone> (func: WithFn<T>) -> Goal<T> {
    Goal::With(WithGoal(func))
}


#[derive(Clone)]
pub struct WithGoal<T: Eq + Clone + 'static>(pub WithFn<T>);

#[cfg(test)]
mod tests {
    use super::with;
    use crate::goal::{both, equal};
    use crate::lvar::LVar;
    use crate::state::{Cell, State};

    #[test]
    fn basic_with() {
        let state: State<u32> = State::new();
        let y = LVar::new();
        let y2 = y.clone();
        let f = || {
            let x = Cell::Var(LVar::new());
            let yy =Cell::Var(y);
            // let yy = Cell::Var(LVar::new());
            both(equal(x, Cell::Value(5)), equal(x, yy))
        }
        let goal = with(f);
        let mut result = goal.run(&state);
        // TODO: Use one of the fancier (as of yet unimplemented) goals to
        // inject a variable we can use to check this result
        assert!(result.nth(0).is_some());
    }
}
