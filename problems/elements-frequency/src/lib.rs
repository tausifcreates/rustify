//! # elements frequency
//!
//! Finds frequency of the unique elements present in the list.
//! This lbrary can work with any types that implement `Clone`.
//! So it is expected it will work with `String`, `&str`, `i32` etc.
//!
//! ### Quick Example
//! ```
//! use elements_frequency::Elements;
//!
//! let list: Vec<i32> = vec![1, 1, -6, 2, 6, 2, 7, 1];
//!
//! let mut elements = Elements::new(list);
//!
//! let result:&Vec<(T, u32)> = elements.find_arbitrary().update_ordered().result();
//!
//! let expected: &Vec<(i32, u32)> = &vec![(1, 3), (-6, 1), (2, 2), (6, 1), (7, 1)];
//!
//! assert_eq!(expected, result);
//! ```

use std::{collections::HashMap, hash::Hash};

#[derive(PartialEq)]
pub struct Row<U> {
    pub element: U,
    pub frequency: u32,
}

impl<U> Row<U> {
    pub fn new(element: U, frequency: u32) -> Self {
        Row { element, frequency }
    }
}

pub struct Elements<'list, T> {
    list: &'list Vec<T>,
    unordered_hash: HashMap<T, u32>,
    ordered_table: Vec<Row<T>>,
}

impl<'list, T> Elements<'list, T>
where
    T: Clone + Hash + Eq,
{
    pub fn new(list: &'list Vec<T>) -> Self {
        Elements {
            list,
            unordered_hash: HashMap::new(),
            ordered_table: Vec::new(),
        }
    }

    pub fn hash_unorderly(&mut self) -> &mut Self {
        let list: &Vec<T> = self.list;

        let unordered_hash: &mut HashMap<T, u32> = &mut self.unordered_hash;

        let ordered_table: &mut Vec<Row<T>> = &mut self.ordered_table;

        for i in list {
            match unordered_hash.get_mut(i) {
                Some(val) => {
                    *val += 1;
                }

                None => {
                    unordered_hash.insert((*i).clone(), 1);
                    ordered_table.push(Row::new((*i).clone(), 0))
                }
            }
        }

        self
    }

    pub fn update_ordered(&mut self) -> &Self {
        let unordered_hash: &mut HashMap<T, u32> = &mut self.unordered_hash;

        let ordered_table: &mut Vec<Row<T>> = &mut self.ordered_table;

        for row in ordered_table.iter_mut() {
            let val: &u32 = unordered_hash.get(&row.element).unwrap();
            row.frequency += *val;
        }

        self
    }

    pub fn result(&self) -> &Vec<Row<T>> {
        let ordered_table: &Vec<Row<T>> = &self.ordered_table;
        ordered_table
    }
}
