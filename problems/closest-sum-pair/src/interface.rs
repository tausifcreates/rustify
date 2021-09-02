pub struct Elements<'list> {
    list: &'list mut [i32],
    desired_sum: i32,
    pair: Option<(i32, i32)>,
    init_distance: Option<i32>,
}

impl<'list> Elements<'list> {
    pub fn new(list: &'list mut [i32], desired_sum: i32) -> Self {
        list.sort();
        Elements {
            list,
            desired_sum,
            pair: None,
            init_distance: None,
        }
    }

    // This method finds a number for `init_distance` field so that
    // `init_distance ≥ | sum of any pair - desired sum |`\
    pub fn init_distance(&mut self) -> &mut Self {
        let len: usize = self.list.len();

        let highest_sum: i32 = self.list[len - 1] + self.list[len - 2];

        let lowest_sum: i32 = self.list[0] + self.list[1];

        let avg_sum: i32 = (highest_sum + lowest_sum) / 2;

        if avg_sum - self.desired_sum <= 0 {
            self.init_distance = Some(self.desired_sum - lowest_sum);
        } else {
            self.init_distance = Some(highest_sum - self.desired_sum);
        }

        self
    }

    // This method runs a loop and tries to minimize the distance
    // for every comparison between current distance and sum of
    // two numbers in list.

    /// # Examples
    /// ```
    /// use closest_sum_pair::interface::Elements;
    ///
    /// let mut list: Vec<i32> = vec![-2, -4, -7, -2, -5, -13, -7];
    ///
    /// let len: usize = list.len();
    ///
    /// let desired_sum: i32 = -1;
    ///
    /// let mut elements = Elements::new(&mut list, desired_sum);
    ///
    /// let pair: (i32, i32) = elements.init_distance().find_pair();
    ///
    /// assert_eq!((-2, -2), pair);
    /// ```
    pub fn find_pair(&mut self) -> (i32, i32) {
        let len: usize = self.list.len();

        let mut distance = self.init_distance.unwrap();

        let mut left_index: usize = 0;

        let mut right_index: usize = len - 1;

        for _ in 0..(len - 2) + 1 {
            let temp_sum: i32 = self.list[left_index] + self.list[right_index];

            let temp_distance: i32 = self.desired_sum - temp_sum;

            if temp_distance.abs() < distance.abs() {
                distance = temp_distance;
                self.pair = Some((self.list[left_index], self.list[right_index]));
            }

            if temp_distance > 0 {
                left_index += 1;
            } else if temp_distance < 0 {
                right_index -= 1;
            } else {
                break;
            }
        }

        self.pair.unwrap()
    }
}
