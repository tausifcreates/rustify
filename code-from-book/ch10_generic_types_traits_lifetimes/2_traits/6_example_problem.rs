// This function has one parameter named `list`, which is a
// slice of values of type `T`.

// Because `largest` function is generic, it's possible for the `list`
// parameter to have types in it that don’t implement the `Copy` trait.
// Consequently, we wouldn’t be able to move the value out of `list[0]`
// and into the `largest` variable, resulting in error.

// To call this code with only those types that implement the `Copy` trait,
// we can add Copy to the trait bounds of `T`.

// In the body of `largest` we wanted to compare two values of type `T`
// using the greater than (`>`) operator. Because that operator is defined
// as a default method on the standard library trait `std::cmp::PartialOrd`,
// we need to specify `PartialOrd` in the trait bounds for `T` so the
// `largest` function can work on slices of any type that we can compare.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list: Vec<i32> = vec![4, 3, 9, 5, 7, 6];

    let result = largest(&number_list);

    println!("Largest number in the list is {}", result);
}
