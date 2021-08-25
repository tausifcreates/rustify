use std::cmp::min;
use std::mem::swap;

/// # Examples
///
/// ```
/// use gcd_bitwise::interface::gcd;
///
/// fn main() {
///     let num1 = 15;
///
///     let num2 = 51;
///     
///     let gcd = gcd(num1, num2);
///     
///     println!("gcd: {}", gcd); // 3   
/// }
pub fn gcd(mut num1: u64, mut num2: u64) -> u64 {
    if num1 == 0 {
        return num2;
    } else if num2 == 0 {
        return num1;
    }

    let min_twos: u32 = {
        let twos_num1: u32 = num1.trailing_zeros();
        let twos_num2: u32 = num2.trailing_zeros();

        num1 >>= twos_num1;
        num2 >>= twos_num2;

        min(twos_num1, twos_num2)
    };

    loop {
        if num1 > num2 {
            swap(&mut num1, &mut num2);
        }

        num2 -= num1;

        if num2 == 0 {
            return num1 << min_twos;
        }

        num1 >>= num1.trailing_zeros();
    }
}
