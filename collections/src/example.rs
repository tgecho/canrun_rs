use crate::lmap::LMap;

canrun::domain! {
    pub LMapI32 {
        i32,
        LMap<i32, i32>,
    }
}
