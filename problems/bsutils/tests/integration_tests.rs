#[cfg(test)]
mod tests {
    use bsutils::interface::first_last_idxs;

    #[allow(unused_variables)]
    fn setup(list: &[i32], given: i32, test_against: (Option<usize>, Option<usize>)) {
        let (f_idx, l_idx) = first_last_idxs(list, given);
        println!("{:?}, {:?}", f_idx, l_idx);
        assert_eq!(test_against.0, f_idx);
        assert_eq!(test_against.1, l_idx);
    }

    #[test]
    fn test1() {
        let list = [1, 3, 6, 6, 6, 8, 8, 9, 12];
        let given = 8;
        let test_against: (Option<usize>, Option<usize>) = (Some(5), Some(6));
        setup(&list, given, test_against);
    }

    #[test]
    fn test2() {
        let list = [1, 6, 6, 6, 8, 8, 9, 12];
        let given = 6;
        let test_against: (Option<usize>, Option<usize>) = (Some(1), Some(3));
        setup(&list, given, test_against);
    }

    #[test]
    fn test3() {
        let list = [1, 2, 3, 4, 6];
        let given = 6;
        let test_against: (Option<usize>, Option<usize>) = (Some(4), Some(4));
        setup(&list, given, test_against);
    }

    #[test]
    fn test4() {
        let list = [1, 2, 3];
        let given = 3;
        let test_against: (Option<usize>, Option<usize>) = (Some(2), Some(2));
        setup(&list, given, test_against);
    }

    #[test]
    fn test5() {
        let list = [2, 2, 2];
        let given = 2;
        let test_against: (Option<usize>, Option<usize>) = (Some(0), Some(2));
        setup(&list, given, test_against);
    }

    #[test]
    fn test6() {
        let list = [2, 2];
        let given = 2;
        let test_against: (Option<usize>, Option<usize>) = (Some(0), Some(1));
        setup(&list, given, test_against);
    }

    #[test]
    fn test7() {
        let list = [2];
        let given = 2;
        let test_against: (Option<usize>, Option<usize>) = (Some(0), Some(0));
        setup(&list, given, test_against);
    }

    #[test]
    fn test8() {
        let list = [3, 4, 4];
        let given = 4;
        let test_against: (Option<usize>, Option<usize>) = (Some(1), Some(2));
        setup(&list, given, test_against);
    }

    #[test]
    fn test9() {
        let list = [1, 1, 4, 6, 7, 7, 7, 7, 7, 9, 9, 11];
        let given = 7;
        let test_against: (Option<usize>, Option<usize>) = (Some(4), Some(8));
        setup(&list, given, test_against);
    }

    #[test]
    fn test10() {
        let list = [-3, -3, -1, 5, 8, 8, 9, 11];
        let given = -1;
        let test_against: (Option<usize>, Option<usize>) = (Some(2), Some(2));
        setup(&list, given, test_against);
    }
}