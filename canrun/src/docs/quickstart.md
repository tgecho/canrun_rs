# Quick Start Explained

First, we import some basic parts:
```rust
use canrun::{domain, var, Goal, both, unify};
```

In order to allow compile time type checking of our goals, we need to create a domain that defines the possible types of values we will use. I've tried to make this as streamlined as possible, but I'm up for ideas to eliminate the need!
```rust
# use canrun::domain;
domain! {
    pub I32 { i32 }
}
```

You can create multiple domains, and each domain can contain multiple types (e.g. `pub Foo { i32, String }`).

Next we create "logical variables" to represent the unresolved parts of the goals we're about to create.

```rust
# use canrun::{domain, var, Goal, both, unify};
# domain! { pub I32 { i32 } }
let x = var();
let y = var();
# let goal: Goal<I32> = both(unify(x, y), unify(1, x));
```
Each [`LVar`](crate::value::LVar) has a parameter type, which is typically inferred but can be specified if needed (e.g. `var::<i32>()`).

With our variables prepped, we're ready to create our goal.
```rust
# use canrun::{domain, var, Goal, both, unify};
# domain! { pub I32 { i32 } }
# let x = var();
# let y = var();
let goal: Goal<I32> = both(unify(x, y), unify(1, x));
```

First, note that the [`Goal`](crate::goals) is parameterized by the domain we created earlier. Any variables or values inside this goal must match up with the types in the associated domain.

The outer [`both`](crate::goals::both) goal will succeed if both of its nested goals succeed. The other two [`unify`](crate::goals::unify)goals are essentially asserting that their two parameters are equal (in a sense... see the [more detailed attempt to explain unification](crate::UnifyIn)). So if `x` and `y` are equal (and nothing in this goal precludes that assertion), and `x` and `1` are equal, then...

Now it's time to collect our results. We do this with a [query](crate::Query):
```rust
# use canrun::{domain, var, Goal, both, unify};
# domain! { pub I32 { i32 } }
# let x = var();
# let y = var();
# let goal: Goal<I32> = both(unify(x, y), unify(1, x));
let result: Vec<_> = goal.query(y).collect();
assert_eq!(result, vec![1])
```

In order to extract results out of a goal, we need to be looking for something specific and relevant. In this case, we're basically saying, "I made a few assertions about how `x`, `y` and `1` are related. What are the possible ["reified"](crate::ReifyIn) values of `y`. The result is collected into a `Vec` containing every possible value that unifies with the `y` logical variable. In this case: `1`.

We can also get multiple possible results. For example, what if we say that it could be [`either`](crate::goals::either) `1` or `2`?

```rust
# use canrun::{domain, var, Goal, both, unify, either};
# domain! { pub I32 { i32 } }
# let x = var();
# let y = var();
let goal: Goal<I32> = both(
    unify(x, y),
    either(unify(1, x), unify(2, x))
);
let result: Vec<_> = goal.query(y).collect();
assert_eq!(result, vec![1, 2])
```

We can also query for multiple variables, given the relevent [`ReifyIn`](crate::ReifyIn) implementations (provided for tuples and a few basic std collections).


```rust
# use canrun::{domain, var, Goal, both, unify, either};
# domain! { pub I32 { i32 } }
# let x = var();
# let y = var();
# let goal: Goal<I32> = both(
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
