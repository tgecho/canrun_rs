use crate::{Can, CanT, GoalIter, State};
use std::iter::empty;

pub fn resolve<T: CanT + 'static>(state: &State<T>, vec: &Vec<Can<T>>) -> Can<T> {
    let resolved = vec.iter().map(|i| state.resolve(i));
    Can::Vec(resolved.collect())
}

pub fn unify<T: CanT + 'static>(state: &State<T>, a: Vec<Can<T>>, b: Vec<Can<T>>) -> GoalIter<T> {
    if a.len() == b.len() {
        let mut pairs = a.iter().zip(b.iter());
        // Start with a single copy of the state
        let initial = vec![state.clone()];
        let states = pairs.try_fold(initial, |states, (a, b)| {
            // Try to unify the two sides in each state and flatten out the results
            // Failed unifications will return empty and those states will drop out
            if states.len() > 0 {
                Some(states.iter().flat_map(|s| s.unify(a, b)).collect())
            } else {
                None
            }
        });
        match states {
            Some(states) => Box::new(states.into_iter()),
            None => Box::new(empty()),
        }
    } else {
        Box::new(empty())
    }
}

#[cfg(test)]
mod tests {
    use super::{Can, State};
    use crate::LVar;

    #[test]
    fn unify_two_vecs() {
        let x = LVar::new();
        let mut unified = State::new().unify(
            &Can::Vec(vec![Can::Val(1), Can::Var(x), Can::Val(3)]),
            &Can::Vec(vec![Can::Val(1), Can::Val(2), Can::Val(3)]),
        );
        assert_eq!(unified.nth(0).unwrap().resolve_var(x), Can::Val(2));
    }
}
