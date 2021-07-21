struct Elements<'list> {
    list: &'list Vec<i32>,
    len: usize,
    odd_occuring_element: Option<i32>,
}

impl<'list> Elements<'list> {
    fn new(list: &'list Vec<i32>, len: usize) -> Self {
        Elements {
            list,
            len,
            odd_occuring_element: None,
        }
    }

    fn xor_of_elements(&mut self) -> &Self {
        let len = self.len;
        let list = self.list;

        let mut temp = list[0];

        for i in 1..len {
            temp = temp ^ list[i];
        }

        self.odd_occuring_element = Some(temp);

        self
    }

    fn result(&self) {
        let result = self.odd_occuring_element.unwrap();
        println!("{}", result);
    }
}

fn main() {
    let list = vec![0, 1, 1, 0, 2];

    let len = list.len();

    let mut elements = Elements::new(&list, len);

    elements.xor_of_elements().result();
}
