use crate::{Can, CanT, LVar, ResolveResult, State, StateIter};
use im::HashSet;
use std::iter::empty;

pub fn resolve<T: CanT + 'static>(
    state: &State<T>,
    vec: &[Can<T>],
    history: &HashSet<LVar>,
) -> ResolveResult<T> {
    let mut resolved = Vec::with_capacity(vec.len());
    for val in vec {
        resolved.push(state.checked_resolve(val, history)?);
    }
    Ok(Can::Vec(resolved))
}

pub fn unify<T: CanT + 'static>(state: &State<T>, a: Vec<Can<T>>, b: Vec<Can<T>>) -> StateIter<T> {
    if a.len() == b.len() {
        let mut pairs = a.iter().zip(b.iter());
        // Start with a single copy of the state
        let initial = vec![state.clone()];
        let states = pairs.try_fold(initial, |states, (a, b)| {
            // Try to unify the two sides in each state and flatten out the results
            // Failed unifications will return empty and those states will drop out
            if states.is_empty() {
                None
            } else {
                Some(states.iter().flat_map(|s| s.unify(a, b)).collect())
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
    use crate::{Can, State, var};

    #[test]
    fn unify_two_vecs() {
        let x = var();
        let mut unified = State::new().unify(
            &Can::Vec(vec![x.can()]),
            &Can::Vec(vec![Can::Val(2)]),
            // &Can::Vec(vec![Can::Val(1), Can::Var(x), Can::Val(3)]),
            // &Can::Vec(vec![Can::Val(1), Can::Val(2), Can::Val(3)]),
        );
        assert_eq!(unified.nth(0).unwrap().resolve_var(x).unwrap(), Can::Val(2));
    }
}
