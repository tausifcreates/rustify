use std::fmt::Debug;

pub struct Elements<'list, T> {
    big_list: &'list mut [Option<T>],
    small_list: &'list [Option<T>],
    big_len: usize,
    small_len: usize,
}

impl<'list, T> Elements<'list, T>
where
    T: Copy + PartialOrd + Debug + PartialEq,
{
    pub fn new(big_list: &'list mut [Option<T>], small_list: &'list [Option<T>]) -> Self {
        let big_len: usize = big_list.len();
        let small_len: usize = small_list.len();

        Elements {
            big_list,
            small_list,
            big_len,
            small_len,
        }
    }

    pub fn roll(&mut self) -> &mut Self {
        let big_list: &mut &mut [Option<T>] = &mut self.big_list;

        let big_len: usize = self.big_len;

        let small_len: usize = self.small_len;

        let diff: usize = big_len - small_len;

        for i in 0..diff {
            big_list[big_len - 1 - i] = big_list[diff - 1 - i];
            big_list[diff - 1 - i] = None;
        }

        self
    }

    pub fn compare(&mut self) -> &Self {
        let big_list: &mut &mut [Option<T>] = &mut self.big_list;

        let small_list: &[Option<T>] = self.small_list;

        let big_len: usize = self.big_len;

        let small_len: usize = self.small_len;

        let mut big_i: usize = small_len;

        let mut small_i: usize = 0;

        for i in 0..big_len {
            if small_i > small_len - 1 {
                continue;
            } else if big_i > big_len - 1 {
                big_list[i] = small_list[small_i];
                small_i += 1;
                continue;
            }

            if big_list[big_i] <= small_list[small_i] {
                big_list[i] = big_list[big_i];
                big_list[big_i] = None;
                big_i += 1;
            } else if small_list[small_i] < big_list[big_i] {
                big_list[i] = small_list[small_i];
                small_i += 1
            }
        }

        self
    }

    pub fn result(&self) -> &[Option<T>] {
        self.big_list
    }
}
