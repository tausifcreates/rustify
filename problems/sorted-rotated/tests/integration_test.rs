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
    fn pivot_random() {
        let list: Vec<i32> = vec![5, 6, 1, 2, 3, 4];

        let desired: i32 = 2;

        let expected: Option<usize> = Some(3);

        setup(&list, desired, expected);
    }
}
