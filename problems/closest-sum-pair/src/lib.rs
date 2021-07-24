//! # closest_sum_pair
//!
//! Finds two numbers in a `Vec<i32>`, that has the closest sum
//! to a given number. If there are multiple choices, pair that
//! has most distance between them is selected.
//!
//! The algorithm has time complexity of `O(NlogN)` and space complexity
//! of `O(1)`.
//!
//! # Quick Start
//! ```
//! use closest_sum_pair::Elements;
//!
//! let mut list: Vec<i32> = vec![-2, -4, -7, -2, -5, -13, -7];
//!
//! let len: usize = list.len();
//!
//! let desired_sum: i32 = -1;
//!
//! let mut elements = Elements::new(&mut list, len, desired_sum);
//!
//! let pair: (i32, i32) = elements
//!     .sort_list()
//!     .find_init_distance()
//!     .find_pair()
//!     .result();
//!
//! assert_eq!((-2, -2), pair);
//! ```


/// This struct holds 5 pieces of information related to the list.\
/// \
/// 1.`list`: A mutable ref to a `Vec<i32>.`\
/// \
/// 2.`len`: Length of the vector.\
/// \
/// 3.`desired_sum`: This is a value that users will provide. This is
/// the target value for the pairs.\
/// \
/// 4.`pair`: Pair that has the closest sum to `desired_sum`.\
/// \
/// 5.`init_distance`: a number that is always greater than
///  `sum of any two numbers in list - desired sum`.
pub struct Elements<'list> {
    list: &'list mut Vec<i32>,
    len: usize,
    desired_sum: i32,
    pair: Option<(i32, i32)>,
    init_distance: Option<i32>,
}

impl<'list> Elements<'list> {
    /// Returns an instance of `Elements`.
    ///
    /// # Examples
    ///
    /// ```
    /// use closest_sum_pair::Elements;
    ///
    /// let mut list: Vec<i32> = vec![3 ,5, 7];
    ///
    /// let len: usize = list.len();
    ///
    /// let desired_sum: i32 = 9;
    ///
    /// let elements = Elements::new(&mut list, len, desired_sum);
    /// ```
    pub fn new(list: &'list mut Vec<i32>, len: usize, desired_sum: i32) -> Self {
        Elements {
            list,
            len,
            desired_sum,
            pair: None,
            init_distance: None,
        }
    }

    /// Sorts the list in ascending order. We used rust's builtin sort
    /// method for this.
    pub fn sort_list(&mut self) -> &mut Self {
        self.list.sort();
        self
    }

    /// This method finds a number for `init_distance` field so that
    /// `init_distance ≥ | sum of any pair - desired sum |`\
    /// \
    /// The `find_pair` method tries to minimize the `distance` on
    /// every comparison. So if `init_distance` is big enough, it 
    /// will be replaced with a shorter distance value for *any pair*
    /// that comes first.
    pub fn find_init_distance(&mut self) -> &mut Self {
        let list: &Vec<i32> = self.list;

        let len: usize = self.len;

        let desired_sum: i32 = self.desired_sum;

        let highest_sum: i32 = list[len - 1] + list[len - 2];

        let lowest_sum: i32 = list[0] + list[1];

        let avg_sum: i32 = (highest_sum + lowest_sum) / 2;

        let distance: i32;

        if avg_sum - desired_sum <= 0 {
            distance = desired_sum - lowest_sum;
        } else {
            distance = highest_sum - desired_sum;
        }

        self.init_distance = Some(distance);

        self
    }

    /// This method runs a loop and tries to minimize the distance
    /// for every comparison between current distance and sum of
    /// two numbers in list. It always doesn't check all pairs,
    /// rather it breaks from the loop and returns from the function
    /// when `desired_sum` is found, otherwise it will check for every
    /// possibilities and return the closest sum.
    pub fn find_pair(&mut self) -> &Self {
        let list: &Vec<i32> = self.list;

        let len: usize = self.len;

        let desired_sum: i32 = self.desired_sum;

        let mut distance = self.init_distance.unwrap();

        let mut left_index: usize = 0;

        let mut right_index: usize = len - 1;

        for _ in 0..(len - 2) + 1 {
            let temp_sum: i32 = list[left_index] + list[right_index];

            let temp_distance: i32 = desired_sum - temp_sum;

            if temp_distance.abs() < distance.abs() {
                distance = temp_distance;
                self.pair = Some((list[left_index], list[right_index]));
            }

            if temp_distance > 0 {
                left_index += 1;
            } else if temp_distance < 0 {
                right_index -= 1;
            } else {
                break;
            }
        }

        self
    }

    /// We need to chain `sort_list`, `find_init_distance`, `find_pair`
    /// and `result` methods. The `result` method returns the pair.\
    ///
    /// # Examples
    /// ```
    /// use closest_sum_pair::Elements;
    ///
    /// let mut list: Vec<i32> = vec![-2, -4, -7, -2, -5, -13, -7];
    ///
    /// let len: usize = list.len();
    ///
    /// let desired_sum: i32 = -1;
    ///
    /// let mut elements = Elements::new(&mut list, len, desired_sum);
    ///
    /// let pair: (i32, i32) = elements
    ///     .sort_list()
    ///     .find_init_distance()
    ///     .find_pair()
    ///     .result();
    ///
    /// assert_eq!((-2, -2), pair);
    /// ```
    pub fn result(&self) -> (i32, i32) {
        let (first_val, second_val) = self.pair.unwrap();
        (first_val, second_val)
    }
}
