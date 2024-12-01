use canrun::{all, any, either, unify, LVar, Query};
use criterion::Criterion;

macro_rules! goal_bench {
    ($c:ident $name:literal ($($v:ident),+) $goal:block) => {
        $c.bench_function(&format!("{} goal", $name), |b| {
            $(let $v = LVar::new();)+
            b.iter(|| $goal);
        });

        $c.bench_function(&format!("{} query", $name), |b| {
            $(let $v = LVar::new();)+
            b.iter(|| $goal.query(($($v),+)));
        });

        $c.bench_function(&format!("{} nth(0)", $name), |b| {
            $(let $v = LVar::new();)+
            b.iter(|| {
                $goal.query(($($v),+)).nth(0)
            });
        });

        $c.bench_function(&format!("{} collect()", $name), |b| {
            $(let $v = LVar::new();)+
            b.iter(|| {
                let results: Vec<_> = $goal.query(($($v),+)).collect();
                results
            });
        });
    }
}

#[allow(clippy::too_many_lines)]
pub fn benches(c: &mut Criterion) {
    goal_bench! {c "one" (x) {
        unify(x, 1)
    }}

    goal_bench! {c "two" (x, y) {
        all![unify(x, 1), unify(y, 1), unify(y, x)]
    }}

    goal_bench! {c "three" (x, y, z) {
        all![
            unify(x, 1),
            unify(y, 1),
            unify(z, 1),
            unify(y, x),
            unify(x, z),
        ]
    }}

    goal_bench! {c "forking" (a, b, c, d, e) {
        all![
            unify(a, 1),
            unify(b, 1),
            unify(c, 1),
            unify(b, a),
            unify(a, c),
            either(unify(d, c), unify(d, e)),
            unify(e, 2),
            either(unify(d, e), unify(a, e)),
        ]
    }}

    goal_bench! {c "fail_after_forking" (a, b, c, d, e) {
        all![
            unify(a, 1),
            unify(b, 1),
            unify(c, 1),
            unify(b, a),
            unify(a, c),
            either(unify(d, c), unify(d, e)),
            unify(e, 2),
            either(unify(a, 2), unify(b, 3)),
        ]
    }}

    goal_bench! {c "fail_before_forking" (a, b, c, d, e) {
        all![
            unify(a, 1),
            unify(b, 1),
            unify(c, 1),
            unify(b, a),
            unify(a, c),
            either(unify(d, c), unify(d, e)),
            unify(e, 2),
            unify(e, a),
            either(unify(d, e), unify(a, e)),
        ]
    }}

    goal_bench! {c "unify_a_chain" (k) {
        {
            let (a, b, c, d, e, f, g, h, i, j) =
                (LVar::new(), LVar::new(), LVar::new(), LVar::new(), LVar::new(), LVar::new(), LVar::new(), LVar::new(), LVar::new(), LVar::new());
            all![
                unify(1, a),
                unify(a, b),
                unify(b, c),
                unify(c, d),
                unify(d, e),
                unify(e, f),
                unify(f, g),
                unify(g, h),
                unify(h, i),
                unify(i, j),
                unify(j, k),
            ]
        }
    }}

    goal_bench! {c "unify_a_repetitive" (k) {
        {
            let (a, b, c, d, e, f, g, h, i, j) =
                (LVar::new(), LVar::new(), LVar::new(), LVar::new(), LVar::new(), LVar::new(), LVar::new(), LVar::new(), LVar::new(), LVar::new());
            all![
                unify(a, 1),
                unify(b, 1),
                unify(c, 1),
                unify(d, 1),
                unify(e, 1),
                unify(f, 1),
                unify(g, 1),
                unify(h, 1),
                unify(i, 1),
                unify(j, 1),
                unify(k, 1),
            ]
        }
    }}

    goal_bench! {c "wide_forking" (a) {
        any![
            unify(a, 1),
            unify(a, 2),
            unify(a, 3),
            unify(a, 4),
            unify(a, 5),
            unify(a, 6),
            unify(a, 7),
            unify(a, 8),
            unify(a, 9),
            unify(a, 10),
            unify(a, 11),
        ]
    }}
}
