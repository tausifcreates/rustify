use elements_frequency::Elements;

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, hash::Hash};

    use super::*;

    fn setup<T>(list: &Vec<T>, expected: &Vec<(T, u32)>) -> () 
    where T: Clone + Hash + Eq + Debug
    {
        let mut elements = Elements::new(list);

        let result = elements
            .arbitrary_frequency_table()
            .update_ordered()
            .result();
        
        assert_eq!(expected, result);
    }

    #[test]
    fn positive_random() {
        let list: Vec<i32> = vec![1, 1, -6, 2, 6, 2, 7, 1];

        let expected: &Vec<(i32, u32)> = &vec![(1, 3), (-6, 1), (2, 2), (6, 1), (7, 1)];

        setup(&list, expected);
    }
}