//! # closest_sum_pair
//!
//! Finds a pair from a vector of type `i32`, that has the closest sum
//! to a given number. If there are multiple choices, pair that has most
//! distance between them is selected.
//!
//! The algorithm has time complexity of `O(NlogN)` and space complexity
//! of `O(1)`.
//!
//! # Quick Start
//! ```
//! use closest_sum_pair::Elements;
//!
//! let mut list: Vec<i32> = vec![3 ,5, 7];
//!
//! let len: usize = list.len();
//!
//! let desired_sum: i32 = 9;
//!
//! let elements = Elements::new(&mut list, len, desired_sum);
//!
//! elements.sort_list().find_init_distance().find_pair().result();
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
/// 5.`init_distance`: Initialize the distance to a minimum number that
/// must be overwritten on the first iteration.

pub struct Elements<'list> {
    list: &'list mut Vec<i32>,
    len: usize,
    desired_sum: i32,
    pair: Option<(i32, i32)>,
    init_distance: Option<i32>,
}

impl<'list> Elements<'list> {
    /// Returns a new variable of type `Elements`.
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

    /// This function uses a simple algorithm to find the minimum
    /// distance from `desired_sum` that will always be greater
    /// than the distance between sum of any pairs in the vector
    /// and `desired_sum`.\
    /// \
    /// The reason for that is when comparing the sum of pair and
    /// its distance from the `desired_sum`, we need to overwrite
    /// the dummy value with what `distance` was initialized with
    /// actual distance from a pair's sum to `desired_sum`.\
    /// \
    /// This method overwrites the `init_distance` attribute of
    /// the struct.
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

    /// This method finds the pair, overwrites `pair` attribute 
    /// of the struct.
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

    /// Prints the pair and returns it. useful for testing.
    pub fn result(&self) -> (i32, i32) {
        let (first_val, second_val) = self.pair.unwrap();
        println!("First value: {},\nSecond value: {}", first_val, second_val);
        (first_val, second_val)
    }
}
