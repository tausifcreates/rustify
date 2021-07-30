#[cfg(test)]
mod tests {
    use sorted_rotated::Elements;

    fn setup(list: &Vec<i32>, desired: i32, expected: Option<usize>) {
        let mut elem: Elements = Elements::new(list, desired);

        let result: Option<usize> = elem.find_pivot().find_desired().result();

        assert_eq!(expected, result);
    }

    #[test]
    fn no_pivot() {
        let list: Vec<i32> = vec![1, 2, 3, 4, 5];

        let desired: i32 = 2;

        let expected: Option<usize> = Some(1);

        setup(&list, desired, expected);
    }

    #[test]
    fn random_1() {
        let list: Vec<i32> = vec![5, 6, 1, 2, 3, 4];

        let desired: i32 = 2;

        let expected: Option<usize> = Some(3);

        setup(&list, desired, expected);
    }

    #[test]
    fn random_3() {
        let list: Vec<i32> = vec![4, 5, 2];

        let desired: i32 = 5;

        let expected: Option<usize> = Some(1);

        setup(&list, desired, expected);
    }

    #[test]
    fn mix_1() {
        let list: Vec<i32> = vec![-2, 3, 7, 11, 14, 16, -7, -5];

        let desired: i32 = 14;

        let expected: Option<usize> = Some(4);

        setup(&list, desired, expected);
    }

    #[test]
    fn mix_2() {
        let list: Vec<i32> = vec![12, -8, -7, -3, -2, 2, 6, 8, 9];

        let desired: i32 = 12;

        let expected: Option<usize> = Some(0);

        setup(&list, desired, expected);
    }

    #[test]
    fn negative_1() {
        let list: Vec<i32> = vec![-3, -1, -12, -8, -6, -5];

        let desired: i32 = -8;

        let expected: Option<usize> = Some(3);

        setup(&list, desired, expected);
    }

    #[test]
    fn two() {
        let list: Vec<i32> = vec![7, 4];

        let desired: i32 = 4;

        let expected: Option<usize> = Some(1);

        setup(&list, desired, expected);
    }

    #[test]
    fn random_2() {
        let list: Vec<i32> = vec![4, 1, 2, 3];

        let desired: i32 = 2;

        let expected: Option<usize> = Some(2);

        setup(&list, desired, expected);
    }
}
