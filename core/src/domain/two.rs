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
    use crate::goal::{all, assert_2, unify, Goal};
    use crate::tests::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var::<Vec<i32>>();
        let y = var::<i32>();
        let goal: Goal<OfTwo> = all(vec![
            unify(x, vec![1, 2, 3]),
            unify(y, 1),
            assert_2(x, y, |x: &Vec<i32>, y: &i32| x.contains(y)),
        ]);
        let result = util::goal_resolves_to(goal, (x, y));
        assert_eq!(result, vec![(vec![1, 2, 3], 1)]);
    }
    // TODO: Prove a few more use cases around connecting values of different
    // types.
    // - Specifically, how should we handle things like Vec<Val<T>>?
    // - Does this need to be somehow special cased in the macro?
    // - Or will it work (and is it clear/ergonomic enough) to just say if you
    //   want to be able to relate to values inside they need to be wrapped in a
    //   Val and have an equivalent elsewhere?
}
