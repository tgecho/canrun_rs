use canrun::{any, unify, var, Goal};
use canrun_collections::{example::LMapI32, lmap};
use criterion::Criterion;

macro_rules! goal_bench {
    ($c:ident $name:literal ($($v:ident),+) $goal:block) => {
        $c.bench_function(&format!("{} goal", $name), |b| {
            $(let $v = var();)+
            b.iter(|| $goal);
        });

        $c.bench_function(&format!("{} query", $name), |b| {
            $(let $v = var();)+
            b.iter(|| $goal.query(($($v),+)));
        });

        $c.bench_function(&format!("{} nth(0)", $name), |b| {
            $(let $v = var();)+
            b.iter(|| {
                $goal.query(($($v),+)).nth(0)
            });
        });

        $c.bench_function(&format!("{} collect()", $name), |b| {
            $(let $v = var();)+
            b.iter(|| {
                let results: Vec<_> = $goal.query(($($v),+)).collect();
                results
            });
        });
    }
}

pub fn benches(c: &mut Criterion) {
    goal_bench! {c "small_lmaps" (m) {
        let (w, x, y, z) = (var(), var(), var(), var());
        any![
            unify(m, lmap! {1 => x, 2 => w, y => x, 4 => x}),
            unify(m, lmap! {w => 2, x => 1, 3 => x, z => x}),
        ] as Goal<LMapI32>
    }}

    goal_bench! {c "bad_lmaps" (m) {
        let (w, x, y, z, a, b, c, d) = (var(), var(), var(), var(), var(), var(), var(), var());
        any![
            unify(m, lmap! {w => 0, x => 1, y => 2, z => 3, a => 4, b => 5, c => 6, d => 7}),
            unify(m, lmap! {0 => w, 2 => x, 2 => y, 3 => z, 4 => a, 5 => b, 6 => c, 7 => d}),
        ] as Goal<LMapI32>
    }}
}
