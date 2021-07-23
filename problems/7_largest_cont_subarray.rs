// Program to find the sum of contiguous subarray within a
// one-dimensional array of numbers that has the largest sum.

// Time Complexity: O(n)
// Space Complexity: O(1)

struct Elements<'list> {
    list: &'list Vec<i32>,
    len: usize,
    lcs_sum: Option<i32>,
}

impl<'list> Elements<'list> {
    fn new(list: &'list Vec<i32>, len: usize) -> Self {
        Elements {
            list,
            len,
            lcs_sum: None,
        }
    }

    fn find_lcs_sum(&mut self) -> &Self {
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

        self.lcs_sum = Some(lcs_sum);

        self
    }

    fn result(&self) -> i32 {
        let lcs_sum: i32 = self.lcs_sum.unwrap();
        println!("LCS sum: {}", lcs_sum);
        lcs_sum
    }
}

fn main() {}

mod tests {
    use super::*;

    #[test]
    fn lcs_sum_validify() {
        let list1: Vec<i32> = vec![-2, -3, 4, -1, -2, 1, 5, -3];
        let lcs_sum1: i32 = Elements::new(&list1, list1.len()).find_lcs_sum().result();

        let list2: Vec<i32> = vec![5, -4, 13, 6, -9, 11, 0, 3];
        let lcs_sum2: i32 = Elements::new(&list2, list2.len()).find_lcs_sum().result();

        let list3: Vec<i32> = vec![0, 0, 0, 0, 0, 0];
        let lcs_sum3: i32 = Elements::new(&list3, list3.len()).find_lcs_sum().result();

        let list4: Vec<i32> = vec![2, 5, 6, 9, 12, 16];
        let lcs_sum4: i32 = Elements::new(&list4, list4.len()).find_lcs_sum().result();

        let list5: Vec<i32> = vec![-2, -5, -5, -11, -14];
        let lcs_sum5: i32 = Elements::new(&list5, list5.len()).find_lcs_sum().result();

        let list6: Vec<i32> = vec![-1, -4, -7, 6, 9, 12];
        let lcs_sum6: i32 = Elements::new(&list6, list6.len()).find_lcs_sum().result();

        let list7: Vec<i32> = vec![-1, -1, -1, -1, -1];
        let lcs_sum7: i32 = Elements::new(&list7, list7.len()).find_lcs_sum().result();

        let list8: Vec<i32> = vec![2, 2, 2, 2, 2];
        let lcs_sum8: i32 = Elements::new(&list8, list8.len()).find_lcs_sum().result();

        assert_eq!(7, lcs_sum1);
        assert_eq!(25, lcs_sum2);
        assert_eq!(0, lcs_sum3);
        assert_eq!(50, lcs_sum4);
        assert_eq!(-2, lcs_sum5);
        assert_eq!(27, lcs_sum6);
        assert_eq!(-1, lcs_sum7);
        assert_eq!(10, lcs_sum8);
    }
}
