use std::fmt::Debug;

struct Elements<'list, T> {
    list: &'list Vec<T>,
    len: usize,
    majority_element: Option<T>,
    candidate_index: usize,
}

impl<'list, T> Elements<'list, T>
where
    T: Clone + PartialOrd + Debug,
{
    fn new(list: &'list Vec<T>, len: usize) -> Self {
        Elements {
            list,
            len,
            majority_element: None,
            candidate_index: 0,
        }
    }

    fn pick_candidate(&mut self) -> &mut Self {
        let list = self.list;

        let len = self.len;

        let mut count: u32 = 1;

        let maj_index = &mut self.candidate_index;

        for i in 1..len - 1 {
            if list[i] == list[*maj_index] {
                count += 1;
            } else {
                count -= 1;
            }

            if count == 0 {
                *maj_index = i;
                count = 1;
            }
        }

        self
    }

    fn validate(&mut self) -> &Self {
        let list = self.list;

        let len = self.len;

        let candidate_index = self.candidate_index;

        let mut occurance_count = 0;

        for elem in list.iter() {
            if elem == &list[candidate_index] {
                occurance_count += 1;
            }
        }

        if occurance_count > len / 2 {
            let element = list[candidate_index].clone();
            self.majority_element = Some(element);
        }

        self
    }

    fn result(&self) {
        match self.majority_element.clone() {
            Some(v) => println!("Majority element: {:?}", v),
            None => println!("There is no majority element."),
        }
    }
}

fn main() {
    let list = vec!["hi", "there", "hi", "hi"];

    let len: usize = list.len();

    let mut elements = Elements::new(&list, len);

    elements.pick_candidate().validate().result();
}
