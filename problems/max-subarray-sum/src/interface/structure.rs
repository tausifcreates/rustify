pub mod elements {
    pub struct Elements<'list> {
        pub list: &'list [i32],
        pub len: usize,
        pub max_sum: Option<i32>,
    }
}
