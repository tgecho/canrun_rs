canrun_codegen::domain! {
    OfTwo
    i32,
    Vec<i32>,
    String,
}

#[cfg(test)]
mod tests {
    use super::OfTwo;
    use crate::goal::{all, project, unify, Goal};
    use crate::state::{State, Watch};
    use crate::tests::util;
    use crate::value::{var, Val};

    #[test]
    fn succeeds() {
        let x = var::<Vec<i32>>();
        let y = var::<i32>();
        let goal: Goal<OfTwo> = all::<OfTwo>(vec![
            unify(x, vec![1, 2, 3]),
            unify(y, 1),
            project(|s: State<OfTwo>| {
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
            }) as Goal<OfTwo>,
        ]);
        let result = util::goal_resolves_to(goal, (&x, &y));
        assert_eq!(result, vec![(vec![1, 2, 3], 1)]);
    }
}
