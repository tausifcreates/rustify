use no_adjacent::interface as no_adjacent;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(list: &[i32], expected: i32) {
        let result = no_adjacent::calculate(list);
        assert_eq!(expected, result);
    }

    #[test]
    fn test1() {
        let list = [5, 5, 10, 40, 50, 35];
        let expected = 80;
        setup(&list, expected);
    }

    #[test]
    fn test2() {
        let list = [2, 1, 4, 6, 8, 9, 18];
        let expected = 32;
        setup(&list, expected);
    }

    #[test]
    fn test3() {
        let list = [2, 16, 4, 3, 32, 8, 7, 23, 17, 11];
        let expected = 82;
        setup(&list, expected);
    }

    #[test]
    fn test4() {
        let list = [4, 17, 3, 9, 18, 6, 22, 2, 1, 7, 8];
        let expected = 66;
        setup(&list, expected);
    }
}
