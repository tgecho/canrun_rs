//! A port of the puzzle from
//! <https://gist.github.com/Spuffynism/446c7c2d498477491d8137e8f234d4a9>
//!
//! This is based on the "nicer" implentation, where we (mostly) just need to
//! define the `lt` and `gt relationships between the reindeer.
use canrun::{
    all,
    cmp::{gt, lt},
    ltup, lvec,
    lvec::{member, LVec},
    LVar, Query, Value,
};

pub fn order_reindeer() -> Vec<(&'static str, i32)> {
    // Create a variable to hold the position of each reindeer
    let vixen = LVar::new();
    let rudolph = LVar::new();
    let prancer = LVar::new();
    let dasher = LVar::new();
    let dancer = LVar::new();
    let comet = LVar::new();
    let donder = LVar::new();
    let blitzen = LVar::new();
    let cupid = LVar::new();

    // An LVec of all the possible positions
    let positions = Value::new(LVec::from_iter(1..=9));

    let ordering = all![
        // Each reindeer has a position
        member(vixen, &positions),
        member(rudolph, &positions),
        member(prancer, &positions),
        member(dasher, &positions),
        member(dancer, &positions),
        member(comet, &positions),
        member(donder, &positions),
        member(blitzen, &positions),
        member(cupid, &positions),
        // Vixen should be behind Rudolph, Prancer and Dasher,
        gt(vixen, rudolph),
        gt(vixen, prancer),
        gt(vixen, dasher),
        // whilst Vixen should be in front of Dancer and Comet.
        lt(vixen, dancer),
        lt(vixen, comet),
        // Dancer should be behind Donder, Blitzen and Rudolph.
        gt(dancer, donder),
        gt(dancer, blitzen),
        gt(dancer, comet),
        // Comet should be behind Cupid, Prancer and Rudolph.
        gt(comet, cupid),
        gt(comet, prancer),
        gt(comet, rudolph),
        // Donder should be behind Comet, Vixen, Dasher, Prancer and Cupid.
        gt(donder, comet),
        gt(donder, vixen),
        gt(donder, dasher),
        gt(donder, prancer),
        gt(donder, cupid),
        // Cupid should be in front of Comet, Blitzen, Vixen, Dancer and Rudolph.
        lt(cupid, comet),
        lt(cupid, blitzen),
        lt(cupid, vixen),
        lt(cupid, dancer),
        lt(cupid, rudolph),
        // Prancer should be in front of Blitzen, Donder and Cupid.
        lt(prancer, blitzen),
        lt(prancer, donder),
        lt(prancer, cupid),
        // Blitzen should be behind Cupid but in front of Dancer, Vixen and Donder.
        gt(blitzen, cupid),
        lt(blitzen, dancer),
        lt(blitzen, vixen),
        lt(blitzen, donder),
        // Rudolph should be behind Prancer but in front of Dasher, Dancer and Donder.
        gt(rudolph, prancer),
        lt(rudolph, dasher),
        lt(rudolph, dancer),
        lt(rudolph, donder),
        // Finally, Dasher should be behind Prancer but in front of Blitzen, Dancer and Vixen.
        gt(dasher, prancer),
        lt(dasher, blitzen),
        lt(dasher, dancer),
        lt(dasher, vixen)
    ];

    // A quick and dirty way to get a readable way to interprate the results.
    // There are probably more elegant approaches, but this works. It shows
    // creating a few logic structures with placeholders and using query to fill
    // them in.
    ordering
        .query(lvec![
            ltup!("vixen", vixen),
            ltup!("rudolph", rudolph),
            ltup!("prancer", prancer),
            ltup!("dasher", dasher),
            ltup!("dancer", dancer),
            ltup!("comet", comet),
            ltup!("donder", donder),
            ltup!("blitzen", blitzen),
            ltup!("cupid", cupid),
        ])
        .take(1)
        // Then we just use straight rust to sort the results
        .flat_map(|mut v| {
            v.sort_by_key(|r| r.1);
            v.into_iter()
        })
        .collect()
}

#[test]
fn test_reindeer() {
    assert_eq!(
        order_reindeer(),
        vec![
            ("prancer", 1),
            ("cupid", 2),
            ("rudolph", 3),
            ("dasher", 4),
            ("blitzen", 5),
            ("vixen", 6),
            ("comet", 7),
            ("donder", 8),
            ("dancer", 9),
        ]
    )
}
