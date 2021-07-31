use elements_frequency::interface::{Elements, Row};
use std::{fmt::Debug, hash::Hash, u32};

#[cfg(test)]
mod tests {
    use super::*;

    fn setup<T>(list: &Vec<T>, expected: &Vec<(T, u32)>) -> ()
    where
        T: Clone + Hash + Eq + Debug,
    {
        let mut elements: Elements<T> = Elements::new(list);

        let frequency_table: &Vec<Row<T>> = elements.hash_couple().update_order().result();

        println!("{:?}", frequency_table);

        let mut i: usize = 0;

        for row in frequency_table.iter() {
            let (element, frequency) = ((row.element).clone(), row.frequency);

            assert_eq!(expected[i].0, element);
            assert_eq!(expected[i].1, frequency);

            i += 1;
        }
    }

    #[test]
    fn random_int_1() {
        let list: Vec<i32> = vec![1, 1, -6, 2, 6, 2, 7, 1];

        let expected: Vec<(i32, u32)> = vec![(1, 3), (-6, 1), (2, 2), (6, 1), (7, 1)];

        setup(&list, &expected);
    }

    #[test]
    fn random_int_2() {
        let list: Vec<i32> = vec![-5, 11, 4, 4, -5, -7, 11];

        let expected: Vec<(i32, u32)> = vec![(-5, 2), (11, 2), (4, 2), (-7, 1)];

        setup(&list, &expected);
    }

    #[test]
    fn positive_int_1() {
        let list: Vec<i32> = vec![1, 3, 3, 1, 2, 3, 2, 1];

        let expected: Vec<(i32, u32)> = vec![(1, 3), (3, 3), (2, 2)];

        setup(&list, &expected);
    }

    #[test]
    fn positive_int_2() {
        let list: Vec<i32> = vec![2, 2, 2, 2, 2, 2];

        let expected: Vec<(i32, u32)> = vec![(2, 6)];

        setup(&list, &expected);
    }

    #[test]
    fn negative_int_1() {
        let list: Vec<i32> = vec![-5, -2, -7, -2, -7, -7, -2, -5, -8];

        let expected: Vec<(i32, u32)> = vec![(-5, 2), (-2, 3), (-7, 3), (-8, 1)];

        setup(&list, &expected);
    }

    #[test]
    fn negative_int_2() {
        let list: Vec<i32> = vec![-3, -3, -3, -3, -3, -3];

        let expected: Vec<(i32, u32)> = vec![(-3, 6)];

        setup(&list, &expected);
    }

    #[test]
    fn slice_1() {
        let list: Vec<&str> = vec!["hi", "who", "me", "who", "me", "hi"];

        let expected: Vec<(&str, u32)> = vec![("hi", 2), ("who", 2), ("me", 2)];

        setup(&list, &expected);
    }

    #[test]
    fn slice_2() {
        let list: Vec<&str> = vec!["hi", "hi", "hi", "hi", "hi", "hi"];

        let expected: Vec<(&str, u32)> = vec![("hi", 6)];

        setup(&list, &expected);
    }

    #[test]
    fn string_1() {
        let list: Vec<String> = vec![
            String::from("hey"),
            String::from("you"),
            String::from("hey"),
            String::from("stop"),
            String::from("you"),
            String::from("stop"),
        ];

        let expected: Vec<(String, u32)> = vec![
            (String::from("hey"), 2),
            (String::from("you"), 2),
            (String::from("stop"), 2),
        ];

        setup(&list, &expected);
    }

    #[test]
    fn string_2() {
        let list: Vec<String> = vec![
            String::from("hi"),
            String::from("hi"),
            String::from("hi"),
            String::from("hi"),
            String::from("hi"),
            String::from("hi"),
        ];

        let expected: Vec<(String, u32)> = vec![(String::from("hi"), 6)];

        setup(&list, &expected);
    }

    #[test]
    fn char_1() {
        let list: Vec<char> = vec!['a', 'c', 'a', 'b', 'c'];

        let expected: Vec<(char, u32)> = vec![('a', 2), ('c', 2), ('b', 1)];

        setup(&list, &expected);
    }
}
