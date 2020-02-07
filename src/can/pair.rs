use crate::{Can, CanT, LVar, ResolveResult, State, StateIter};
use im::HashSet;
use std::iter::once;

pub fn pair<T: CanT>(l: Can<T>, r: Can<T>) -> Can<T> {
    Can::Pair {
        l: Box::new(l),
        r: Box::new(r),
    }
}

pub fn resolve<T: CanT>(
    state: &State<T>,
    l: &Can<T>,
    r: &Can<T>,
    history: &HashSet<LVar>,
) -> ResolveResult<T> {
    Ok(Can::Pair {
        l: Box::new(state.checked_resolve(l, history)?),
        r: Box::new(state.checked_resolve(r, history)?),
    })
}

pub fn unify<'a, T: CanT + 'a>(
    state: State<T>,
    al: Can<T>,
    ar: Can<T>,
    bl: Can<T>,
    br: Can<T>,
) -> StateIter<'a, T> {
    Box::new(
        state
            .unify(al, bl)
            .zip(once((ar, br)).cycle())
            .flat_map(|(l, (ar, br)): (State<T>, (Can<T>, Can<T>))| l.unify(ar, br)),
    )
}
