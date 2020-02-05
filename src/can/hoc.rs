use crate::{pair, var, Can, CanT, LVar, ResolveResult, State, StateIter};
use im::HashSet;
use std::iter::{empty, once};

pub type HocUnifyFn<T> = fn(LVar, Can<T>, Can<T>, State<T>) -> StateIter<T>;

#[derive(Clone)]
pub struct HoC<T: CanT> {
    pub composed: bool,
    pub var: LVar,
    pub value: Box<Can<T>>,
    pub unify: HocUnifyFn<T>,
}

pub(crate) fn resolve<T: CanT + 'static>(
    state: &State<T>,
    hoc: &HoC<T>,
    history: &HashSet<LVar>,
) -> ResolveResult<T> {
    if state.contains_var(&hoc.var) {
        state.checked_resolve(&hoc.var.can(), history)
    } else if hoc.composed {
        Ok(Can::HoC(hoc.clone()))
    } else {
        Ok(Can::HoC(HoC {
            composed: false,
            var: hoc.var.clone(),
            value: Box::new(state.checked_resolve(&hoc.value, history)?),
            unify: hoc.unify,
        }))
    }
}

fn unify_combined<T: CanT + 'static>(
    var: LVar,
    value: Can<T>,
    other: Can<T>,
    state: State<T>,
) -> StateIter<T> {
    match value {
        Can::Pair { l, r } => match (*l, *r) {
            (Can::HoC(l), Can::HoC(r)) => {
                let l_unify = l.unify;
                let r_unify = r.unify;
                // The trick here is that we want to apply the inner hocs WITHOUT resolving their .var fields, which would loop back to the composed hoc
                // this clearly exposes the shoddiness of the overall data structure, but I think we've almost got something here.
                // needs more massaging and boiling down to find the hopefully elegant core in this small mess
                let state = state.assign(var, other.clone());
                let iter = l_unify(l.var, *l.value, other.clone(), state)
                    .zip(once(r).cycle())
                    .flat_map(move |(s, r)| r_unify(r.var, *r.value, other.clone(), s));
                Box::new(iter)
            }
            _ => Box::new(empty()),
        },
        _ => Box::new(empty()),
    }
}

pub(crate) fn unify<T: CanT + 'static>(a: HoC<T>, b: HoC<T>, state: &State<T>) -> StateIter<T> {
    let combined = Can::HoC(HoC {
        composed: true,
        var: var(),
        value: Box::new(pair(Can::HoC(a.clone()), Can::HoC(b.clone()))),
        unify: unify_combined,
    });
    // by definition we only arrive here if both a and b are unresolved, so we
    // can just assign directly to avoid resolving the contents
    Box::new(once(
        state
            .assign(a.var, combined.clone())
            .assign(b.var, combined),
    ))
}
