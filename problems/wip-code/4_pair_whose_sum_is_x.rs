use std::collections::HashSet;

fn main() {
    let arr = vec![-10, 5, 8, 12, 17];

    let desired_sum: i32 = -5;

    let result: Option<(i32, i32)> = find_pair(&arr, desired_sum);

    match result {
        Some(v) => println!("First value: {}\nSecond value: {}", v.0, v.1),
        None => println!("Such pair not found."),
    }
}

fn find_pair(arr: &Vec<i32>, desired_sum: i32) -> Option<(i32, i32)> {
    let mut set: HashSet<i32> = HashSet::new();

    let mut seek_match: i32;

    let mut found_match: Option<(i32, i32)> = None;

    for i in arr {
        seek_match = desired_sum - i;

        let result: Option<&i32> = set.get(&seek_match);

        match result {
            Some(v) => {
                found_match = Some((*i, *v));
                break;
            }
            None => {
                set.insert(*i);
            }
        }
    }

    found_match
}
