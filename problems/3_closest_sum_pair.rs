// A program that, given an array of `n` numbers and
// another number `x`, finds two elements in the array
// whose sum is closest to `x`.

// Time Complexity: O(n)
// Space Complexity: O(1)

struct Elements<'list> {
    list: &'list Vec<i32>,
    len: usize,
    pair: Option<(i32, i32)>,
}

impl<'list> Elements<'list> {
    fn new(list: &'list Vec<i32>, len: usize) -> Self {
        Elements {
            list,
            len,
            pair: None,
        }
    }

    fn init_distance(&self, desired_sum: i32) -> i32 {
        let list: &Vec<i32> = self.list;

        let len: usize = self.len;

        let highest_sum: i32 = list[len - 1] + list[len - 2];

        let lowest_sum: i32 = list[0] + list[1];

        let avg_sum: i32 = (highest_sum + lowest_sum) / 2;

        let distance: i32;

        if avg_sum - desired_sum <= 0 {
            distance = desired_sum - lowest_sum;
        } else {
            distance = highest_sum - desired_sum;
        }

        distance
    }

    fn find_pair(&mut self, desired_sum: i32, mut distance: i32) -> &Self {
        let list: &Vec<i32> = self.list;

        let len: usize = self.len;

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

mod tests {
    use super::*;

    #[test]
    fn mixed_upper() {
        let mut list: Vec<i32> = vec![12, 4, -5, 7, -9, 8, 11, 1, -3, -5];
        list.sort();
        let desired_sum: i32 = 16;
        let mut elems: Elements = Elements::new(&list, list.len());
        let distance = elems.init_distance(desired_sum);
        let pair: (i32, i32) = elems.find_pair(desired_sum, distance).result();

        assert_eq!((4, 12), pair);
    }

    #[test]
    fn mixed_middle() {
        let mut list: Vec<i32> = vec![12, 4, -5, 7, -9, 8, 11, 1, -3, -5];
        list.sort();
        let desired_sum: i32 = 6;
        let mut elems: Elements = Elements::new(&list, list.len());
        let distance = elems.init_distance(desired_sum);
        let pair: (i32, i32) = elems.find_pair(desired_sum, distance).result();

        assert_eq!((-5, 11), pair);
    }

    #[test]
    fn mixed_lower() {
        let mut list: Vec<i32> = vec![12, 4, -5, 7, -9, 8, 11, 1, -3, -5];
        list.sort();
        let desired_sum: i32 = -13;
        let mut elems: Elements = Elements::new(&list, list.len());
        let distance = elems.init_distance(desired_sum);
        let pair: (i32, i32) = elems.find_pair(desired_sum, distance).result();

        assert_eq!((-9, -3), pair);
    }

    #[test]
    fn positive_upper() {
        let mut list: Vec<i32> = vec![2, 5, 7, 9, 13, 14, 6, 1, 11];
        list.sort();
        let desired_sum: i32 = 25;
        let mut elems: Elements = Elements::new(&list, list.len());
        let distance = elems.init_distance(desired_sum);
        let pair: (i32, i32) = elems.find_pair(desired_sum, distance).result();

        assert_eq!((11, 14), pair);
    }

    #[test]
    fn positive_middle() {
        let mut list: Vec<i32> = vec![2, 5, 7, 9, 13, 14, 6, 1, 11];
        list.sort();
        let desired_sum: i32 = 17;
        let mut elems: Elements = Elements::new(&list, list.len());
        let distance = elems.init_distance(desired_sum);
        let pair: (i32, i32) = elems.find_pair(desired_sum, distance).result();

        assert_eq!((6, 11), pair);
    }

    #[test]
    fn positive_lower() {
        let mut list: Vec<i32> = vec![2, 5, 7, 9, 13, 14, 6, 1, 11];
        list.sort();
        let desired_sum: i32 = -7;
        let mut elems: Elements = Elements::new(&list, list.len());
        let distance = elems.init_distance(desired_sum);
        let pair: (i32, i32) = elems.find_pair(desired_sum, distance).result();

        assert_eq!((1, 2), pair);
    }

    #[test]
    fn negative_upper() {
        let mut list: Vec<i32> = vec![-2, -4, -7, -2, -5, -13, -7];
        list.sort();
        let desired_sum: i32 = -1;
        let mut elems: Elements = Elements::new(&list, list.len());
        let distance = elems.init_distance(desired_sum);
        let pair: (i32, i32) = elems.find_pair(desired_sum, distance).result();

        assert_eq!((-2, -2), pair);
    }

    #[test]
    fn negative_middle() {
        let mut list: Vec<i32> = vec![-2, -4, -7, -2, -5, -13, -7];
        list.sort();
        let desired_sum: i32 = -10;
        let mut elems: Elements = Elements::new(&list, list.len());
        let distance = elems.init_distance(desired_sum);
        let pair: (i32, i32) = elems.find_pair(desired_sum, distance).result();

        assert_eq!((-7, -2), pair);
    }

    #[test]
    fn negative_lower() {
        let mut list: Vec<i32> = vec![-2, -4, -7, -2, -5, -13, -7];
        list.sort();
        let desired_sum: i32 = -16;
        let mut elems: Elements = Elements::new(&list, list.len());
        let distance = elems.init_distance(desired_sum);
        let pair: (i32, i32) = elems.find_pair(desired_sum, distance).result();

        assert_eq!((-13, -2), pair);
    }
}
