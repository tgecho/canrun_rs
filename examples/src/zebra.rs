use canrun::{all, any, domain, either, unify, val, var, Goal, Val};
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

fn v() -> Val<&'static str> {
    Val::Var(var())
}

fn on_right<'a>(left: LHouse, right: LHouse, houses: LVec<LHouse>) -> Goal<'a, Zebra> {
    any![
        unify(
            lvec![left.clone(), right.clone(), var(), var(), var()],
            houses.clone()
        ),
        unify(
            lvec![var(), left.clone(), right.clone(), var(), var()],
            houses.clone()
        ),
        unify(
            lvec![var(), var(), left.clone(), right.clone(), var()],
            houses.clone()
        ),
        unify(lvec![var(), var(), var(), left, right], houses),
    ]
}

fn next_to(a: LHouse, b: LHouse, houses: LVec<LHouse>) -> Goal<'static, Zebra> {
    either(
        on_right(a.clone(), b.clone(), houses.clone()),
        on_right(b, a, houses),
    )
}

pub fn zebra() -> Vec<Vec<House>> {
    let houses: LVec<LHouse> = lvec![
        (val!("norwegian"), v(), v(), v(), v()),
        var(),
        (v(), v(), val!("milk"), v(), v()),
        var(),
        var(),
    ];
    let goal: Goal<Zebra> = all![
        member(
            (val!("englishman"), v(), v(), v(), val!("red")),
            houses.clone()
        ),
        on_right(
            (v(), v(), v(), v(), val!("ivory")),
            (v(), v(), v(), v(), val!("green")),
            houses.clone()
        ),
        next_to(
            (val!("norwegian"), v(), v(), v(), v()),
            (v(), v(), v(), v(), val!("blue")),
            houses.clone()
        ),
        member(
            (v(), val!("kools"), v(), v(), val!("yellow")),
            houses.clone()
        ),
        member(
            (val!("spaniard"), v(), v(), val!("dog"), v()),
            houses.clone()
        ),
        member(
            (v(), v(), val!("coffee"), v(), val!("green")),
            houses.clone()
        ),
        member(
            (val!("ukrainian"), v(), val!("tea"), v(), v()),
            houses.clone()
        ),
        member(
            (v(), val!("luckystrikes"), val!("oj"), v(), v()),
            houses.clone()
        ),
        member(
            (val!("japanese"), val!("parliaments"), v(), v(), v()),
            houses.clone()
        ),
        member(
            (v(), val!("oldgolds"), v(), val!("snails"), v()),
            houses.clone()
        ),
        next_to(
            (v(), v(), v(), val!("horse"), v()),
            (v(), val!("kools"), v(), v(), v()),
            houses.clone()
        ),
        next_to(
            (v(), v(), v(), val!("fox"), v()),
            (v(), val!("chesterfields"), v(), v(), v()),
            houses.clone()
        ),
        member((v(), v(), val!("water"), v(), v()), houses.clone()),
        member((v(), v(), v(), val!("zebra"), v()), houses.clone()),
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
