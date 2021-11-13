/// ```
/// use bsutils::interface::first_last_idxs;
/// 
/// let list = [1, 1, 4, 6, 7, 7, 7, 7, 9, 9, 11];
/// let given = 7; // the item we look for
/// let both_idxs = first_last_idxs(&list, given);
///```
pub fn first_last_idxs<T>(slice: &[T], given: T) -> (Option<usize>, Option<usize>)
where
	T: Copy + PartialOrd,
{
	let first_occur_idx: Option<usize> = find_first_idx(slice, given);

	let last_occur_idx: Option<usize> =
		first_occur_idx.map(|idx| find_last_idx(&slice[idx..], given).unwrap() + idx);

	(first_occur_idx, last_occur_idx)
}

#[allow(clippy::collapsible_else_if)]
pub fn find_first_idx<F>(slice: &[F], given: F) -> Option<usize>
where
	F: PartialOrd,
{
	let mut left_idx: usize = 0;
	let mut right_idx: usize = slice.len() - 1;
	let mut mid_idx: usize = (left_idx + right_idx) / 2;
	// The loop will at most run this amount so after final turn,
	// -> the relation becomes `right_idx - left_idx == 1`
	let turns: usize = (slice.len() as f64).log2().floor() as usize;

	for _ in 1..=turns {
		if slice[mid_idx] >= given {
			right_idx = mid_idx;
			mid_idx = (left_idx + right_idx) / 2;
		} else {
			// This if block will be checked in the final turn.
			if slice[mid_idx + 1] == given {
				return Some(mid_idx + 1);
			}
			// In the final turn, set `right_idx - left_idx == 1`
			else {
				left_idx = mid_idx;
				mid_idx = (left_idx + right_idx) / 2;
			}
		}
	}

	if left_idx == right_idx {
		if slice[left_idx] == given {
			return Some(left_idx);
		}
	} else if right_idx == left_idx + 1 {
		if slice[left_idx] == given {
			return Some(left_idx);
		} else if slice[right_idx] == given {
			return Some(right_idx);
		}
	}

	None
}

pub fn find_last_idx<L>(slice: &[L], given: L) -> Option<usize>
where
	L: PartialOrd,
{
	let slice_len: usize = slice.len();

	let mut left_idx: usize = 0;
	let mut right_idx: usize = slice_len - 1;
	let mut mid_idx: usize = (left_idx + right_idx) / 2;

	let turns: usize = (slice_len as f64).log2().floor() as usize;

	for _ in 1..=turns {
		if slice[mid_idx] > given {
			if slice[mid_idx - 1] == given {
				return Some(mid_idx - 1);
			} else {
				right_idx = mid_idx;
				mid_idx = (left_idx + right_idx) / 2;
			}
		} else {
			left_idx = mid_idx;
			mid_idx = (left_idx + right_idx) / 2;
		}
	}

	if left_idx == right_idx {
		return Some(left_idx);
	} else if right_idx == left_idx + 1 {
		if slice[right_idx] == given {
			return Some(right_idx);
		} else {
			return Some(left_idx);
		}
	}

	None
}
