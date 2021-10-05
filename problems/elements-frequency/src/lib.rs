/// Examples
/// ```
/// use elements_frequency::interface::frequency_finder;
/// 
/// fn main () {
///     let myList = [1, 1, -6, 2, 6, 2, 7, 1];
/// 
///     let myThreads = 6;
/// 
///     let frequency_map = frequency_finder(&myList, myThreads);
/// 
///     println!("{:?}", frequency_map);
///     
///     // Output:
///     // {6: 1, -6: 1, 2: 2, 7: 1, 1: 3}
/// }
/// 
/// ```
pub mod interface;
