use closest_sum_pair::interface::find_pair;
use std::convert::TryInto;
use std::ops::{Add, Sub};

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    fn setup<X>(list: &mut [X], desired_sum: X, expected_pair: (X, X))
    where
        X: Copy + PartialOrd + Debug + Add<Output = X> + Sub<Output = X> + TryInto<f64>,
    {
        list.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let pair = find_pair(list, desired_sum);

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
        let mut list: [i32; 10] = [12, 4, -5, 7, -9, 8, 11, 1, -3, -5];

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
        let mut list: [u32; 9] = [2, 5, 7, 9, 13, 14, 6, 1, 11];

        let desired_sum: u32 = 25;

        let expected_pair: (u32, u32) = (11, 14);

        setup(&mut list, desired_sum, expected_pair);
    }

    #[test]
    fn positive_middle() {
        let mut list: Vec<u16> = vec![2, 5, 7, 9, 13, 14, 6, 1, 11];

        let desired_sum: u16 = 17;

        let expected_pair: (u16, u16) = (6, 11);

        setup(&mut list, desired_sum, expected_pair);
    }

    #[test]
    fn positive_lower() {
        let mut list: [i16; 9] = [2, 5, 7, 9, 13, 14, 6, 1, 11];

        let desired_sum: i16 = -7;

        let expected_pair: (i16, i16) = (1, 2);

        setup(&mut list, desired_sum, expected_pair);
    }

    #[test]
    fn negative_upper() {
        let mut list: Vec<i16> = vec![-2, -4, -7, -2, -5, -13, -7];

        let desired_sum: i16 = -1;

        let expected_pair: (i16, i16) = (-2, -2);

        setup(&mut list, desired_sum, expected_pair);
    }

    #[test]
    fn negative_middle() {
        let mut list: Vec<i8> = vec![-2, -4, -7, -2, -5, -13, -7];

        let desired_sum: i8 = -10;

        let expected_pair: (i8, i8) = (-7, -2);

        setup(&mut list, desired_sum, expected_pair);
    }

    #[test]
    fn negative_lower() {
        let mut list: [i32; 7] = [-2, -4, -7, -2, -5, -13, -7];

        let desired_sum: i32 = -16;

        let expected_pair: (i32, i32) = (-13, -2);

        setup(&mut list, desired_sum, expected_pair);
    }

    #[test]
    fn float_test() {
        let mut list: [f32; 7] = [-2.2, -4.0, -7.9, -2.1, -5.5, -13.0, -7.1];

        let desired_sum: f32 = -16.7;

        let expected_pair: (f32, f32) = (-13.0, -4.0);

        setup(&mut list, desired_sum, expected_pair);
    }
}
