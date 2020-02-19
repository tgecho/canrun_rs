use crate::{Can, CanT, Goal, LVar, State, StateIter};

pub fn equal<'a, T, A, B>(a: A, b: B) -> Goal<'a, T>
where
    T: CanT,
    A: Into<Can<T>>,
    B: Into<Can<T>>,
{
    Goal::Equal {
        a: a.into(),
        b: b.into(),
    }
}

pub(crate) fn run<'a, T: CanT + 'a>(state: State<T>, a: Can<T>, b: Can<T>) -> StateIter<'a, T> {
    Box::new(state.unify(a, b))
}

pub trait Equals<'a, T: CanT> {
    fn equals<I: Into<Can<T>>>(self, value: I) -> Goal<'a, T>;
}

impl<'a, T: CanT> Equals<'a, T> for Can<T> {
    fn equals<I: Into<Can<T>>>(self, value: I) -> Goal<'a, T> {
        equal(self, value.into())
    }
}

impl<'a, T: CanT> Equals<'a, T> for &Can<T> {
    fn equals<I: Into<Can<T>>>(self, value: I) -> Goal<'a, T> {
        equal(self.clone(), value.into())
    }
}

impl<'a, T: CanT> Equals<'a, T> for &LVar {
    fn equals<I: Into<Can<T>>>(self, value: I) -> Goal<'a, T> {
        equal(self.can(), value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::Equals;
    use crate::{var, Can, State};

    #[test]
    fn basic_equal() {
        let state: State<u32> = State::new();
        let x = var();
        let goal = x.equals(5);
        let mut result = goal.run(state);
        assert_eq!(result.nth(0).unwrap().resolve_var(x).unwrap(), Can::Val(5));
    }
}
