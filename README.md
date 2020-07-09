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
use canrun::{Goal, both, unify, var};
use canrun::example::I32;

let x = var();
let y = var();
let goal: Goal<I32> = both(unify(x, y), unify(1, x));
let result: Vec<_> = goal.query(y).collect();
assert_eq!(result, vec![1])
```
