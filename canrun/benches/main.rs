use criterion::{criterion_group, criterion_main};
mod collections;
mod core;

criterion_group!(benches, core::benches, collections::unify_lmaps);
criterion_main!(benches);
