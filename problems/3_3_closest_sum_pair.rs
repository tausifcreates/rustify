struct SortedNumbers<'list> {
    list: &'list mut Vec<i32>,
    len: usize,
    pair: Option<(i32, i32)>,
}

impl<'list> SortedNumbers<'list> {
    fn new(list: &'list mut Vec<i32>, len: usize) -> Self {
        list.sort();

        SortedNumbers {
            list,
            len,
            pair: None,
        }
    }

    fn init_distance(&self, desired_sum: i32) -> i32 {
        let list: &Vec<i32> = self.list;

        let len: usize = self.len;

        let highest_sum: i32 = list[len - 1] + list[len - 2];

        let lowest_sum: i32 = list[0] + list[1];

        let avg_sum: i32 = (highest_sum + lowest_sum) / 2;

        let distance: i32;

        if avg_sum - desired_sum <= 0 {
            distance = desired_sum - lowest_sum;
        } else {
            distance = highest_sum - desired_sum;
        }

        distance
    }

    fn find_pair(&mut self, desired_sum: i32, mut distance: i32) -> &Self {
        let list: &mut Vec<i32> = self.list;

        let len: usize = self.len;

        let mut left_index: usize = 0;

        let mut right_index: usize = len - 1;

        for _ in 0..(len - 2) + 1 {
            let temp_sum: i32 = list[left_index] + list[right_index];

            let temp_distance: i32 = desired_sum - temp_sum;

            if temp_distance.abs() < distance.abs() {
                distance = temp_distance;
                self.pair = Some((list[left_index], list[right_index]));
            }

            if temp_distance > 0 {
                left_index += 1;
            } else if temp_distance < 0 {
                right_index -= 1;
            } else {
                break;
            }
        }

        self
    }

    fn result(&self) {
        match self.pair {
            Some((first_val, second_val)) => {
                println!("First value: {}\nSecond value: {}", first_val, second_val)
            }
            None => (),
        }
    }
}

fn main() {
    let mut list: Vec<i32> = vec![12, 4, -5, 7, -9, 8, 11, 1, -3, -5];

    let len = list.len();

    let desired_sum: i32 = 16;

    let mut list = SortedNumbers::new(&mut list, len);

    let distance = list.init_distance(desired_sum);

    list.find_pair(desired_sum, distance).result();
}
