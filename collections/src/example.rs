//! Example domains for Canrun collections
use crate::lmap::LMap;
use crate::lvec::LVec;

canrun::domain! {
    pub Collections {
        i32,
        LMap<i32, i32>,
        LVec<i32>
    }
}
