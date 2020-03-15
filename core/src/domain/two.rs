// The name "canrun" is not available from within the crate for testing.
// I think this workaround should work ~95% of the time. I guess it
// will fall down if someone renames the crate or something.
// https://github.com/rust-lang/rust/issues/54363
use crate as canrun;
use canrun_codegen::domains;

domains! {
    pub domain OfThree {
        i32,
        Vec<i32>,
        String,
    }
}

#[cfg(test)]
mod tests {
    use super::OfThree;
    use crate::goal::{all, project, unify, Goal};
    use crate::state::{State, Watch};
    use crate::tests::util;
    use crate::value::{var, Val};

    #[test]
    fn succeeds() {
        let x = var::<Vec<i32>>();
        let y = var::<i32>();
        let goal: Goal<OfThree> = all::<OfThree>(vec![
            unify(x, vec![1, 2, 3]),
            unify(y, 1),
            project(|s: State<OfThree>| {
                // This is pretty gnarly
                let x = Val::Var(x);
                let x = s.resolve_val(&x).resolved();
                let y = Val::Var(y);
                let y = s.resolve_val(&y).resolved();
                match (x, y) {
                    (Ok(x), Ok(y)) => Watch::done(if x.contains(y) { Some(s) } else { None }),
                    (Err(x), Err(y)) => Watch::watch(s, x).and(y),
                    (_, Err(y)) => Watch::watch(s, y),
                    (Err(x), _) => Watch::watch(s, x),
                }
            }) as Goal<OfThree>,
        ]);
        let result = util::goal_resolves_to(goal, (&x, &y));
        assert_eq!(result, vec![(vec![1, 2, 3], 1)]);
    }
}
