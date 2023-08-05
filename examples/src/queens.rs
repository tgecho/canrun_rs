use std::rc::Rc;

use canrun::{
    both,
    goals::{All, Any},
    lvec::{member, LVec},
    project::assert_2,
    unify, Goal, Query, Reify, StateIterator, Unify, Value,
};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Unify for Pos {
    fn unify(state: canrun::State, a: Rc<Self>, b: Rc<Self>) -> Option<canrun::State> {
        if a == b {
            Some(state)
        } else {
            None
        }
    }
}

impl Reify for Pos {
    type Reified = Pos;

    fn reify_in(&self, _state: &canrun::ReadyState) -> Result<Self::Reified, canrun::LVarList> {
        Ok(*self)
    }
}

pub fn n_queens(n: usize) -> Vec<Vec<Pos>> {
    let queens: Vec<Value<Pos>> = vec![Value::var(); n];
    let valid_positions = &Value::new(
        (0..n)
            .cartesian_product(0..n)
            .map(|(x, y)| dbg!(Pos { x, y }))
            .collect::<LVec<_>>(),
    );
    let positions: All = queens
        .iter()
        .map(|q| Box::new(member(q, valid_positions)) as Box<dyn Goal>)
        .collect();
    let restrictions: All = queens
        .iter()
        .tuple_combinations()
        .map(|(a, b)| Box::new(assert_2(a, b, not_attacking)) as Box<dyn Goal>)
        .collect();

    // let goals = both(positions, restrictions);
    positions
        .into_states()
        .map(|s| queens.reify_in(&s.ready().unwrap()).unwrap())
        .collect()
    // goals.query(queens).collect()
}

fn not_attacking(a: &Pos, b: &Pos) -> bool {
    dbg!((a, b));
    // down
    if a.y == b.y ||
        // right
        a.x == b.x ||
        // down and right
        a.x + b.x == a.y + b.y ||
        // down and left
        (a.x > b.x && b.y > a.y && a.x - b.x == b.y - a.y)
    {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::queens::{n_queens, not_attacking, Pos};

    #[test]
    #[rustfmt::skip]
    fn test_attacking() {
        assert!(not_attacking(&Pos{x:0, y:0}, &Pos{x:1, y:2}));
        assert!(!not_attacking( &Pos{x:0, y:0}, &Pos{x:0, y:2}));
        assert!(!not_attacking( &Pos{x:0, y:0}, &Pos{x:1, y:1}));
        assert!(!not_attacking( &Pos{x:0, y:0}, &Pos{x:2, y:0}));
        assert!(!not_attacking( &Pos{x:0, y:0}, &Pos{x:2, y:2}));
    }

    #[test]
    fn test_queens() {
        #[rustfmt::skip]
        assert_eq!(n_queens(4), vec![
            // vec![Pos{x:0, y:1}, Pos{x:1, y:3}, Pos{x:2, y:0}, Pos{x:3, y:2}],
            // vec![Pos{x:0, y:2}, Pos{x:1, y:0}, Pos{x:2, y:3}, Pos{x:3, y:1}],
        ] as Vec<Vec<Pos>>);
    }
}
