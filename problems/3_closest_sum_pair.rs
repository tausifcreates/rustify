// A program that, given an array of `n` numbers and
// another number `x`, finds two elements in the array
// whose sum is closest to `x`.

// Time Complexity: O(NlogN)
// Space Complexity: O(1)

#[allow(unused)]
struct Elements<'list> {
    list: &'list Vec<i32>,
    len: usize,
    desired_sum: i32,
    pair: Option<(i32, i32)>,
    init_distance: Option<i32>,
}

#[allow(dead_code)]
impl<'list> Elements<'list> {
    pub fn new(list: &'list Vec<i32>, len: usize, desired_sum: i32) -> Self {
        Elements {
            list,
            len,
            desired_sum,
            pair: None,
            init_distance: None,
        }
    }

    fn find_init_distance(&mut self) -> &mut Self {
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

    fn find_pair(&mut self) -> &Self {
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

    fn result(&self) -> (i32, i32) {
        let (first_val, second_val) = self.pair.unwrap();
        println!("First value: {},\nSecond value: {}", first_val, second_val);
        (first_val, second_val)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(list: &mut Vec<i32>, desired_sum: i32, expected_pair: (i32, i32)) {
        list.sort();

        let len: usize = list.len();

        let mut elements = Elements::new(&list, len, desired_sum);

        let pair: (i32, i32) = elements.find_init_distance().find_pair().result();

        assert_eq!(expected_pair, pair);
    }

    #[test]
    fn mixed_upper() {
        let mut list: Vec<i32> = vec![12, 4, -5, 7, -9, 8, 11, 1, -3, -5];

        let desired_sum: i32 = 16;

        let expected_pair: (i32, i32) = (4, 12);

        setup(&mut list, desired_sum, expected_pair);
    }

    #[test]
    fn mixed_middle() {
        let mut list: Vec<i32> = vec![12, 4, -5, 7, -9, 8, 11, 1, -3, -5];

        let desired_sum: i32 = 6;

        let expected_pair: (i32, i32) = (-5, 11);

        setup(&mut list, desired_sum, expected_pair);
    }

    #[test]
    fn mixed_lower() {
        let mut list: Vec<i32> = vec![12, 4, -5, 7, -9, 8, 11, 1, -3, -5];

        let desired_sum: i32 = -13;

        let expected_pair: (i32, i32) = (-9, -3);

        setup(&mut list, desired_sum, expected_pair);
    }

    #[test]
    fn positive_upper() {
        let mut list: Vec<i32> = vec![2, 5, 7, 9, 13, 14, 6, 1, 11];

        let desired_sum: i32 = 25;

        let expected_pair: (i32, i32) = (11, 14);

        setup(&mut list, desired_sum, expected_pair);
    }

    #[test]
    fn positive_middle() {
        let mut list: Vec<i32> = vec![2, 5, 7, 9, 13, 14, 6, 1, 11];

        let desired_sum: i32 = 17;

        let expected_pair: (i32, i32) = (6, 11);

        setup(&mut list, desired_sum, expected_pair);
    }

    #[test]
    fn positive_lower() {
        let mut list: Vec<i32> = vec![2, 5, 7, 9, 13, 14, 6, 1, 11];

        let desired_sum: i32 = -7;

        let expected_pair: (i32, i32) = (1, 2);

        setup(&mut list, desired_sum, expected_pair);
    }

    #[test]
    fn negative_upper() {
        let mut list: Vec<i32> = vec![-2, -4, -7, -2, -5, -13, -7];

        let desired_sum: i32 = -1;

        let expected_pair: (i32, i32) = (-2, -2);

        setup(&mut list, desired_sum, expected_pair);
    }

    #[test]
    fn negative_middle() {
        let mut list: Vec<i32> = vec![-2, -4, -7, -2, -5, -13, -7];

        let desired_sum: i32 = -10;

        let expected_pair: (i32, i32) = (-7, -2);

        setup(&mut list, desired_sum, expected_pair);
    }

    #[test]
    fn negative_lower() {
        let mut list: Vec<i32> = vec![-2, -4, -7, -2, -5, -13, -7];

        let desired_sum: i32 = -16;

        let expected_pair: (i32, i32) = (-13, -2);

        setup(&mut list, desired_sum, expected_pair);
    }
}
