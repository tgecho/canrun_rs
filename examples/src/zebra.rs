use canrun::{all, any, domain, either, ltup, unify, var, Goal, Val};
use canrun_collections::{lvec, lvec::member, LVec};

type LHouse = (
    Val<&'static str>,
    Val<&'static str>,
    Val<&'static str>,
    Val<&'static str>,
    Val<&'static str>,
);
type House = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
);

domain! {
    Zebra {
        &'static str,
        LHouse,
        LVec<LHouse>,
    }
}

fn on_right<'a>(left: &LHouse, right: &LHouse, houses: &LVec<LHouse>) -> Goal<'a, Zebra> {
    any![
        unify(lvec![left, right, var(), var(), var()], houses),
        unify(lvec![var(), left, right, var(), var()], houses),
        unify(lvec![var(), var(), left, right, var()], houses),
        unify(lvec![var(), var(), var(), left, right], houses),
    ]
}

fn next_to(a: &LHouse, b: &LHouse, houses: &LVec<LHouse>) -> Goal<'static, Zebra> {
    either(on_right(a, b, houses), on_right(b, a, houses))
}

pub fn zebra() -> Vec<Vec<House>> {
    let houses: LVec<LHouse> = lvec![
        ltup!("norwegian", var(), var(), var(), var()),
        var(),
        ltup!(var(), var(), "milk", var(), var()),
        var(),
        var(),
    ];
    let goal: Goal<Zebra> = all![
        member(ltup!("englishman", var(), var(), var(), "red"), &houses),
        on_right(
            &ltup!(var(), var(), var(), var(), "ivory"),
            &ltup!(var(), var(), var(), var(), "green"),
            &houses
        ),
        next_to(
            &ltup!("norwegian", var(), var(), var(), var()),
            &ltup!(var(), var(), var(), var(), "blue"),
            &houses
        ),
        member(ltup!(var(), "kools", var(), var(), "yellow"), &houses),
        member(ltup!("spaniard", var(), var(), "dog", var()), &houses),
        member(ltup!(var(), var(), "coffee", var(), "green"), &houses),
        member(ltup!("ukrainian", var(), "tea", var(), var()), &houses),
        member(ltup!(var(), "luckystrikes", "oj", var(), var()), &houses),
        member(
            ltup!("japanese", "parliaments", var(), var(), var()),
            &houses
        ),
        member(ltup!(var(), "oldgolds", var(), "snails", var()), &houses),
        next_to(
            &ltup!(var(), var(), var(), "horse", var()),
            &ltup!(var(), "kools", var(), var(), var()),
            &houses
        ),
        next_to(
            &ltup!(var(), var(), var(), "fox", var()),
            &ltup!(var(), "chesterfields", var(), var(), var()),
            &houses
        ),
        member(ltup!(var(), var(), "water", var(), var()), &houses),
        member(ltup!(var(), var(), var(), "zebra", var()), &houses),
    ];
    goal.query(houses).collect()
}

#[test]
fn test_zebra() {
    assert_eq!(
        zebra(),
        vec![vec![
            ("norwegian", "kools", "water", "fox", "yellow"),
            ("ukrainian", "chesterfields", "tea", "horse", "blue"),
            ("englishman", "oldgolds", "milk", "snails", "red"),
            ("spaniard", "luckystrikes", "oj", "dog", "ivory"),
            ("japanese", "parliaments", "coffee", "zebra", "green"),
        ]]
    )
}
