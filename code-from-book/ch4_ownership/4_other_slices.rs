fn main() {
    let a = [1, 2, 4, 5, 7, 8];

    let slice = &a[1..4];

    println!("{:p}", &a);

    println!("{:p}", slice);

    assert_eq!(slice, &[2, 4, 5]);
}
