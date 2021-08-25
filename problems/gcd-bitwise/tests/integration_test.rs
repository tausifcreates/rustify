use gcd_bitwise::interface::GcdBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(num1: u64, num2: u64, expected: u64) {
        let gcd_builder = GcdBuilder::new(num1, num2);
        let result = gcd_builder.build();
        assert_eq!(result, expected);
    }

    #[test]
    fn gcd1() {
        let num1 = 15;
        let num2 = 51;
        let expected = 3;
        setup(num1, num2, expected);
    }

    #[test]
    fn gcd2() {
        let num1 = 75;
        let num2 = 125;
        let expected = 25;
        setup(num1, num2, expected);
    }

    #[test]
    fn gcd3() {
        let num1 = 1235648;
        let num2 = 346848;
        let expected = 32;
        setup(num1, num2, expected);
    }
}
