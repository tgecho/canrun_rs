use canrun::{both, equal, pair, Can, LVar, State};

#[test]
fn does_not_overflow() {
    let x = LVar::new();
    let infinite_xs = equal(x.into(), pair(x.into(), Can::Nil));
    // An overflow is not triggered if infinite_xs is the second argument
    let bad_goal = both(infinite_xs, equal(x.into(), Can::Val(1)));
    let results: Vec<_> = bad_goal.run(&State::new()).collect();
    // The goal should be invalidated early and not return a state
    assert_eq!(results, vec![]);
}
