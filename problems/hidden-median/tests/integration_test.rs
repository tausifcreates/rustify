use hidden_median::interface::Holder;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(list_a: &[i32], list_b: &[i32], expected: (i32, i32)) {
        let mut holder = Holder::new(list_a, list_b);
        let result = holder.init().result();

        println!("{:?}", result);

        assert_eq!(expected.0, result.0);
        println!("1st check done");

        assert_eq!(expected.1, result.1);
        println!("2nd check done");
    }

    #[test]
    fn odd_strict_small() {
        let list_a = [1, 3, 5];
        let list_b = [8, 10, 12, 17];

        let expected = (8, 8);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_s_s2() {
        let list_a = [1, 3, 6];
        let list_b = [8, 10, 12, 17, 21, 24, 26, 29];

        let expected = (12, 12);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_s_s3() {
        let list_a = [1, 3, 6, 9];
        let list_b = [10, 12, 17, 21, 24, 26, 29];

        let expected = (12, 12);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_s_s4() {
        let list_a = [7, 10, 11, 13, 15];
        let list_b = [8, 10, 12, 17, 21, 24, 26, 29];

        let expected = (13, 13);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_mid_1() {
        let list_a = [7, 10, 11, 13, 15];
        let list_b = [8, 10, 12, 17, 21, 24, 26, 29];

        let expected = (13, 13);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_m2() {
        let list_a = [8, 9, 11, 13, 18, 22];
        let list_b = [8, 10, 12, 17, 21, 24, 26];

        let expected = (13, 13);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_m3() {
        let list_a = [11, 13, 14, 16, 18];
        let list_b = [8, 10, 12, 17, 21, 24];

        let expected = (14, 14);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_m4() {
        let list_a = [1, 3, 5, 7];
        let list_b = [2, 4, 6, 8];

        let expected = (4, 5);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_m5() {
        let list_a = [2, 2, 2, 2];
        let list_b = [3, 3, 3, 3, 3];

        let expected = (3, 3);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn even_m6() {
        let list_a = [1, 2, 2, 3, 3];
        let list_b = [2, 3, 3, 4, 5];

        let expected = (3, 3);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn even_one_elem() {
        let list_a = [1];
        let list_b = [2, 3, 3, 4, 5];

        let expected = (3, 3);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_strict_bigger() {
        let list_a = [10, 12, 14, 16];
        let list_b = [1, 3, 5, 9];

        let expected = (9, 10);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_s_b2() {
        let list_a = [10, 12, 14];
        let list_b = [1, 3, 5, 6, 8, 10];

        let expected = (8, 8);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_s_b3() {
        let list_a = [10, 12, 14, 17];
        let list_b = [1, 3, 5, 6, 8, 10];

        let expected = (8, 10);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn even_s_b4() {
        let list_a = [10, 12, 14, 17];
        let list_b = [1, 3, 5, 6, 8];

        let expected = (8, 8);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_s_b5() {
        let list_a = [10, 12, 14, 17, 21];
        let list_b = [1, 3, 5, 6, 7, 8, 9];

        let expected = (8, 9);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn even_m_b1() {
        let list_a = [10, 12, 14, 17];
        let list_b = [1, 3, 7, 11, 12, 13];

        let expected = (11, 12);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_m_b2() {
        let list_a = [8, 10, 12, 14, 17];
        let list_b = [1, 3, 7, 11, 12, 13];

        let expected = (11, 11);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_m_b3() {
        let list_a = [8, 10, 12, 13, 16];
        let list_b = [1, 3, 7, 8, 11, 13];

        let expected = (10, 10);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_m_b4() {
        let list_a = [8, 10, 12, 13, 16];
        let list_b = [1, 3, 7, 8, 11, 13];

        let expected = (10, 10);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn odd_m_b5() {
        let list_a = [8, 10, 12];
        let list_b = [1, 3, 7, 8, 11, 13];

        let expected = (8, 8);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn two_elem1() {
        let list_a = [8, 10];
        let list_b = [1, 3, 7, 8, 11, 13];

        let expected = (8, 8);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn two_elem2() {
        let list_a = [12, 17];
        let list_b = [1, 3, 7, 8, 11];

        let expected = (8, 8);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn two_elem3() {
        let list_a = [14, 18];
        let list_b = [1, 3, 7, 8, 11];

        let expected = (8, 8);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn some_negative() {
        let list_a = [-3, -1, 5, 6];
        let list_b = [-7, -2, 4, 8, 11];

        let expected = (4, 4);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn surprise() {
        let list_a = [1, 4, 6, 7, 9];
        let list_b = [5, 7, 11, 14];

        let expected = (7, 7);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn fails() {
        let list_a = [4, 5, 6, 8, 9, 11];
        let list_b = [1, 2, 3, 7, 10, 12];

        let expected = (6, 7);

        setup(&list_a, &list_b, expected);
    }

    #[test]
    fn very_big() {
        let mut list_a = [0; 50];
        let mut list_b = [0; 50];

        let mut init_a = 25;

        for i in 0..50 {
            list_a[i] = init_a;
            init_a += 1;
        }

        let mut init_b = 50;

        for i in 0..50 {
            list_b[i] = init_b;
            init_b += 1;
        }

        let expected = (62, 62);

        setup(&list_a, &list_b, expected);
    }
}
