#[macro_use]
extern crate criterion;

use criterion::Criterion;
mod goals;

criterion_group!(benches, goals::benches);
criterion_main!(benches);
