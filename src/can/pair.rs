use crate::{Can, CanT, GoalIter, State};

pub fn pair<T: CanT>(l: Can<T>, r: Can<T>) -> Can<T> {
    Can::Pair {
        l: Box::new(l),
        r: Box::new(r),
    }
}

pub fn resolve<T: CanT + 'static>(state: &State<T>, l: &Can<T>, r: &Can<T>) -> Can<T> {
    Can::Pair {
        l: Box::new(state.resolve(l)),
        r: Box::new(state.resolve(r)),
    }
}

pub fn unify<T: CanT + 'static>(
    state: &State<T>,
    al: Can<T>,
    ar: Can<T>,
    bl: Can<T>,
    br: Can<T>,
) -> GoalIter<T> {
    Box::new(state.unify(&al, &bl).flat_map(move |l| l.unify(&ar, &br)))
}
