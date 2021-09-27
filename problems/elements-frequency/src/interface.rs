use std::{collections::HashMap, hash::Hash};

#[derive(Debug, PartialEq)]
pub struct Row<U> {
    pub element: U,
    pub frequency: u32,
}

impl<U> Row<U> {
    /// Returns an instance of `Row` struct.
    pub fn new(element: U, frequency: u32) -> Self {
        Row { element, frequency }
    }
}

#[derive(Debug)]
pub struct Elements<'list, T> {
    list: &'list Vec<T>,
    couple_hash: HashMap<T, u32>,
    ordered_table: Vec<Row<T>>,
}

impl<'list, T> Elements<'list, T>
where
    T: Clone + Hash + Eq,
{
    /// Returns a new instance of the struct.
    pub fn new(list: &'list Vec<T>) -> Self {
        Elements {
            list,
            couple_hash: HashMap::new(),
            ordered_table: Vec::new(),
        }
    }

    /// Hash the elements to their frequency.
    pub fn hash_couple(&mut self) -> &mut Self {
        let list: &Vec<T> = self.list;

        let couple_hash: &mut HashMap<T, u32> = &mut self.couple_hash;

        let ordered_table: &mut Vec<Row<T>> = &mut self.ordered_table;

        for i in list {
            match couple_hash.get_mut(i) {
                Some(val) => {
                    *val += 1;
                }

                None => {
                    couple_hash.insert((*i).clone(), 1);
                    ordered_table.push(Row::new((*i).clone(), 0))
                }
            }
        }

        self
    }

    /// When we iterate over the hash, the elements might come unordered.
    /// So we update their order in this method in a third list namingly
    /// frequency table.
    pub fn update_order(&mut self) -> &Self {
        let couple_hash: &mut HashMap<T, u32> = &mut self.couple_hash;

        let ordered_table: &mut Vec<Row<T>> = &mut self.ordered_table;

        for row in ordered_table.iter_mut() {
            let val: &u32 = couple_hash.get(&row.element).unwrap();
            row.frequency += *val;
        }

        self
    }

    /// Finally we need to chain `hash_couple` and `update_order` with
    /// `result` method to get final result.
    ///
    /// # Examples
    /// ```
    /// use elements_frequency::interface::Elements;
    /// 
    /// let list = vec![-5, 11, 4, 4, -5, -7, 11];
    ///
    /// let mut elements = Elements::new(&list);
    /// 
    /// let frequency_table = elements.hash_couple().update_order().result();
    /// ```
    pub fn result(&self) -> &Vec<Row<T>> {
        let ordered_table: &Vec<Row<T>> = &self.ordered_table;
        ordered_table
    }
}
