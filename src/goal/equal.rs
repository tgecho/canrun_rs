use crate::{Can, CanT, Goal, State, StateIter, LVar};

pub fn equal<T, A, B>(a: A, b: B) -> Goal<T> where
T: CanT, A: Into<Can<T>>, B: Into<Can<T>> {
    Goal::Equal { a: a.into(), b: b.into() }
}

pub(crate) fn run<T: CanT + 'static>(state: &State<T>, a: &Can<T>, b: &Can<T>) -> StateIter<T> {
    Box::new(state.unify(a, b))
}

pub trait Equals<T: CanT> {
    fn equals<I: Into<Can<T>>>(self, value: I) -> Goal<T>;
}

impl <T: CanT> Equals<T> for Can<T> {
    fn equals<I: Into<Can<T>>>(self, value: I) -> Goal<T> {
        equal(self, value.into())
    }
}

impl <T: CanT> Equals<T> for &Can<T> {
    fn equals<I: Into<Can<T>>>(self, value: I) -> Goal<T> {
        equal(self.clone(), value.into())
    }
}

impl <T: CanT> Equals<T> for &LVar {
    fn equals<I: Into<Can<T>>>(self, value: I) -> Goal<T> {
        equal(self.can(), value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::{Equals};
    use crate::{Can, State, var};

    #[test]
    fn basic_equal() {
        let state: State<u32> = State::new();
        let x = var();
        let goal = x.equals(5);
        let mut result = goal.run(&state);
        assert_eq!(result.nth(0).unwrap().resolve_var(x).unwrap(), Can::Val(5));
    }
}
