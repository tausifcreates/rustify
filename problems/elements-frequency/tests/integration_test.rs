use elements_frequency::interface::frequency_finder;
use std::{fmt::Debug, hash::Hash, marker::Sync};

#[cfg(test)]
mod tests {
    use super::*;

    fn setup<T>(list: &[T], expected: &[(T, u64)])
    where
        T: Copy + Eq + Hash + Debug + Sync + Send,
    {
        let frequency_table = frequency_finder(list);

        println!("{:?}", frequency_table);

        let mut flag = 0;

        for couple in frequency_table {
            for k in expected {
                if couple.0 == k.0 {
                    flag += 1;
                    assert_eq!(k.1, couple.1);
                }
            }
            if flag == 0 {
                panic!("item could not be found")
            }
        }
    }

    #[test]
    fn random_int_1() {
        let list = [1, 1, -6, 2, 6, 2, 7, 1];
        let expected = [(1, 3), (-6, 1), (2, 2), (6, 1), (7, 1)];
        setup(&list, &expected);
    }

    #[test]
    fn random_int_2() {
        let list: Vec<i32> = vec![-5, 11, 4, 4, -5, -7, 11];

        let expected = vec![(-5, 2), (11, 2), (4, 2), (-7, 1)];

        setup(&list, &expected);
    }

    #[test]
    fn positive_int_1() {
        let list = [1, 3, 3, 1, 2, 3, 2, 1];

        let expected = [(1, 3), (3, 3), (2, 2)];

        setup(&list, &expected);
    }

    #[test]
    fn positive_int_2() {
        let list = vec![2, 2, 2, 2, 2, 2];

        let expected = vec![(2, 6)];

        setup(&list, &expected);
    }

    #[test]
    fn negative_int_1() {
        let list = [-5, -2, -7, -2, -7, -7, -2, -5, -8];

        let expected = [(-5, 2), (-2, 3), (-7, 3), (-8, 1)];

        setup(&list, &expected);
    }

    #[test]
    fn negative_int_2() {
        let list = vec![-3, -3, -3, -3, -3, -3];

        let expected = vec![(-3, 6)];

        setup(&list, &expected);
    }

    #[test]
    fn slice_1() {
        let list = ["hi", "who", "me", "who", "me", "hi"];

        let expected = [("hi", 2), ("who", 2), ("me", 2)];

        setup(&list, &expected);
    }

    #[test]
    fn slice_2() {
        let list = vec!["hi", "hi", "hi", "hi", "hi", "hi"];

        let expected = vec![("hi", 6)];

        setup(&list, &expected);
    }
}
