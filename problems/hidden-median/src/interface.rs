use std::cmp::{max, min};

#[derive(Debug)]
pub struct Holder<'list> {
    list_a: &'list [i32],
    list_b: &'list [i32],
    len_a: usize,
    len_b: usize,
    median: Option<(i32, i32)>,
}

impl<'list> Holder<'list> {
    pub fn new(list_a: &'list [i32], list_b: &'list [i32]) -> Self {
        let mut len_a: usize = list_a.len();
        let mut len_b: usize = list_b.len();

        if len_a > len_b {
            if (len_a + len_b) % 2 != 0 {
                len_a -= 1;
            }

            return Self {
                list_a: list_b,
                list_b: list_a,
                len_a: len_b,
                len_b: len_a,
                median: None,
            };
        } else {
            if (len_a + len_b) % 2 != 0 {
                len_b -= 1;
            }

            return Self {
                list_a,
                list_b,
                len_a,
                len_b,
                median: None,
            };
        }
    }

    pub fn run(&mut self, mut left_boundary_a: usize, mut right_boundary_a: usize) {
        let list_a: &[i32] = self.list_a;
        let list_b: &[i32] = self.list_b;
        let len_a: usize = self.len_a;
        let len_b: usize = self.len_b;

        let turns: u32 = (len_a as f32).log(2.0) as u32;

        for _ in 1..turns {
            let divider_a_left: usize = (right_boundary_a + left_boundary_a) / 2;
            let divider_b_left: usize = (len_a + len_b) / 2 - (divider_a_left + 1) - 1;

            let divider_a_right: usize = divider_a_left + 1;
            let divider_b_right: usize = divider_b_left + 1;

            if list_a[divider_a_left] > list_b[divider_b_right] {
                right_boundary_a = divider_a_left;
            } else if list_b[divider_b_left] > list_a[divider_a_right] {
                left_boundary_a = divider_a_left;
            } else {
                let median_left: i32 = max(list_a[divider_a_left], list_b[divider_b_left]);
                let median_right: i32 = min(list_a[divider_a_right], list_b[divider_b_right]);
                self.median = Some((median_left, median_right));
                return;
            }
        }

        if right_boundary_a - left_boundary_a == 2 {
            let divider_a_left: usize = (right_boundary_a + left_boundary_a) / 2;
            let divider_b_left: usize = (len_a + len_b) / 2 - (divider_a_left + 1) - 1;

            let divider_a_right: usize = divider_a_left + 1;
            let divider_b_right: usize = divider_b_left + 1;

            if list_a[divider_a_left] > list_b[divider_b_right] {
                right_boundary_a = divider_a_left;
            } else if list_b[divider_b_left] > list_a[divider_a_right] {
                left_boundary_a = divider_a_left;
            } else {
                let median_left: i32 = max(list_a[divider_a_left], list_b[divider_b_left]);
                let median_right: i32 = min(list_a[divider_a_right], list_b[divider_b_right]);
                self.median = Some((median_left, median_right));
                return;
            }
        }

        if right_boundary_a == 1 {
            let divider_a_left: usize = 0;
            let mut divider_a_right: usize = 1;

            let mut divider_b_left: usize = (len_a + len_b) / 2 - 2;
            let mut divider_b_right: usize = divider_b_left + 1;

            if list_a[divider_a_left] <= list_b[divider_b_right]
                && list_b[divider_b_left] <= list_a[divider_a_right]
            {
                let median_left: i32 = max(list_a[divider_a_left], list_b[divider_b_left]);
                let median_right: i32 = min(list_a[divider_a_right], list_b[divider_b_right]);
                self.median = Some((median_left, median_right));
                return;
            } else {
                divider_a_right = 0;
                divider_b_left += 1;

                if len_a == len_b {
                    let median_left: i32 = list_b[divider_b_left];
                    let median_right: i32 = list_a[divider_a_right];
                    self.median = Some((median_left, median_right));
                    return;
                } else {
                    divider_b_right = divider_b_left + 1;
                    let median_left: i32 = list_b[divider_b_left];
                    let median_right: i32 = min(list_a[divider_a_right], list_b[divider_b_right]);
                    self.median = Some((median_left, median_right));
                    return;
                }
            }
        }

        if left_boundary_a == len_a - 2 {
            let mut divider_a_left: usize = len_a - 2;
            let divider_a_right: usize = len_a - 1;

            let mut divider_b_left: usize = (len_a + len_b) / 2 - (divider_a_left + 1) - 1;
            let mut divider_b_right: usize = divider_b_left + 1;

            if list_a[divider_a_left] <= list_b[divider_b_right]
                && list_b[divider_b_left] <= list_a[divider_a_right]
            {
                let median_left: i32 = max(list_a[divider_a_left], list_b[divider_b_left]);
                let median_right: i32 = min(list_a[divider_a_right], list_b[divider_b_right]);
                self.median = Some((median_left, median_right));
                return;
            } else {
                divider_a_left = len_a - 1;
                divider_b_right -= 1;

                if len_a == len_b {
                    let median_left: i32 = list_a[divider_a_left];
                    let median_right: i32 = list_b[divider_b_right];
                    self.median = Some((median_left, median_right));
                    return;
                } else {
                    divider_b_left = divider_b_right - 1;
                    let median_left: i32 = max(list_a[divider_a_left], list_b[divider_b_left]);
                    let median_right: i32 = list_b[divider_b_right];
                    self.median = Some((median_left, median_right));
                    return;
                }
            }
        }
    }

    pub fn init(&mut self) -> &Self {
        let list_a: &[i32] = self.list_a;
        let list_b: &[i32] = self.list_b;
        let len_a: usize = self.len_a;
        let len_b: usize = self.len_b;
        let left_boundary_a: usize = 0;
        let right_boundary_a: usize = len_a - 1;

        if len_a == 1 {
            if len_b == 1 {
                if list_a[0] <= list_b[0] {
                    self.median = Some((list_a[0], list_b[0]));
                } else {
                    self.median = Some((list_b[0], list_a[0]));
                }
            } else {
                let median_b_idx: usize = len_b / 2;

                let mut make_median: [i32; 4] = [0; 4];

                make_median[0] = list_a[0];
                make_median[1] = list_b[median_b_idx - 1];
                make_median[2] = list_b[median_b_idx];
                make_median[3] = list_b[median_b_idx + 1];

                make_median.sort();

                self.median = Some((make_median[1], make_median[2]));
            }
        } else if len_a == 2 {
            if len_b == 2 {
                let midean_left: i32 = max(list_a[0], list_b[0]);
                let midean_right: i32 = min(list_a[1], list_b[1]);

                self.median = Some((midean_left, midean_right));
            } else {
                let median_left_b_idx: usize = (len_b / 2) - 1;
                let median_right_b_idx: usize = median_left_b_idx + 1;

                let mut make_median: [i32; 6] = [0; 6];

                make_median[0] = list_a[0];
                make_median[1] = list_a[1];
                make_median[2] = list_b[median_left_b_idx - 1];
                make_median[3] = list_b[median_left_b_idx];
                make_median[4] = list_b[median_right_b_idx];
                make_median[5] = list_b[median_right_b_idx + 1];

                make_median.sort();

                self.median = Some((make_median[2], make_median[3]));
            }
        } else {
            self.run(left_boundary_a, right_boundary_a);
        }

        if self.list_b.len() != self.len_b {
            let list_b: &[i32] = self.list_b;
            let len_b: usize = list_b.len();
            let unpaired_elem: i32 = list_b[len_b - 1];

            let median_left: i32 = self.median.unwrap().0;
            let median_right: i32 = self.median.unwrap().1;

            if unpaired_elem <= median_left {
                self.median = Some((median_left, median_left));
            } else if unpaired_elem >= median_right {
                self.median = Some((median_right, median_right));
            } else {
                self.median = Some((unpaired_elem, unpaired_elem))
            }
        }

        self
    }
    /// # Examples
    ///
    /// ```
    /// use hidden_median::interface::Holder;
    /// 
    /// fn main() {
    ///     let list_a = [-3, -1, 5, 6];
    ///     let list_b = [-7, -2, 4, 8, 11];
    ///
    ///     let mut holder = Holder::new(&list_a, &list_b);
    ///
    ///     let result = holder.init().result();
    ///     
    ///     // `result` is a tuple, that comes with a help text!
    ///     
    ///     println!("{:?}", result);
    ///     
    ///     // output: (4, 4, "median is a single number.")
    /// }
    /// ```
    pub fn result(&self) -> (i32, i32, &str) {
        let len_a: usize = self.list_a.len();
        let len_b: usize = self.list_b.len();

        let (ml, mr) = self.median.unwrap();

        if (len_a + len_b) % 2 != 0 {
            (ml, mr, "median is a single number.")
        } else {
            (ml, mr, "median is consisted of 2 numbers.")
        }
    }
}
