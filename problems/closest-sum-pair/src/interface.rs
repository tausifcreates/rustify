use std::cmp::Ordering;
use std::ops::{Add, Sub};

// This method runs a loop and tries to minimize the distance
// for every comparison between current distance and sum of
// two numbers in list.
/// # Examples
/// ```
/// use closest_sum_pair::interface::find_pair;
///
/// let mut list = [-2, -4, -7, -2, -5, -13, -7];
///
/// list.sort();
///
/// let desired_sum = -1;
///
/// let pair = find_pair(&mut list, desired_sum);
/// ```
pub fn find_pair<U>(list: &[U], desired_sum: U) -> (U, U)
where
	U: Copy + Add<Output = U> + Sub<Output = U> + Into<f64>,
{
	let desired_sum: f64 = desired_sum.into();

	let mut left_bound: usize = 0;

	let mut right_bound: usize = list.len() - 1;

	let mut pair: Option<(U, U)> = Some((list[left_bound], list[right_bound]));

	let mut init_distance = (list[left_bound] + list[right_bound]).into();

	for _ in 0..(list.len() - 2) + 1 {
		let this_pair_sum: U = list[left_bound] + list[right_bound];

		let this_distance: f64 = desired_sum - this_pair_sum.into();

		if this_distance.abs() < init_distance.abs() {
			init_distance = this_distance;
			pair = Some((list[left_bound], list[right_bound]));
		}

		match this_distance.partial_cmp(&0.0).unwrap() {
			Ordering::Greater => left_bound += 1,
			Ordering::Less => right_bound -= 1,
			Ordering::Equal => break,
		}
	}

	pair.unwrap()
}
