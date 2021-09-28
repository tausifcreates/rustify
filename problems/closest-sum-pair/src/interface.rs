use std::convert::TryInto;
use std::ops::{Add, Sub};

// `distance ≥ | sum of any pair - desired sum |`
pub fn find_distance<T>(list: &mut [T], desired_sum: T) -> (f64, f64)
where
    T: Copy + Add<Output = T> + Sub<Output = T> + TryInto<f64>,
{
    let len: usize = list.len();

    let highest_sum: f64;

    match (list[len - 1] + list[len - 2]).try_into() {
        Ok(val) => highest_sum = val,
        Err(_) => panic!("Could not convert types!"),
    }

    let lowest_sum: f64;

    match (list[0] + list[1]).try_into() {
        Ok(val) => lowest_sum = val,
        Err(_) => panic!("Could not convert types!"),
    }

    let avg_sum = (highest_sum + lowest_sum) / 2.0;

    match desired_sum.try_into() {
        Ok(val) => {
            if avg_sum - val <= 0.0 {
                return (val - lowest_sum, val);
            }

            return (highest_sum - val, val);
        }
        Err(_) => panic!("Could not convert types!"),
    }
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
pub fn find_pair<U>(list: &mut [U], desired_sum: U) -> (U, U)
where
    U: Copy + Add<Output = U> + Sub<Output = U> + TryInto<f64>,
{
    let (mut init_distance, conv_desired_sum) = find_distance(list, desired_sum);

    let mut pair: Option<(U, U)> = None;

    let len: usize = list.len();

    let mut left_index: usize = 0;

    let mut right_index: usize = len - 1;

    for _ in 0..(len - 2) + 1 {
        let temp_sum = list[left_index] + list[right_index];

        let temp_distance: f64;

        match temp_sum.try_into() {
            Ok(val) => {
                temp_distance = conv_desired_sum - val;
            }
            Err(_) => panic!("Could not convert types!"),
        }

        if temp_distance.abs() < init_distance.abs() {
            init_distance = temp_distance;
            pair = Some((list[left_index], list[right_index]));
        }

        if temp_distance > 0.0 {
            left_index += 1;
        } else if temp_distance < 0.0 {
            right_index -= 1;
        } else {
            break;
        }
    }

    pair.unwrap()
}
