fn main() {
    const R1: usize = 4;

    const C1: usize = 4;

    const R2: usize = 4;

    const C2: usize = 4;

    let m1: [[i32; C1]; R1] = [
        [12, 19, 6, 11],
        [-1, 5, 16, 3],
        [-5, 7, 11, -15],
        [7, -6, 12, 18],
    ];

    let m2: [[i32; C2]; R2] = [
        [6, 13, 4, -15],
        [-3, 9, 8, -12],
        [5, 11, -9, 1],
        [-4, -12, 7, 7],
    ];

    let mut m3: [[i32; R2]; C1] = [[0; R2]; C1];

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