#[cfg(test)]
mod tests {
    use sorted_rotated::Elements;

    fn setup(list: &Vec<i32>, expected: Option<usize>) {
        let mut elem = Elements::new(list, 0);
        let result = elem.find_pivot().result();
        assert_eq!(expected, result);
    }

    #[test]
    fn no_pivot() {
        let list = vec![1, 2, 3, 4, 5];

        let expected: Option<usize> = None;

        setup(&list, expected);
    }

    #[test]
    fn pivot_random() {
        let list = vec![5, 6, 1, 2, 3, 4];

        let expected: Option<usize> = Some(1);

        setup(&list, expected);
    }
}
