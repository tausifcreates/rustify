use rotation::interface as rotator;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_anticlock(list: &mut [i32], offset: usize, expected: &[i32]) {
        let rotated = rotator::rotate_anticlock(list, offset);
        let mut i = 0;
        for item in expected.iter() {
            assert_eq!(*item, rotated[i]);
            i += 1;
        }
    }

    fn setup_clock(list: &mut [i32], offset: usize, expected: &[i32]) {
        let rotated = rotator::rotate_clock(list, offset);
        let mut i = 0;
        for item in expected.iter() {
            assert_eq!(*item, rotated[i]);
            i += 1;
        }
    }

    #[test]
    fn cont1() {
        let mut list = [1, 2, 3, 4, 5, 6];
        let offset = 2;
        let expected = [3, 4, 5, 6, 1, 2];
        setup_anticlock(&mut list, offset, &expected);
    }

    #[test]
    fn cont2() {
        let mut list = [1, 2, 3, 4, 5, 6];
        let offset = 5;
        let expected = [2, 3, 4, 5, 6, 1];
        setup_anticlock(&mut list, offset, &expected);
    }

    #[test]
    fn cont3() {
        let mut list = [1, 2, 3, 4, 5, 6];
        let offset = 12;
        let expected = [1, 2, 3, 4, 5, 6];
        setup_anticlock(&mut list, offset, &expected);
    }

    #[test]
    fn cont4() {
        let mut list = [1, 2, 3, 4, 5, 6];
        let offset = 2;
        let expected = [5, 6, 1, 2, 3, 4];
        setup_clock(&mut list, offset, &expected);
    }

    #[test]
    fn cont5() {
        let mut list = [1, 2, 3, 4, 5, 6];
        let offset = 5;
        let expected = [6, 1, 2, 3, 4, 5];
        setup_clock(&mut list, offset, &expected);
    }

    #[test]
    fn cont6() {
        let mut list = [1, 2, 3, 4, 5, 6];
        let offset = 13;
        let expected = [6, 1, 2, 3, 4, 5];
        setup_clock(&mut list, offset, &expected);
    }

    #[test]
    fn rand1() {
        let mut list = [4, -2, 6, -1, 3, 3];
        let offset = 2;
        let expected = [6, -1, 3, 3, 4, -2];
        setup_anticlock(&mut list, offset, &expected);
    }

    #[test]
    fn rand2() {
        let mut list = [4, -2, 6, -1, 3, 3];
        let offset = 2;
        let expected = [3, 3, 4, -2, 6, -1];
        setup_clock(&mut list, offset, &expected);
    }
}
