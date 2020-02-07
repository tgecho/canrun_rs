use canrun::{both, pair, var, Can, Equals, Goal, State};
extern crate env_logger;
#[test]
fn does_not_overflow() {
    let _ = env_logger::init();
    let x = var();
    let infinite_xs: Goal<usize> = x.equals(pair(x.can(), Can::Nil));
    // An overflow is not triggered if infinite_xs is the second argument
    let bad_goal = both(infinite_xs, x.equals(Can::Val(1)));
    let results: Vec<_> = bad_goal.run(State::new()).collect();
    // The goal should be invalidated early and not return a state
    assert_eq!(results, vec![]);
}
