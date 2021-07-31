pub use super::interface::Elements;

impl<'list> Elements<'list> {
    /// Returns a new instance of struct `Elements`.
    pub fn new(list: &'list [i32]) -> Self {
        Elements {
            list,
            len: list.len(),
            max_sum: None,
        }
    }

    /// This method finds the max subarray sum. If there are multiple
    /// subarrays with equal sum it selects the subarray that came first.
    pub fn find_max_sum(&mut self) -> &Self {
        let list: &[i32] = self.list;

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
    /// use max_subarray_sum::interface::Elements;
    ///
    /// let mut list = vec![-2, -3, 4, -1, -2, 1, 5, -3];
    ///
    /// let mut elements = Elements::new(&mut list);
    ///
    /// let max_sum = elements.find_max_sum().result();
    ///
    /// assert_eq!(7, max_sum);
    /// ```
    pub fn result(&self) -> i32 {
        self.max_sum.unwrap()
    }
}
