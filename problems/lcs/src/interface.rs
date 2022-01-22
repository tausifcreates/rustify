#[derive(Clone, Copy, Debug)]
struct Link {
	prev_head: Option<usize>,
	prev_head_idx: Option<usize>,
}

pub fn compute_lcs(slice_a: &[char], slice_b: &[char]) {
	let mut head_idxs: Vec<usize> = Vec::new();

	// Create a vector that simulates a 2D matrix
	let mut link_chains: Vec<Option<Link>> = vec![None; slice_a.len().pow(2)];

	// Stores the last occupied column of each row of `link_chains`.
	let mut row_data: Vec<Option<usize>> = vec![None; slice_a.len()];

	let first_b_item = slice_b.first().unwrap();

	for (a_idx, a_item) in slice_a.iter().enumerate() {
		if a_item == first_b_item {
			head_idxs.push(a_idx);
			let link = Link {
				prev_head: None,
				prev_head_idx: None,
			};
			row_data[a_idx] = Some(0);
			link_chains[a_idx * slice_a.len()] = Some(link);
			break;
		}
	}

	// Make a full iteration of slice_a for every item of slice_b to
	// find suitable match
	for b_item in slice_b.iter().skip(1) {
		// todo: end_head might be None if no head is found in
		// first iteration
		let end_head = *head_idxs.last().unwrap();

		let mut last_match_idx: Option<usize> = None;

		// 1st part: iterate from where the last head was found to the end of
		// `slice_a` in reverse order. In this part, we will travel until we
		// find the last match
		for (a_idx, a_item) in slice_a[end_head + 1..=slice_a.len() - 1]
			.iter()
			.enumerate()
			.rev()
		{
			let relative_a_idx = a_idx + end_head + 1;
			if a_item == b_item {
				last_match_idx = Some(relative_a_idx)
			}
		}

		if let Some(match_idx) = last_match_idx {
			head_idxs.push(match_idx);
			
			let link = Link {
				prev_head: Some(*head_idxs.last().unwrap()),
				prev_head_idx: Some(head_idxs.len() - 1),
			};

			match row_data[match_idx] {
				Some(col) => {
					let next_col = col + 1;
					link_chains[match_idx * slice_a.len() + next_col] = Some(link);
					row_data[match_idx] = Some(next_col);
				}

				None => {
					link_chains[match_idx * slice_a.len()] = Some(link);
					row_data[match_idx] = Some(0);
				}
			};
		}

		// 2nd part: iterate from the beginning of the slice to `end_head`
		for (a_idx, a_item) in slice_a[0..=end_head].iter().enumerate().rev() {
			if a_item == b_item {
				let new_head_idx = lower_bound(&head_idxs, a_idx);

				if let Some(replace_idx) = new_head_idx {
					head_idxs[replace_idx] = a_idx;
					let link: Link;
					if replace_idx == 0 {
						link = Link {
							prev_head: None,
							prev_head_idx: None,
						}
					} else {
						link = Link {
							prev_head: Some(head_idxs[replace_idx - 1]),
							prev_head_idx: Some(replace_idx - 1),
						};
					}

					match row_data[a_idx] {
						Some(col) => {
							let next_col = col + 1;
							link_chains[a_idx * slice_a.len() + next_col] = Some(link);
							row_data[a_idx] = Some(next_col);
						}

						None => {
							link_chains[a_idx * slice_a.len()] = Some(link);
							row_data[a_idx] = Some(0);
						}
					}
				}
			}
		}
	}

	for (idx, item) in link_chains.iter().enumerate() {
		if idx % slice_a.len() == 0 {
			println!("\n");
			println!("{}th Row:", idx / slice_a.len());
			println!("\n")
		}
		println!("{:?} ", *item);
	}
}

fn lower_bound(head_idxs: &[usize], given: usize) -> Option<usize> {
	if head_idxs.is_empty() {
		return None;
	}

	let mut left_idx: usize = 0;
	let mut right_idx: usize = head_idxs.len() - 1;

	// Loop until 2 items left
	let turns: usize = (head_idxs.len() as f64).log2().ceil() as usize;

	for _ in 1..=turns {
		let mid_idx = (left_idx + right_idx) / 2;

		if head_idxs[mid_idx] >= given {
			right_idx = mid_idx;
		} else {
			left_idx = mid_idx;
		}
	}

	if head_idxs[left_idx] > given {
		Some(left_idx)
	} else if head_idxs[right_idx] > given {
		Some(right_idx)
	} else {
		None
	}
}
