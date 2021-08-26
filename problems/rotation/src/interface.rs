use gcd_bitwise::interface as gcd_builder;

pub fn rotate_anticlock(list: &mut [i32], mut offset: usize) -> &[i32] {
    let len = list.len();
    offset %= len;
    let gcd = gcd_builder::gcd(len as u64, offset as u64) as usize;
    let turns = len / gcd - 1;

    let mut start: usize = 0;

    for _ in 1..turns + 1 {
        for i in start..start + gcd {
            list[i] += list[gcd + i];
            list[gcd + i] = list[i] - list[gcd + i];
            list[i] -= list[gcd + i];
        }

        start += gcd;
    }

    list
}

pub fn rotate_clock(list: &mut [i32], mut offset: usize) -> &[i32] {
    let len = list.len();
    offset %= len;
    let gcd = gcd_builder::gcd(len as u64, offset as u64) as usize;
    let turns = len / gcd - 1;

    let mut end: usize = len - 1;

    for _ in 1..turns + 1 {
        for i in ((end - gcd + 1)..end + 1).rev() {
            list[i] += list[i - gcd];
            list[i - gcd] = list[i] - list[i - gcd];
            list[i] -= list[i - gcd];
        }

        end -= gcd;
    }

    list
}
