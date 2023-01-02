#![allow(clippy::extra_unused_lifetimes)]

use canrun::goals::Goal;
use canrun::lvec::{member, LVec};
use canrun::{all, either, ltup, Value};
use canrun::{lvec, LVar, Query};

type LHouse = (
    Value<&'static str>,
    Value<&'static str>,
    Value<&'static str>,
    Value<&'static str>,
    Value<&'static str>,
);
type House = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
);

fn on_right<'a>(left: LHouse, right: LHouse, houses: &Value<LVec<LHouse>>) -> impl Goal {
    lvec::subset(lvec![left, right], houses)
}

fn next_to(a: LHouse, b: LHouse, houses: &Value<LVec<LHouse>>) -> impl Goal {
    either(
        on_right(a.clone(), b.clone(), houses),
        on_right(b, a, houses),
    )
}

pub fn zebra() -> Option<Vec<House>> {
    let houses: Value<LVec<LHouse>> = Value::new(lvec![
        ltup!(
            "norwegian",
            LVar::new(),
            LVar::new(),
            LVar::new(),
            LVar::new()
        ),
        LVar::new(),
        ltup!(LVar::new(), LVar::new(), "milk", LVar::new(), LVar::new()),
        LVar::new(),
        LVar::new(),
    ]);
    let goal = all![
        member(
            ltup!("englishman", LVar::new(), LVar::new(), LVar::new(), "red") as LHouse,
            &houses
        ),
        on_right(
            ltup!(LVar::new(), LVar::new(), LVar::new(), LVar::new(), "ivory"),
            ltup!(LVar::new(), LVar::new(), LVar::new(), LVar::new(), "green"),
            &houses
        ),
        next_to(
            ltup!(
                "norwegian",
                LVar::new(),
                LVar::new(),
                LVar::new(),
                LVar::new()
            ),
            ltup!(LVar::new(), LVar::new(), LVar::new(), LVar::new(), "blue"),
            &houses
        ),
        member(
            ltup!(LVar::new(), "kools", LVar::new(), LVar::new(), "yellow"),
            &houses
        ),
        member(
            ltup!("spaniard", LVar::new(), LVar::new(), "dog", LVar::new()),
            &houses
        ),
        member(
            ltup!(LVar::new(), LVar::new(), "coffee", LVar::new(), "green"),
            &houses
        ),
        member(
            ltup!("ukrainian", LVar::new(), "tea", LVar::new(), LVar::new()),
            &houses
        ),
        member(
            ltup!(LVar::new(), "luckystrikes", "oj", LVar::new(), LVar::new()),
            &houses
        ),
        member(
            ltup!(
                "japanese",
                "parliaments",
                LVar::new(),
                LVar::new(),
                LVar::new()
            ),
            &houses
        ),
        member(
            ltup!(LVar::new(), "oldgolds", LVar::new(), "snails", LVar::new()),
            &houses
        ),
        next_to(
            ltup!(LVar::new(), LVar::new(), LVar::new(), "horse", LVar::new()),
            ltup!(LVar::new(), "kools", LVar::new(), LVar::new(), LVar::new()),
            &houses
        ),
        next_to(
            ltup!(LVar::new(), LVar::new(), LVar::new(), "fox", LVar::new()),
            ltup!(
                LVar::new(),
                "chesterfields",
                LVar::new(),
                LVar::new(),
                LVar::new()
            ),
            &houses
        ),
        member(
            ltup!(LVar::new(), LVar::new(), "water", LVar::new(), LVar::new()),
            &houses
        ),
        member(
            ltup!(LVar::new(), LVar::new(), LVar::new(), "zebra", LVar::new()),
            &houses
        ),
    ];
    goal.query(houses).next()
}

#[test]
fn test_zebra() {
    assert_eq!(
        zebra(),
        Some(vec![
            ("norwegian", "kools", "water", "fox", "yellow"),
            ("ukrainian", "chesterfields", "tea", "horse", "blue"),
            ("englishman", "oldgolds", "milk", "snails", "red"),
            ("spaniard", "luckystrikes", "oj", "dog", "ivory"),
            ("japanese", "parliaments", "coffee", "zebra", "green"),
        ])
    )
}
