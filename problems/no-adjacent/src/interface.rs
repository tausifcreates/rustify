pub fn calculate(list: &[i32]) -> i32 {
    let mut max_excl = list[0];
    let mut max_incl = list[1];
    let mut next_max_incl: Option<i32>;

    let len: usize = list.len();

    for i in 2..len {
        next_max_incl = Some(list[i] + max_excl);

        if max_incl > max_excl {
            max_excl = max_incl;
        }

        max_incl = next_max_incl.unwrap();
    }

    println!("{}, {}", max_incl, max_excl);

    if max_incl > max_excl {
        return max_incl;
    }

    return max_excl;
}
