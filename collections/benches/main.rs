use criterion::{criterion_group, criterion_main};
mod goals;

criterion_group!(benches, goals::benches);
criterion_main!(benches);
