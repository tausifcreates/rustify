use closest_sum_pair::Elements;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(list: &mut Vec<i32>, desired_sum: i32, expected_pair: (i32, i32)) {
        let len: usize = list.len();

        let mut elements: Elements = Elements::new(list, len, desired_sum);

        let pair: (i32, i32) = elements
            .sort_list()
            .find_init_distance()
            .find_pair()
            .result();

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


