use max_subarray_sum::Elements;

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
}
