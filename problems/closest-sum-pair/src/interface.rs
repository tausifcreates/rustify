// `distance ≥ | sum of any pair - desired sum |`
pub fn find_distance(list: &mut [i32], desired_sum: i32) -> i32 {
    list.sort();

    let len: usize = list.len();

    let highest_sum: i32 = list[len - 1] + list[len - 2];

    let lowest_sum: i32 = list[0] + list[1];

    let avg_sum: i32 = (highest_sum + lowest_sum) / 2;

    if avg_sum - desired_sum <= 0 {
        return desired_sum - lowest_sum;
    }

    return highest_sum - desired_sum;
}
// This method runs a loop and tries to minimize the distance
// for every comparison between current distance and sum of
// two numbers in list.
/// # Examples
/// ```
/// use closest_sum_pair::interface::find_pair;
///
/// fn main() {
///    let mut list = [-2, -4, -7, -2, -5, -13, -7];
///
///     let desired_sum = -1;
///
///     let pair = find_pair(&mut list, desired_sum);
///
///     println!("pair {:?}", pair) // (-2, -2)
/// }
/// ```
pub fn find_pair(list: &mut [i32], desired_sum: i32) -> (i32, i32) {
    let mut init_distance = find_distance(list, desired_sum);

    let mut pair: Option<(i32, i32)> = None;

    let len: usize = list.len();

    let mut left_index: usize = 0;

    let mut right_index: usize = len - 1;

    for _ in 0..(len - 2) + 1 {
        let temp_sum: i32 = list[left_index] + list[right_index];

        let temp_distance: i32 = desired_sum - temp_sum;

        if temp_distance.abs() < init_distance.abs() {
            init_distance = temp_distance;
            pair = Some((list[left_index], list[right_index]));
        }

        if temp_distance > 0 {
            left_index += 1;
        } else if temp_distance < 0 {
            right_index -= 1;
        } else {
            break;
        }
    }

    pair.unwrap()
}