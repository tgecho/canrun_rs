//! Basic domains for simple use cases.
//!
//! | Domain     | Types |
//! | ------     | ----- |
//! | `I32`      | `i32` |
//! | `TupleI32` | `i32`, `(Val<i32>, Val<i32>)` |

// Figure out how to get the macro to generate docs with these types listed out.

use crate::lmap::LMap;
use crate::lvec::LVec;
use crate::value::Val;

canrun_codegen::canrun_internal_domain! {
    pub I32 { i32 }
}
canrun_codegen::canrun_internal_domain! {
    pub TupleI32 {
        i32,
        (Val<i32>, Val<i32>),
    }
}
canrun_codegen::canrun_internal_domain! {
    pub Collections {
        i32,
        LMap<i32, i32>,
        LVec<i32>
    }
}
