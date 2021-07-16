fn main() {
    let mut arr: Vec<i32> = vec![12, 4, -5, 7, -9, 8, 11, 1, -3, -5];

    let desired_sum: i32 = 10;

    let len: usize = arr.len();

    arr.sort();

    let distance: i32 = init_distance(&mut arr, len, desired_sum);

    let values: Option<(i32, i32)> = find_pair(&mut arr, len, desired_sum, distance);

    match values {
        Some(v) => println!("First value: {}\nSecond value: {}", v.0, v.1),
        None => (),
    }
}

fn init_distance(arr: &mut Vec<i32>, len: usize, desired_sum: i32) -> i32 {
    let distance: i32;

    let highest_sum: i32 = arr[len - 1] + arr[len - 2];

    let lowest_sum: i32 = arr[0] + arr[1];

    let avg_sum: i32 = (highest_sum + lowest_sum) / 2;

    if avg_sum - desired_sum <= 0 {
        distance = desired_sum - lowest_sum;
    } else {
        distance = highest_sum - desired_sum;
    }

    distance
}

fn find_pair(
    arr: &mut Vec<i32>,
    len: usize,
    desired_sum: i32,
    mut distance: i32,
) -> Option<(i32, i32)> {
    let mut left_index: usize = 0;

    let mut right_index: usize = len - 1;

    let mut result: Option<(i32, i32)> = None;

    for _ in 0..(len - 2) + 1 {
        let temp_sum: i32 = arr[right_index] + arr[left_index];

        let temp_distance: i32 = desired_sum - temp_sum;

        if temp_distance.abs() < distance.abs() {
            distance = temp_distance;
            result = Some((arr[left_index], arr[right_index]));
        }

        if temp_distance > 0 {
            left_index += 1;
        } else {
            right_index -= 1;
        }
    }

    result
}
