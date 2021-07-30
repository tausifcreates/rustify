pub struct Elements<'list> {
    list: &'list [i32],
    len: usize,
    pivot: Option<usize>,
    //desired: i32,
    //result: Option<usize>,
}

impl<'list> Elements<'list> {
    pub fn new(list: &'list [i32], _desired: i32) -> Self {
        Elements {
            list,
            len: list.len(),
            pivot: None,
            //desired,
            //result: None,
        }
    }

    pub fn find_pivot(&mut self) -> &mut Self {
        let list: &[i32] = self.list;

        let len: usize = self.len;

        let turns: usize = (len as f32).log(2.0).abs() as usize + 1;

        let mut left: usize = 0;

        let mut right: usize = len - 1;

        for _ in 0..turns {
            let mid: usize = (left + right) / 2;

            if list[mid] > list[mid + 1] {
                self.pivot = Some(mid);
                break;
            }

            if list[mid] > list[left] {
                left = mid;
            } else if list[mid] < list[left] {
                right = mid;
            }
        }

        self
    }

    pub fn result(&self) -> Option<usize> {
        self.pivot
    }
}
