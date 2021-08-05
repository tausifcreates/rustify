use std::fmt::Debug;
use two_sorted::interface::Elements;

#[cfg(test)]
mod tests {

    use super::*;

    fn setup<T>(big_list: &mut [Option<T>], small_list: &[Option<T>], expected: &[Option<T>]) -> ()
    where
        T: Copy + PartialOrd + Debug,
    {
        let mut elements: Elements<T> = Elements::new(big_list, small_list);

        let result: &[Option<T>] = elements.roll().compare().result();

        let mut i: usize = 0;

        for r in result {
            assert_eq!(expected[i], *r);
            i += 1;
        }
    }

    #[test]
    fn mixup() {
        let mut big_list = [Some(4), Some(6), Some(7), Some(11), None, None, None];

        let small_list = [Some(3), Some(6), Some(9)];

        let expected = [
            Some(3),
            Some(4),
            Some(6),
            Some(6),
            Some(7),
            Some(9),
            Some(11),
        ];

        setup(&mut big_list, &small_list, &expected);
    }

    #[test]
    fn big_ends() {
        let mut big_list = [Some(2), Some(3), Some(4), None, None, None];

        let small_list = [Some(5), Some(6), Some(11)];

        let expected = [Some(2), Some(3), Some(4), Some(5), Some(6), Some(11)];

        setup(&mut big_list, &small_list, &expected);
    }

    #[test]
    fn small_ends() {
        let mut big_list = [Some(8), Some(9), Some(14), Some(15), None, None];

        let small_list = [Some(2), Some(3)];

        let expected = [Some(2), Some(3), Some(8), Some(9), Some(14), Some(15)];

        setup(&mut big_list, &small_list, &expected);
    }

    #[test]
    fn chars() {
        let mut big_list = [Some('b'), Some('e'), Some('g'), Some('h'), None, None, None];

        let small_list = [Some('a'), Some('d'), Some('f')];

        let expected = [
            Some('a'),
            Some('b'),
            Some('d'),
            Some('e'),
            Some('f'),
            Some('g'),
            Some('h'),
        ];

        setup(&mut big_list, &small_list, &expected);
    }
}
