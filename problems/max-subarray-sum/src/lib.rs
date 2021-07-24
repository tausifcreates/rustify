//! # max_subarray_sum
//!
//! Finds maximum subarray sum in a `Vec<i32>`. This is also
//! known as max sum contigious subarray. If there are multiple
//! such subarrays, only the one that comes first is selected.
//!
//! The algorithm has time complexity of `O(N)` and space complexity
//! of `O(1)`.
//!
//! # Quick Start
//! ```
//! use max_subarray_sum::Elements;
//!
//! let mut list: Vec<i32> = vec![-2, -3, 4, -1, -2, 1, 5, -3];
//!
//! let len: usize = list.len();
//!
//! let mut elements = Elements::new(&mut list, len);
//!
//! let max_sum: i32 = elements.find_max_sum().result();
//!
//! assert_eq!(7, max_sum);
//! ```

/// The `Elements` struct holds the list and related pieces of informations
/// of it.
pub struct Elements<'list> {
    list: &'list Vec<i32>,
    len: usize,
    max_sum: Option<i32>,
}

impl<'list> Elements<'list> {
    /// Returns a new instance of `Elements`.
    ///
    /// # Examples
    ///
    /// ```
    /// use max_subarray_sum::Elements;
    ///
    /// let mut list: Vec<i32> = vec![-2, -3, 4, -1, -2, 1, 5, -3];
    ///
    /// let len: usize = list.len();
    ///
    /// let mut elements = Elements::new(&mut list, len);
    /// ```
    pub fn new(list: &'list Vec<i32>, len: usize) -> Self {
        Elements {
            list,
            len,
            max_sum: None,
        }
    }

    /// This method finds the max subarray sum.
    pub fn find_max_sum(&mut self) -> &Self {
        let list: &Vec<i32> = self.list;

        let len: usize = self.len;

        let mut temp_sum: i32 = list[0];

        let mut max_sum: i32 = temp_sum;

        for i in 1..len {
            if temp_sum > 0 {
                temp_sum += list[i];
            } else {
                temp_sum = list[i];
            }

            if temp_sum > max_sum {
                max_sum = temp_sum;
            }
        }

        self.max_sum = Some(max_sum);

        self
    }

    /// This method returns the max subarray sum. We need to chain
    /// this method with `find_max_sum`.
    ///
    /// # Examples
    /// ```
    /// use max_subarray_sum::Elements;
    ///
    /// let mut list: Vec<i32> = vec![-2, -3, 4, -1, -2, 1, 5, -3];
    ///
    /// let len: usize = list.len();
    ///
    /// let mut elements = Elements::new(&mut list, len);
    ///
    /// let max_sum: i32 = elements.find_max_sum().result();
    ///
    /// assert_eq!(7, max_sum);
    /// ```
    pub fn result(&self) -> i32 {
        let max_sum: i32 = self.max_sum.unwrap();
        max_sum
    }
}
