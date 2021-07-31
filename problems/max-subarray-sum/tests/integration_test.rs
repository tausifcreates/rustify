use max_subarray_sum::interface::structure::elements::Elements;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(list: &[i32], expected_sum: i32) {
        let mut elements: Elements = Elements::new(&list);

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
        let list: [i32; 5] = [-1, -1, -1, -1, -1];

        let expected_sum: i32 = -1;

        setup(&list, expected_sum);
    }
}
