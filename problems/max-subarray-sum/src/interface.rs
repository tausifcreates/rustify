/// The `Elements` struct holds the list and related pieces of informations
/// of it.
pub struct Elements<'list> {
    pub list: &'list [i32],
    pub len: usize,
    pub max_sum: Option<i32>,
}