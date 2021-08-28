use std::cmp::min;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Debug;
use std::mem::swap;
use std::ops::{ShlAssign, ShrAssign, SubAssign};
/// # Examples
///
/// ```
/// use gcd_bitwise::interface::gcd;
///
/// fn main() {
///     let num1: u8 = 15;
///
///     let num2: u8 = 51;
///     
///     let gcd = gcd(num1, num2);
///     
///     println!("gcd: {}", gcd); // 3   
/// }
pub fn gcd<T>(mut num1: T, mut num2: T) -> T
where
    T: Copy
        + PartialEq
        + PartialOrd
        + ShrAssign
        + ShlAssign
        + SubAssign
        + TryFrom<u32>
        + TryInto<u32>,
    <T as TryFrom<u32>>::Error: Debug,
    <T as TryInto<u32>>::Error: Debug,
{
    if num1.try_into().unwrap() == 0 {
        return num2;
    } else if num2.try_into().unwrap() == 0 {
        return num1;
    }

    let min_twos: u32 = {
        let twos_num1: u32 = num1.try_into().unwrap().trailing_zeros();
        let twos_num2: u32 = num2.try_into().unwrap().trailing_zeros();

        num1 >>= T::try_from(twos_num1).unwrap();
        num2 >>= T::try_from(twos_num2).unwrap();

        min(twos_num1, twos_num2)
    };

    loop {
        if num1 > num2 {
            swap(&mut num1, &mut num2);
        }

        num2 -= num1;

        if num2.try_into().unwrap() == 0 {
            num1 <<= T::try_from(min_twos).unwrap();
            return num1;
        }

        num1 >>= T::try_from(num1.try_into().unwrap().trailing_zeros()).unwrap();
    }
}
