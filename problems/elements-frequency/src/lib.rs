use std::{collections::HashMap, hash::Hash};

pub struct Elements<'list, T> {
    list: &'list Vec<T>,
    arbitrary_table: HashMap<T, u32>,
    ordered_table: Vec<(T, u32)>,
}

impl<'list, T> Elements<'list, T>
where
    T: Clone + Hash + Eq,
{
    pub fn new(list: &'list Vec<T>) -> Self {
        Elements {
            list,
            arbitrary_table: HashMap::new(),
            ordered_table: Vec::new(),
        }
    }

    pub fn arbitrary_frequency_table(&mut self) -> &mut Self {
        let list: &Vec<T> = self.list;

        let arbitrary_table: &mut HashMap<T, u32> = &mut self.arbitrary_table;

        let ordered_table: &mut Vec<(T, u32)> = &mut self.ordered_table;

        for i in list {
            match arbitrary_table.get_mut(i) {
                Some(val) => {
                    *val += 1;
                }

                None => {
                    arbitrary_table.insert((*i).clone(), 1);
                    ordered_table.push(((*i).clone(), 0));
                }
            }
        }

        self
    }

    pub fn update_ordered(&mut self) -> &Self {
        let arbitrary_table: &mut HashMap<T, u32> = &mut self.arbitrary_table;

        let ordered_table: &mut Vec<(T, u32)> = &mut self.ordered_table;

        for (elem, frequency) in ordered_table.iter_mut() {
            let val = arbitrary_table.get(elem).unwrap();
            *frequency += *val;
        }

        self
    }

    pub fn result(&self) -> &Vec<(T, u32)> {
        let ordered_table: &Vec<(T, u32)> = &self.ordered_table;
        ordered_table
    }
}