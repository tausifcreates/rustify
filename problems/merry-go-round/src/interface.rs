pub struct Elements<'list> {
    list: &'list [i32],
    len: usize,
    offset: u32,
}

impl<'list> Elements<'list> {
    pub fn new(list: &'list [i32], offset: u32) -> Self {
        let len: usize = list.len();

        Self { list, len, offset }
    }
}
