use canrun2::{both, lmap, unify, LVar, Query};
use criterion::{BenchmarkId, Criterion, Throughput};
use std::ops::Range;

fn range_lmap(range: Range<i32>) -> lmap::LMap<i32, i32> {
    range.fold(lmap::LMap::new(), |mut map, n| {
        map.insert(LVar::new(), n);
        map
    })
}

static BASE: i32 = 2;
static MAX_EXP: u32 = 7;

pub fn unify_lmaps(c: &mut Criterion) {
    let mut group = c.benchmark_group("unify_lmaps");
    group.sample_size(30);
    for size in (2..MAX_EXP).map(|n| BASE.pow(n)) {
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(
            BenchmarkId::new("matched/.nth(0)", size),
            &size,
            |bench, size| {
                bench.iter(|| {
                    let x = LVar::new();
                    let goal = both(
                        unify(&x, range_lmap(0..*size)),
                        unify(&x, range_lmap(0..*size)),
                    );
                    goal.query(x).next()
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("matched/.collect()", size),
            &size,
            |bench, size| {
                bench.iter(|| {
                    let x = LVar::new();
                    let goal = both(
                        unify(&x, range_lmap(0..*size)),
                        unify(&x, range_lmap(0..*size)),
                    );
                    let results: Vec<_> = goal.query(x).collect();
                    results
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("mismatched/.nth(0)", size),
            &size,
            |bench, size| {
                bench.iter(|| {
                    let x = LVar::new();
                    let goal = both(
                        unify(&x, range_lmap(0..*size)),
                        unify(&x, range_lmap(*size..(size + size))),
                    );
                    goal.query(x).next()
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("mismatched/.collect()", size),
            &size,
            |bench, size| {
                bench.iter(|| {
                    let x = LVar::new();
                    let goal = both(
                        unify(&x, range_lmap(0..*size)),
                        unify(&x, range_lmap(*size..(size + size))),
                    );
                    let results: Vec<_> = goal.query(x).collect();
                    results
                });
            },
        );
    }
    group.finish();
}
