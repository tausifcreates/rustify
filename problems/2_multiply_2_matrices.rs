fn main() {
    const R1: usize = 4;

    const C1: usize = 2;

    const R2: usize = 2;

    const C2: usize = 3;

    let m1: [[i32; C1]; R1] = [[12, 19], [-1, 5], [-5, 7], [7, -6]];

    let m2: [[i32; C2]; R2] = [[6, 13, 4], [-3, 9, 8]];

    let mut m3: [[i32; C2]; R1] = [[0; C2]; R1];

    for curr_row_m1 in 0..R1 {
        for curr_col_m2 in 0..C2 {
            let mut sum = 0;
            let mut curr_col_m1 = 0;

            for curr_row_m2 in 0..R2 {
                sum += m1[curr_row_m1][curr_col_m1] * m2[curr_row_m2][curr_col_m2];
                curr_col_m1 += 1;
            }

            m3[curr_row_m1][curr_col_m2] = sum;
        }
    }

    println!("{:?}", m3);
}
