use gcd_bitwise::interface as gcd_builder;

/// # Examples
///
/// ```
/// use rotation::interface as rotator;
///
/// fn main() {
///     let mut list = [1, 2, 3, 4, 5];
///
///     let spin = 2;
///
///     // rotate clockwise
///     let rotated = rotator::rotate_clock(&mut list, spin);
///
///     println!("{:?}", rotated); // [4, 5, 1, 2, 3];
/// 
///     // rotate anti clockwise
///     let rotated = rotator::rotate_anticlock(&mut list, spin);
///
///     println!("{:?}", rotated); // [3, 4, 5, 1, 2];
/// }
/// ```
pub fn rotate_anticlock(list: &mut [i32], mut spin: usize) -> &[i32] {
    let len = list.len();
    spin %= len;
    let gcd = gcd_builder::gcd(len as u64, spin as u64) as usize;
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

pub fn rotate_clock(list: &mut [i32], mut spin: usize) -> &[i32] {
    let len = list.len();
    spin %= len;
    let gcd = gcd_builder::gcd(len as u64, spin as u64) as usize;
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
