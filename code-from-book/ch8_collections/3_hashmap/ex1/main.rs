use std::collections::HashMap;

fn main() {
    let list: Vec<i32> = vec![-7, 6, 4, 8, 1, -4, 6, -3, 8, -7, 4, 8, -1];

    let mut map: HashMap<i32, u32> = HashMap::new();

    let mut highest_occurrent: i32 = 0;

    let mut highest_occurrence_count: u32 = 0;

    for num in list {
        let occurence_count: &mut u32 = map.entry(num).or_insert(0);

        *occurence_count += 1;

        if *occurence_count > highest_occurrence_count {
            highest_occurrence_count = *occurence_count;
            highest_occurrent = num;
        }
    }


    println!("Highest occurent: {}", highest_occurrent);

    println!("Occurence count: {}", highest_occurrence_count);

    println!("Occurence map: {:?}", map);
}
