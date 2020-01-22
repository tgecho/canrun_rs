use crate::state::{Cell, State};
use std::iter::once;
// pub mod both;
// pub mod either;
// pub mod equal;
// pub mod with;

// pub trait Goal<T>: Clone
// where
//     T: Eq + Clone,
// {
//     fn run<'a>(self, state: &'a State<T>) -> Box<dyn Iterator<Item = State<T>> + 'a>;
// }

pub fn equal<T: Eq + Clone>(a: Cell<T>, b: Cell<T>) -> Goal<T> {
    Goal::Equal(EqualGoal { a, b })
}

pub fn both<T: Eq + Clone>(a: Goal<T>, b: Goal<T>) -> Goal<T> {
    Goal::Both(BothGoal {
        a: Box::new(a),
        b: Box::new(b),
    })
}

#[derive(Clone)]
pub enum Goal<T: Eq + Clone> {
    Equal(EqualGoal<T>),
    Both(BothGoal<T>),
}

impl<T: Eq + Clone + 'static> Goal<T> {
    fn run<'a>(self, state: &'a State<T>) -> Box<dyn Iterator<Item = State<T>>> {
        match self {
            Goal::Equal(goal) => Box::new(state.unify(&goal.a, &goal.b).into_iter())
                as Box<dyn Iterator<Item = State<T>>>,
            Goal::Both(goal) => {
                let a_states = goal.a.run(&state);
                let b_goals = once(goal.b).cycle();
                let ab_states = a_states.zip(b_goals).flat_map(|(s, b)| b.run(&s));
                Box::new(ab_states) as Box<dyn Iterator<Item = State<T>>>
            }
        }
    }
}

#[derive(Clone)]
pub struct BothGoal<T: Eq + Clone> {
    a: Box<Goal<T>>,
    b: Box<Goal<T>>,
}

#[derive(Clone)]
pub struct EqualGoal<T: Eq + Clone> {
    a: Cell<T>,
    b: Cell<T>,
}

// struct OwnedOptionIter<T>(Option<T>);

// impl<T: Clone> Iterator for OwnedOptionIter<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         match &mut self.0 {
//             Some(t) => {
//                 let result = Some(t.clone());
//                 self.0 = None;
//                 result
//             }
//             None => None,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::{equal, Goal};
//     use crate::lvar::LVar;
//     use crate::state::{Cell, State};
//     #[test]
//     fn basic_equal() {
//         let state: State<u32> = State::new();
//         let x = LVar::new();
//         let goal = equal(Cell::Var(x), Cell::Value(5));
//         let mut result = goal.run(&state);
//         assert_eq!(result.nth(0).unwrap().resolve_var(x), Cell::Value(5));
//     }
// }

#[cfg(test)]
mod tests {
    use super::{both, equal, Goal};
    // use crate::goal::equal::equal;
    use crate::lvar::LVar;
    use crate::state::{Cell, State};
    #[test]
    fn basic_both() {
        let state: State<usize> = State::new();
        let x = LVar::new();
        let xv = Cell::Var(x);
        let y = LVar::new();
        let yv = Cell::Var(y);
        let goal = both(equal(xv.clone(), Cell::Value(5)), equal(yv, Cell::Value(7)));
        let result = goal.run(&state).nth(0).unwrap();
        assert_eq!(result.resolve_var(x), Cell::Value(5));
        assert_eq!(result.resolve_var(y), Cell::Value(7));
    }
}
