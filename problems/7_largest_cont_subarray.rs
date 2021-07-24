// Program to find the sum of contiguous subarray within a
// one-dimensional array of numbers that has the largest sum.

// Time Complexity: O(n)
// Space Complexity: O(1)

struct Elements<'list> {
    list: &'list Vec<i32>,
    len: usize,
    lcs_sum: Option<i32>,
}

impl<'list> Elements<'list> {
    fn new(list: &'list Vec<i32>, len: usize) -> Self {
        Elements {
            list,
            len,
            lcs_sum: None,
        }
    }

    fn find_lcs_sum(&mut self) -> &Self {
        let list: &Vec<i32> = self.list;

        let len: usize = self.len;

        let mut temp_lcs_sum: i32 = list[0];

        let mut lcs_sum: i32 = temp_lcs_sum;

        for i in 1..len {
            if temp_lcs_sum > 0 {
                temp_lcs_sum += list[i];
            } else {
                temp_lcs_sum = list[i];
            }

            if temp_lcs_sum > lcs_sum {
                lcs_sum = temp_lcs_sum;
            }
        }

        self.lcs_sum = Some(lcs_sum);

        self
    }

    fn result(&self) -> i32 {
        let lcs_sum: i32 = self.lcs_sum.unwrap();
        println!("LCS sum: {}", lcs_sum);
        lcs_sum
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(list: &Vec<i32>, expected_sum: i32) {
        let len: usize = list.len();

        let mut elements: Elements = Elements::new(&list, len);

        let max_sum: i32 = elements.find_max_sum().result();

        assert_eq!(expected_sum, max_sum);
    }

    #[test]
    fn mixed_1() {
        let list: Vec<i32> = vec![-2, -3, 4, -1, -2, 1, 5, -3];

        let expected_sum: i32 = 7;

        setup(&list, expected_sum);
    }

    #[test]
    fn mixed_2() {
        let list: Vec<i32> = vec![5, -4, 13, 6, -9, 11, 0, 3];

        let expected_sum: i32 = 25;

        setup(&list, expected_sum);
    }

    #[test]
    fn mixed_3() {
        let list: Vec<i32> = vec![-1, -4, -7, 6, 9, 12];

        let expected_sum: i32 = 27;

        setup(&list, expected_sum);
    }

    #[test]
    fn all_zero() {
        let list: Vec<i32> = vec![0, 0, 0, 0, 0, 0];

        let expected_sum: i32 = 0;

        setup(&list, expected_sum);
    }

    #[test]
    fn all_positive() {
        let list: Vec<i32> = vec![2, 5, 6, 9, 12, 16];

        let expected_sum: i32 = 50;

        setup(&list, expected_sum);
    }

    #[test]
    fn all_negative() {
        let list: Vec<i32> = vec![-2, -3, 4, -1, -2, 1, 5, -3];

        let expected_sum: i32 = 7;

        setup(&list, expected_sum);
    }

    #[test]
    fn identical_positive() {
        let list: Vec<i32> = vec![2, 2, 2, 2, 2];

        let expected_sum: i32 = 10;

        setup(&list, expected_sum);
    }

    #[test]
    fn identical_negative() {
        let list: Vec<i32> = vec![-1, -1, -1, -1, -1];

        let expected_sum: i32 = -1;

        setup(&list, expected_sum);
    }
}

