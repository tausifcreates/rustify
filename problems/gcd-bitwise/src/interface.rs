use std::cmp::min;
use std::mem::swap;

pub struct GcdBuilder {
    num1: u64,
    num2: u64,
}

impl GcdBuilder {
    pub fn new(num1: u64, num2: u64) -> Self {
        Self { num1, num2 }
    }

    /// # Examples
    ///
    /// ```
    /// use gcd_bitwise::interface::GcdBuilder;
    ///
    /// fn main() {
    ///     let num1 = 15;
    ///
    ///     let num2 = 51;
    ///     
    ///     let gcd = GcdBuilder::new(num1, num2);
    ///     
    ///     let gcd = gcd.build();
    ///     
    ///     println!("gcd: {}", gcd); // 3   
    /// }
    pub fn build(&self) -> u64 {
        let mut num1: u64 = self.num1;
        let mut num2: u64 = self.num2;

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
}
