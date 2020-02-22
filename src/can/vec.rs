use crate::state;
use crate::{Can, CanT, LVar, ResolveResult, State, StateIter};
use im::HashSet;

pub fn resolve<'a, T: CanT + 'a>(
    state: &State<'a, T>,
    vec: &[Can<T>],
    history: &HashSet<LVar>,
) -> ResolveResult<T> {
    let mut resolved = Vec::with_capacity(vec.len());
    for val in vec {
        resolved.push(state.checked_resolve(val, history)?);
    }
    Ok(Can::Vec(resolved))
}

pub fn unify<'a, T: CanT + 'a>(
    state: State<'a, T>,
    a: Vec<Can<T>>,
    b: Vec<Can<T>>,
) -> StateIter<'a, T> {
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
                Some(
                    states
                        .into_iter()
                        .flat_map(|s| s.unify(a.clone(), b.clone()))
                        .collect(),
                )
            }
        });
        match states {
            Some(states) => Box::new(states.into_iter()),
            None => state::empty_iter(),
        }
    } else {
        state::empty_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::{var, Can, State};

    #[test]
    fn unify_two_vecs() {
        let x = var();
        let state = State::new();
        let mut unified = state.unify(
            Can::Vec(vec![Can::Val(1), x.can(), Can::Val(3)]),
            Can::Vec(vec![Can::Val(1), Can::Val(2), Can::Val(3)]),
        );
        assert_eq!(unified.nth(0).unwrap().resolve_var(x).unwrap(), Can::Val(2));
    }
}
