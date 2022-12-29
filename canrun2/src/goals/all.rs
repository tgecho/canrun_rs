use super::Goal;
use crate::core::State;

#[derive(Debug)]
pub struct All {
    goals: Vec<Box<dyn Goal>>,
}

impl<I: Iterator<Item = Box<dyn Goal>>> From<I> for All {
    fn from(iter: I) -> Self {
        All {
            goals: iter.collect(),
        }
    }
}

impl Goal for All {
    fn apply(&self, state: State) -> Option<State> {
        self.goals.iter().try_fold(state, |s, g| g.apply(s))
    }
}

#[macro_export]
macro_rules! all {
    ($($item:expr),* $(,)?) => {
        $crate::goals::all::All {goals:vec![$(Box::new($item)),*]}
    };
}
pub use all;

#[cfg(test)]
mod tests {
    use crate::{core::Query, goals::unify::unify, value::LVar};

    use super::all;

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let y = LVar::new();
        let goal = all![unify(y, x), unify(y, 1)];
        let result = goal.query((x, y)).collect::<Vec<_>>();
        assert_eq!(result, vec![(1, 1)]);
    }

    #[test]
    fn fails() {
        let x = LVar::new();
        let goal = all![unify(x, 5), unify(x, 7)];
        let result = goal.query(x).collect::<Vec<_>>();
        assert_eq!(result, vec![]);
    }
}
