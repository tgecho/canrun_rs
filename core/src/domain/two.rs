// The name "canrun" is not available from within the crate for testing.
// I think this workaround should work ~95% of the time. I guess it
// will fall down if someone renames the crate or something.
// https://github.com/rust-lang/rust/issues/54363
use crate as canrun;
use canrun_codegen::domains;

domains! {
    pub domain OfTwo {
        i32,
        Vec<i32>,
    }
}

#[cfg(test)]
mod tests {
    use super::OfTwo;
    use crate::goal::{all, project, unify, Goal};
    use crate::state::{State, Watch};
    use crate::tests::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var::<Vec<i32>>();
        let y = var::<i32>();
        let goal: Goal<OfTwo> = all::<OfTwo>(vec![
            unify(x, vec![1, 2, 3]),
            unify(y, 1),
            project(|s: State<OfTwo>| {
                // TODO: Figure out a more ergonomic way to define watch and/or projection functions.
                // - This may just be to provide the 1/2/3 var versions we had in the original PoC
                // - The query system may be useful here, though we may want to add the ability to NOT clone.

                match (s.get(x), s.get(y)) {
                    (Ok(x), Ok(y)) => Watch::done(if x.contains(y) { Some(s) } else { None }),
                    (Err(x), Err(y)) => Watch::watch(s, x).and(y),
                    (_, Err(y)) => Watch::watch(s, y),
                    (Err(x), _) => Watch::watch(s, x),
                }
            }) as Goal<OfTwo>,
        ]);
        let result = util::goal_resolves_to(goal, (x, y));
        assert_eq!(result, vec![(vec![1, 2, 3], 1)]);
    }
    // TODO: Prove a few more use cases around connecting values of different types.
    // - Specifically, how should we handle things like Vec<Val<T>>?
    // - Does this need to be somehow special cased in the macro?
    // - Or will it work (and is it clear/ergonomic enough) to just say if you want to be able to relate to values inside they need to be wrapped in a Val and have an equivilent elsewhere?
}
