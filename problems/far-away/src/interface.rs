use std::cmp::Ordering;

pub fn find_distance(list: &[u32]) -> Option<usize> {
	let mut smaller_idx_list: Vec<usize> = Vec::with_capacity(list.len());

	let mut smallest_idx_now = 0;

	smaller_idx_list.push(smallest_idx_now);

	// Collects the next smaller elements
	for idx in 1..list.len() {
		if list[idx] < list[smallest_idx_now] {
			smallest_idx_now = idx;
			smaller_idx_list.push(smallest_idx_now);
		}
	}

	// Let rightmost idx be the idx of biggest number for now
	let mut idx_of_biggest = list.len() - 1;

	// Set the march point on idx of biggest number
	let mut march_idx = idx_of_biggest;

	let mut max_distance_now: Option<usize> = None;

	for small_idx in smaller_idx_list.iter().rev() {
		// Stop when search for bigger idx marches past the small idx
		while march_idx > *small_idx {
			// Compare the value at this idx with current biggest number
			match list[march_idx].cmp(&list[idx_of_biggest]) {
				Ordering::Equal => (),
				Ordering::Greater => idx_of_biggest = march_idx,
				Ordering::Less => {
					// March backwards
					march_idx -= 1;
					continue;
				}
			}

			if list[idx_of_biggest] > list[*small_idx] {
				let distance_now = idx_of_biggest - *small_idx;

				match max_distance_now {
					Some(val) => {
						// if current distance > prev max distance, assigns current distance to max distance.
						if distance_now > val {
							max_distance_now = Some(distance_now);
						}
					}
					// if max distance is `None`, assigns current distance.
					None => max_distance_now = Some(distance_now),
				}

				break;
			} else {
				march_idx -= 1;
			}
		}
	}

	max_distance_now
}
