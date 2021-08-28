use gcd_bitwise::interface::*;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Debug;
use std::ops::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup<T>(num1: T, num2: T, expected: T)
    where
        T: PartialEq
            + Debug
            + Copy
            + ShlAssign
            + PartialOrd
            + SubAssign
            + TryFrom<u32>
            + ShrAssign
            + TryInto<u32>,
        <T as TryFrom<u32>>::Error: Debug,
        <T as TryInto<u32>>::Error: Debug,
    {
        let result = gcd(num1, num2);
        assert_eq!(result, expected);
    }

    #[test]
    fn gcd1() {
        let num1: u8 = 15;
        let num2: u8 = 51;
        let expected = 3;
        setup(num1, num2, expected);
    }

    #[test]
    fn gcd2() {
        let num1: u16 = 65535;
        let num2: u16 = 125;
        let expected = 5;
        setup(num1, num2, expected);
    }

    #[test]
    fn gcd3() {
        let num1: u32 = 1456386644;
        let num2: u32 = 78907652;
        let expected = 4;
        setup(num1, num2, expected);
    }

    #[test]
    fn gcd4() {
        let num1: usize = 1235648;
        let num2: usize = 346848;
        let expected = 32;
        setup(num1, num2, expected);
    }

    #[test]
    fn gcd5() {
        let num1: u64 = 123;
        let num2: u64 = 3;
        let expected = 3;
        setup(num1, num2, expected);
    }
}
