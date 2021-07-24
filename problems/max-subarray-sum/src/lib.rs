
pub struct Elements<'list> {
    list: &'list Vec<i32>,
    len: usize,
    max_sum: Option<i32>,
}

impl<'list> Elements<'list> {
    pub fn new(list: &'list Vec<i32>, len: usize) -> Self {
        Elements {
            list,
            len,
            max_sum: None,
        }
    }

    pub fn find_max_sum(&mut self) -> &Self {
        let list: &Vec<i32> = self.list;

        let len: usize = self.len;

        let mut temp_lcs_sum: i32 = list[0];

        let mut lcs_sum: i32 = temp_lcs_sum;

        for i in 1..len {
            if temp_lcs_sum > 0 {
                temp_lcs_sum += list[i];
            } else {
                temp_lcs_sum = list[i];
            }

            if temp_lcs_sum > lcs_sum {
                lcs_sum = temp_lcs_sum;
            }
        }

        self.max_sum = Some(lcs_sum);

        self
    }

    pub fn result(&self) -> i32 {
        let lcs_sum: i32 = self.max_sum.unwrap();
        lcs_sum
    }
}