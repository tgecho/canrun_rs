Canrun is a [logic programming](https://en.wikipedia.org/wiki/Logic_programming)
library inspired by the [\*Kanren](http://minikanren.org/) family of language
DSLs.

## Status: Exploratory and Highly Experimental

I'm still quite new to both Rust and logic programming, so there are likely to
be rough edges. At best the goal is to be a useful implementation of the core
concepts of a Kanren in way that is idiomatic to Rust. At worst it may just be a
poor misinterpretation with fatal flaws.

## Quick Start

```rust
use canrun::{Goal, both, unify, var};
use canrun::domains::example::I32;

let x = var();
let y = var();
let goal: Goal<I32> = both(unify(x, y), unify(1, x));
let result: Vec<_> = goal.query(y).collect();
assert_eq!(result, vec![1])
```

## Concepts

- [Domains](crate::domains) constrain the types of values that you can reason about
  in a particular context.
- [Values](crate::value) are either resolved or [LVars](crate::value::LVar) that
  can be bound to other values through unification.
- [Goals](crate::goal) contain declarative assertions about the relationships
  between values.
- [States](crate::state) track value bindings and constraints during evaluation
  of a logic program.
- [Queries](crate::query) allow easy extraction of resolved values.
