fn main() {
    let mut arr: [i32; 10] = [12, 4, -5, 7, -9, 8, 11, 1, -3, -5];

    let len: usize = arr.len();

    let desired_sum = 25;

    let mut distance: i32;

    arr.sort();

    println!("{:?}", arr);

    let highest_sum = arr[len - 1] + arr[len - 2];

    let lowest_sum = arr[0] + arr[1];

    let avg_sum = (highest_sum + lowest_sum) / 2;

    if avg_sum - desired_sum <= 0 {
        distance = desired_sum - lowest_sum;
    } else {
        distance = highest_sum - desired_sum;
    }

    let mut left_index = 0;

    let mut right_index = len - 1;

    let mut final_left_index: usize = len;

    let mut final_right_index: usize = len;

    for _ in 0..(len - 2) + 1 {
        println!("left index: {}, right_index: {}", left_index, right_index);

        let temp_sum = arr[right_index] + arr[left_index];

        println!("temp_sum: {}", temp_sum);

        let temp_distance = desired_sum - temp_sum;

        println!("temp_distance: {}", temp_distance);

        if temp_distance.abs() < distance.abs() {
            distance = temp_distance;
            final_left_index = left_index;
            final_right_index = right_index;

            println!(
                "distance: {}, final_left: {}, final_right: {}",
                distance, final_left_index, final_right_index
            );
        }

        if desired_sum > temp_sum {
            left_index += 1;
        } else {
            right_index -= 1;
        }
    }

    println!(
        "final left index: {}, final right_index: {}",
        final_left_index, final_right_index
    );
}
