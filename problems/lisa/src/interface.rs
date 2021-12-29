use std::collections::VecDeque;

pub fn find_lis<T>(slice: &[T]) -> VecDeque<T>
where
	T: PartialOrd + Copy,
{
	// `head_idxs` -> increasing subsequence head idxs
	let mut head_idxs: Vec<usize> = vec![0];

	let mut symlinks: Vec<Option<usize>> = vec![None];

	// Initialized with first idx of `symlinks`, it will only
	// update if a new item is appended to `head_idxs`.
	let mut super_head = 0;

	let mut lis: VecDeque<T> = VecDeque::new();

	for (idx, item) in slice.iter().skip(1).enumerate() {
		let res = lower_bound(slice, &head_idxs, *item);

		match res {
			Some(replace_idx) => {
				head_idxs[replace_idx] = idx;

				if replace_idx == 0 {
					symlinks.push(None)
				} else {
					let prev_head_idx = head_idxs[replace_idx - 1];
					symlinks.push(Some(prev_head_idx));
				}

				// If the new subseq is equal len of prev longest
				// subseq, update `super_head` to head of new subseq
				if replace_idx == head_idxs.len() - 1 {
					super_head = symlinks.len() - 1;
				}
			}

			None => {
				let this_head_idx = head_idxs[head_idxs.len() - 1];
				symlinks.push(Some(this_head_idx));
				super_head = symlinks.len() - 1;
				head_idxs.push(idx);
			}
		}
	}

	lis.push_front(slice[super_head]);

	let mut chain = super_head;

	while let Some(val) = symlinks[chain] {
		lis.push_front(slice[val]);
		chain = val;
	}

	lis
}

// returns idx of a value not less than `given`. If such value not
// exists, returns `None`.
fn lower_bound<T>(slice: &[T], head_idxs: &[usize], given: T) -> Option<usize>
where
	T: PartialOrd,
{
	let slice_len = head_idxs.len();
	let mut left_idx: usize = 0;
	let mut right_idx: usize = slice_len - 1;

	// Loop until 2 items left
	let turns: usize = (slice_len as f64).log2().ceil() as usize;

	for _ in 1..=turns {
		let mid_idx = (left_idx + right_idx) / 2;

		if slice[head_idxs[mid_idx]] >= given {
			right_idx = mid_idx;
		} else {
			left_idx = mid_idx;
		}
	}

	if slice[head_idxs[left_idx]] >= given {
		Some(left_idx)
	} else if slice[head_idxs[right_idx]] >= given {
		Some(right_idx)
	} else {
		None
	}
}
