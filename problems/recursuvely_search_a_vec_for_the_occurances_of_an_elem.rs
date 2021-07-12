fn search<'numbers: 'indexes, 'indexes>(
    mut count: usize,
    numbers: &'numbers Vec<i32>,
    elem: i32,
    len: usize,
    indexes: &'indexes mut Vec<usize>,
) -> &'indexes mut Vec<usize> {
    if numbers[count] == elem {
        indexes.push(count);
    }

    count += 1;

    if count == len {
        return indexes;
    }

    search(count, numbers, elem, len, indexes)
}

fn main() {
    let numbers = vec![4, 9, 2, 5, 3, 8, 1, 5];

    let len: usize = numbers.len();

    let elem: i32 = 5;

    let mut indexes: Vec<usize> = vec![];

    let indexes: &mut Vec<usize> = search(0, &numbers, elem, len, &mut indexes);

    if indexes.is_empty() {
        println!("The element {} is not present in the array.", elem);
    } else {
        print!("The element {} is present in position ", elem);

        for i in indexes.iter() {
            print!("{} ", i);
        }
    }
}
