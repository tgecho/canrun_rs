# Quick Start Explained

First, we import some basic parts:
```rust
use canrun::{LVar, both, either, unify, Query};
```

Next, we create "logic variables" to represent the unresolved parts of the goals we're about to create.

```rust
# use canrun::{LVar, both, either, unify, Query};
let x = LVar::new();
let y = LVar::new();
# let goal = both(unify(x, y), unify(1, x));
```
Each [`LVar`](crate::LVar) is parameterized by the type it may be bound to, which is typically inferred but can be specified if needed (e.g. `LVar::<i32>::new()`).

With our variables prepped, we're ready to create our first goal.
```rust
# use canrun::{LVar, both, either, unify, Query};
# let x = LVar::new();
# let y = LVar::new();
let goal = both(unify(x, y), unify(1, x));
```

The outer [`both`](crate::goals::both) goal will succeed if both of its nested goals succeed. The other two [`unify`](crate::goals::unify) goals are essentially asserting that their two parameters are equal (in a sense... see the [more detailed attempt to explain unification](crate::Unify)). So if `x` and `y` are equal (and nothing in this goal precludes that assertion), and `x` and `1` are equal, then...

Now it's time to collect our results. We can do this with a [query](crate::Query):
```rust
# use canrun::{LVar, both, either, unify, Query};
# let x = LVar::new();
# let y = LVar::new();
# let goal = both(unify(x, y), unify(1, x));
let result: Vec<_> = goal.query(y).collect();
assert_eq!(result, vec![1])
```

In order to extract results out of a goal, we need to be looking for something specific and relevant. In this case, we're basically saying, "I made a few assertions about how `x`, `y` and `1` are related. Now, what are the possible ["reified"](crate::Reify) values of `y`? The result is collected into a `Vec` containing every possible value that unifies with the `y` logical variable. In this case: `vec![1]`.

We can also get multiple possible results. For example, what if we say that it could be [`either`](crate::goals::either) `1` or `2`?

```rust
# use canrun::{LVar, both, either, unify, Query};
# let x = LVar::new();
# let y = LVar::new();
let goal = both(
    unify(x, y),
    either(unify(1, x), unify(2, x))
);
let result: Vec<_> = goal.query(y).collect();
assert_eq!(result, vec![1, 2])
```

We can also query for multiple variables, given the relevent [`Reify`](crate::Reify) implementations (provided for tuples and a few basic std collections).


```rust
# use canrun::{LVar, both, either, unify, Query};
# let x = LVar::new();
# let y = LVar::new();
# let goal = both(
#     unify(x, y),
#     either(unify(1, x), unify(2, x))
# );
let result: Vec<_> = goal.query((x, y)).collect();
assert_eq!(result, vec![(1, 1), (2, 2)])
```

Note that the results will match the shape of the query.

## Next
- Read up on the available [goal functions](crate::goals)
- Learn more about how to [query for results](crate::Query)
- Implement [`Unify` for your own types](crate::Unify)
