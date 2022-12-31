[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/tgecho/canrun_rs/CI)](https://github.com/tgecho/canrun_rs/actions/workflows/tests.yml)
[![Coverage](https://img.shields.io/codecov/c/gh/tgecho/canrun_rs?token=7HSAMYDWEB)](https://codecov.io/gh/tgecho/canrun_rs)
[![Crate](https://img.shields.io/crates/v/canrun.svg)](https://crates.io/crates/canrun)
[![Documentation](https://docs.rs/canrun/badge.svg)](https://docs.rs/canrun/latest/canrun/)

Canrun is a [logic programming](https://en.wikipedia.org/wiki/Logic_programming)
library inspired by the [\*Kanren](http://minikanren.org/) family of language
DSLs.

- Intro blog post: [https://esimmler.com/announcing-canrun/](https://esimmler.com/announcing-canrun/)
- How it works (part 1): [https://esimmler.com/building-canrun-part-1/](https://esimmler.com/building-canrun-part-1/)

## Status: Exploratory and Highly Experimental

I'm still quite new to both Rust and logic programming, so there are likely to
be rough edges. At best it may be a useful implementation of something that
resembles the core concepts of a Kanren while being idiomatically Rusty. At
worst it may just be a poor misinterpretation with fatal flaws.

## Quick Start

```rust
use canrun2::{LVar, Query};
// TODO: reduce the goal module::function inception
use canrun2::goals::{both::both, unify::unify};

let x = LVar::new();
let y = LVar::new();
let goal = both(unify(x, y), unify(1, x));

let result: Vec<_> = goal.query(y).collect();

assert_eq!(result, vec![1])
```
